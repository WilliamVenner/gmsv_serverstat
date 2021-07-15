use std::{cell::RefCell, sync::atomic::AtomicUsize};

use crossbeam::channel::{Receiver, Sender};

use crate::{export::{SysInfoRequestData, SysInfoResponseData}, lua::{self, LUA_REGISTRYINDEX, LuaReference}};

static RESPONSE_QUEUE_SIZE: AtomicUsize = AtomicUsize::new(0);

lazy_static! {
	pub(super) static ref RESPONSE_CHANNEL: (Sender<SysInfoResponse>, Receiver<SysInfoResponse>) = crossbeam::channel::unbounded::<SysInfoResponse>();
}
thread_local! {
	pub static ASYNC_CONTROLLER: RefCell<AsyncController> = RefCell::new(AsyncController::init());
}

pub struct SysInfoRequest {
	pub callback_ref: LuaReference,
	pub data: SysInfoRequestData,
}
pub struct SysInfoResponse {
	callback_ref: LuaReference,
	data: Box<dyn SysInfoResponseData>,
}

pub struct AsyncController {
	tx: Sender<SysInfoRequest>,
	hooked: bool
}
impl AsyncController {
	fn init() -> AsyncController {
		let (tx, rx) = crossbeam::channel::unbounded::<SysInfoRequest>();

		std::thread::spawn(move || {
			while let Ok(request) = rx.recv() {
				RESPONSE_CHANNEL.0.send(SysInfoResponse {
					callback_ref: request.callback_ref,
					data: request.data.dispatch(),
				}).ok();
			}
		});

		AsyncController {
			tx,
			hooked: false
		}
	}

	pub fn request(&mut self, lua: lua::State, request: SysInfoRequest) {
		if let Ok(_) = self.tx.send(request) {
			RESPONSE_QUEUE_SIZE.fetch_add(1, std::sync::atomic::Ordering::Release);
			self.async_await(lua);
		}
	}

	#[inline]
	fn async_await(&mut self, lua: lua::State) {
		if !self.hooked {
			unsafe {
				lua.get_global(lua_string!("hook"));
				lua.get_field(-1, lua_string!("Add"));
				lua.push_string("Tick");
				lua.push_string("gmsv_serverstat");
				lua.push_function(AsyncController::async_poll);
				lua.call(3, 0);
				lua.pop();
			}
			self.hooked = true;
		}
	}

	#[inline]
	fn async_finally(&mut self, lua: lua::State) {
		if self.hooked {
			unsafe {
				lua.get_global(lua_string!("hook"));
				lua.get_field(-1, lua_string!("Remove"));
				lua.push_string("Tick");
				lua.push_string("gmsv_serverstat");
				lua.call(2, 0);
				lua.pop();
			}
			self.hooked = false;
		}
	}

	pub unsafe extern "C-unwind" fn async_poll(lua: lua::State) -> i32 {
		ASYNC_CONTROLLER.with(|controller_cell| {
			let mut controller = controller_cell.borrow_mut();
			let mut responses = 0;
			while let Ok(response) = RESPONSE_CHANNEL.1.try_recv() {
				lua.raw_geti(LUA_REGISTRYINDEX, response.callback_ref);
				lua.dereference(response.callback_ref);
				let args = (&*response.data).push_lua(lua);
				lua.pcall(args, 0, 0);

				responses += 1;
			}
			if RESPONSE_QUEUE_SIZE.fetch_sub(responses, std::sync::atomic::Ordering::SeqCst) == 0 {
				controller.async_finally(lua);
			}
		});
		0
	}
}
