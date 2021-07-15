#![feature(c_unwind)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod lua;

mod sysinfo;
use crate::sysinfo::*;

#[no_mangle]
pub unsafe extern "C-unwind" fn gmod13_open(lua: lua::State) -> i32 {
	lua.new_table();

	lua.push_function(process_cpu_usage);
	lua.set_field(-2, lua_string!("ProcessCPUUsage"));

	lua.push_function(process_memory_usage);
	lua.set_field(-2, lua_string!("ProcessMemoryUsage"));

	lua.push_function(system_cpu_usage);
	lua.set_field(-2, lua_string!("SystemCPUUsage"));

	lua.push_function(system_memory_usage);
	lua.set_field(-2, lua_string!("SystemMemoryUsage"));

	lua.push_function(system_total_memory);
	lua.set_field(-2, lua_string!("SystemTotalMemory"));

	lua.push_function(system_available_memory);
	lua.set_field(-2, lua_string!("SystemAvailableMemory"));

	lua.push_function(logical_cpus);
	lua.set_field(-2, lua_string!("LogicalCPUs"));

	lua.push_function(physical_cpus);
	lua.set_field(-2, lua_string!("PhysicalCPUs"));

	lua.set_global(lua_string!("serverstat"));
	0
}

#[no_mangle]
pub unsafe extern "C-unwind" fn gmod13_close(_lua: lua::State) -> i32 {
	0
}
