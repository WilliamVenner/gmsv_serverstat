use crate::{export::SysInfoResponseData, sysinfo::RealtimeData, lua};

impl SysInfoResponseData for RealtimeData {
	unsafe fn push_lua(&self, lua: lua::State) -> i32 {
		lua.create_table(0, 2);

		lua.create_table(0, 2); // Process
		lua.create_table(0, 3); // System
		self.push_lua_struct_fields(lua);

		lua.set_field(-3, lua_string!("System"));
		lua.set_field(-2, lua_string!("Process"));
		1
	}

	unsafe fn push_lua_struct_fields(&self, lua: lua::State) {
		self.system_cpu_usage.push_lua(lua);
		lua.set_field(-2, lua_string!("CPUUsage"));

		self.system_memory_usage.push_lua(lua);
		lua.set_field(-2, lua_string!("MemoryUsage"));

		self.system_available_memory.push_lua(lua);
		lua.set_field(-2, lua_string!("AvailableMemory"));

		self.process_cpu_usage.push_lua(lua);
		lua.set_field(-3, lua_string!("CPUUsage"));

		self.process_memory_usage.push_lua(lua);
		lua.set_field(-3, lua_string!("MemoryUsage"));
	}
}

const DAEMON_SCRIPT: &'static str = concat!(include_str!("realtime.lua"), '\0');

pub unsafe extern "C-unwind" fn refresh_async(lua: lua::State) -> i32 {
	crate::export::r#async::realtime(lua)
}
pub unsafe extern "C-unwind" fn refresh(lua: lua::State) -> i32 {
	crate::export::sync::realtime(lua)
}
pub unsafe extern "C-unwind" fn start(lua: lua::State) -> i32 {
	lua.get_global(lua_string!("serverstat"));
	lua.get_field(-1, lua_string!("realtime"));
	lua.push_function(refresh);
	lua.set_field(-2, lua_string!("Refresh"));
	lua.push_function(refresh_async);
	lua.set_field(-2, lua_string!("RefreshAsync"));
	lua.pop();

	lua.load_string(std::ffi::CStr::from_bytes_with_nul_unchecked(DAEMON_SCRIPT.as_bytes()).as_ptr()).expect("Failed to start daemon script!");
	lua.call(0, 0);
	0
}
pub unsafe extern "C-unwind" fn stop(_lua: lua::State) -> i32 {
	0
}
pub unsafe extern "C-unwind" fn set_interval(lua: lua::State) -> i32 {
	start(lua);
	lua.get_global(lua_string!("serverstat"));
	lua.get_field(-1, lua_string!("realtime"));
	lua.get_field(-1, lua_string!("SetInterval"));
	lua.push_value(1);
	lua.call(1, 0);
	0
}

macro_rules! initializer_func {
	($func:ident) => {
		pub unsafe extern "C-unwind" fn $func(lua: lua::State) -> i32 {
			start(lua);
			crate::export::sync::$func(lua)
		}
	};
}
initializer_func!(system_cpu_usage);
initializer_func!(system_memory_usage);
initializer_func!(system_available_memory);
initializer_func!(process_cpu_usage);
initializer_func!(process_memory_usage);

pub unsafe extern "C-unwind" fn all(lua: lua::State) -> i32 {
	start(lua);
	refresh(lua)
}
pub unsafe extern "C-unwind" fn all_system(lua: lua::State) -> i32 {
	start(lua);
	refresh(lua);
	lua.get_field(-1, lua_string!("System"));
	1
}
pub unsafe extern "C-unwind" fn all_process(lua: lua::State) -> i32 {
	start(lua);
	refresh(lua);
	lua.get_field(-1, lua_string!("Process"));
	1
}