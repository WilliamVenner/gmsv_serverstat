use crate::lua;
use crate::sysinfo::*;

pub trait SysInfoResponseData: Send {
	unsafe fn push_lua(&self, lua: lua::State) -> i32;
	unsafe fn push_lua_struct_fields(&self, _lua: lua::State) {
		unreachable!();
	}
}
impl SysInfoResponseData for f64 {
	#[inline]
	unsafe fn push_lua(&self, lua: lua::State) -> i32 {
		lua.push_number(*self);
		1
	}
}
impl SysInfoResponseData for u16 {
	#[inline]
	unsafe fn push_lua(&self, lua: lua::State) -> i32 {
		lua.push_integer(*self as i32);
		1
	}
}
impl SysInfoResponseData for AllProcess {
	unsafe fn push_lua(&self, lua: lua::State) -> i32 {
		lua.create_table(0, 2);
		self.push_lua_struct_fields(lua);
		1
	}

	unsafe fn push_lua_struct_fields(&self, lua: lua::State) {
		self.cpu_usage.push_lua(lua);
		lua.set_field(-2, lua_string!("ProcessCPUUsage"));

		self.memory_usage.push_lua(lua);
		lua.set_field(-2, lua_string!("ProcessMemoryUsage"));
	}
}
impl SysInfoResponseData for AllSystem {
	unsafe fn push_lua(&self, lua: lua::State) -> i32 {
		lua.create_table(0, 6);
		self.push_lua_struct_fields(lua);
		1
	}

	unsafe fn push_lua_struct_fields(&self, lua: lua::State) {
		self.cpu_usage.push_lua(lua);
		lua.set_field(-2, lua_string!("SystemCPUUsage"));

		self.memory_usage.push_lua(lua);
		lua.set_field(-2, lua_string!("SystemMemoryUsage"));

		self.total_memory.push_lua(lua);
		lua.set_field(-2, lua_string!("SystemTotalMemory"));

		self.available_memory.push_lua(lua);
		lua.set_field(-2, lua_string!("SystemAvailableMemory"));

		self.logical_cpus.push_lua(lua);
		lua.set_field(-2, lua_string!("LogicalCPUs"));

		self.physical_cpus.push_lua(lua);
		lua.set_field(-2, lua_string!("PhysicalCPUs"));
	}
}
impl SysInfoResponseData for (AllSystem, AllProcess) {
	unsafe fn push_lua(&self, lua: lua::State) -> i32 {
		lua.create_table(0, 2 + 6);
		self.0.push_lua_struct_fields(lua);
		self.1.push_lua_struct_fields(lua);
		1
	}
}

macro_rules! sysinfo {
	( $(($sysinfo_fn:ident, $sysinfo_enum:ident);)* ) => {
		pub enum SysInfoRequestData {
			$($sysinfo_enum),*
		}
		impl SysInfoRequestData {
			pub fn dispatch(&self) -> Box<dyn SysInfoResponseData> {
				match self {
					$(SysInfoRequestData::$sysinfo_enum => Box::new(crate::sysinfo::$sysinfo_fn())),*
				}
			}
		}
		pub mod r#async {
			use super::*;
			$(
				pub unsafe extern "C-unwind" fn $sysinfo_fn(lua: lua::State) -> i32 {
					lua.check_function(1);
					lua.push_value(1);
					let callback_ref = lua.reference();
					crate::r#async::ASYNC_CONTROLLER.with(|controller| controller.borrow_mut().request(lua, crate::r#async::SysInfoRequest {
						callback_ref,
						data: SysInfoRequestData::$sysinfo_enum,
					}));
					0
				}
			)*
		}
		pub mod sync {
			use super::*;
			$(
				pub unsafe extern "C-unwind" fn $sysinfo_fn(lua: lua::State) -> i32 {
					crate::sysinfo::$sysinfo_fn().push_lua(lua)
				}
			)*
		}
	};
}

sysinfo!(
	(process_cpu_usage, ProcessCPUUsage);
	(process_memory_usage, ProcessMemoryUsage);
	(system_cpu_usage, SystemCPUUsage);
	(system_memory_usage, SystemMemoryUsage);
	(system_total_memory, SystemTotalMemory);
	(system_available_memory, SystemAvailableMemory);
	(logical_cpus, PhysicalCPUs);
	(physical_cpus, LogicalCPUs);
	(all, All);
	(all_system, AllSystem);
	(all_process, AllProcess);
);
