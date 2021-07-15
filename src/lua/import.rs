use libloading::{Library, Symbol};

use super::State as LuaState;

pub type LuaInt = std::os::raw::c_int;
pub type LuaString = *const std::os::raw::c_char;
pub type LuaFunction = unsafe extern "C-unwind" fn(state: LuaState) -> i32;
pub type LuaNumber = std::os::raw::c_double;

pub const LUA_GLOBALSINDEX: LuaInt = -10002;

lazy_static::lazy_static! {
	pub(super) static ref LUA_SHARED: LuaShared = LuaShared::import();
}

pub(super) struct LuaShared {
	pub lua_setfield: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: LuaInt, k: LuaString)>,
	pub lua_createtable: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, narr: LuaInt, nrec: LuaInt)>,
	pub lua_settop: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, count: LuaInt)>,
	pub lua_pushcclosure: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, func: LuaFunction, upvalues: LuaInt)>,
	pub lua_pushinteger: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, int: LuaInt)>,
	pub lua_pushnumber: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, int: LuaNumber)>,
}
unsafe impl Sync for LuaShared {}
impl LuaShared {
	fn import() -> Self {
		unsafe {
			let library = Self::find_library();
			let library = Box::leak(Box::new(library)); // Keep this library referenced forever

			macro_rules! find_symbol {
				( $symbol:literal ) => {
					Self::find_symbol(library, concat!($symbol, "\0").as_bytes())
				};
			}

			Self {
				lua_setfield: find_symbol!("lua_setfield"),
				lua_createtable: find_symbol!("lua_createtable"),
				lua_settop: find_symbol!("lua_settop"),
				lua_pushcclosure: find_symbol!("lua_pushcclosure"),
				lua_pushnumber: find_symbol!("lua_pushnumber"),
				lua_pushinteger: find_symbol!("lua_pushinteger"),
			}
		}
	}

	unsafe fn find_symbol<T>(library: &'static Library, name: &[u8]) -> Symbol<'static, T> {
		match library.get(name) {
			Ok(symbol) => symbol,
			Err(err) => panic!("Failed to find symbol \"{}\"\n{:#?}", String::from_utf8_lossy(name), err),
		}
	}

	#[cfg(target_os = "windows")]
	unsafe fn find_library() -> Library {
		let result = Library::new("lua_shared.dll");

		#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
		let result = Library::new("lua_shared.so");

		#[cfg(all(target_os = "linux", target_pointer_width = "32"))]
		let result = Library::new("garrysmod/bin/lua_shared_srv.so");

		match result {
			Ok(library) => library,
			Err(_) => panic!("Failed to load lua_shared")
		}
	}

	#[cfg(target_os = "linux")]
	unsafe fn find_library() -> Library {
		for path in [
			"garrysmod/bin/lua_shared_srv.so",
			"lua_shared.so",
			"lua_shared_srv.so",
		].iter() {
			if let Ok(library) = Library::new(path) {
				return library;
			}
		}
		panic!("Failed to load lua_shared");
	}
}
