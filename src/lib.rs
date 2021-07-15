#![feature(c_unwind)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod lua;

mod r#async;
mod sysinfo;
mod export;

#[no_mangle]
pub unsafe extern "C-unwind" fn gmod13_open(lua: lua::State) -> i32 {
	lua.new_table();
	lua.new_table();

	macro_rules! push_func {
		($func:ident, $name:literal) => {
			lua.push_function(export::sync::$func);
			lua.set_field(-3, lua_string!($name));

			lua.push_function(export::r#async::$func);
			lua.set_field(-2, lua_string!($name));
		}
	}

	push_func!(process_cpu_usage, "ProcessCPUUsage");
	push_func!(process_memory_usage, "ProcessMemoryUsage");
	push_func!(system_cpu_usage, "SystemCPUUsage");
	push_func!(system_memory_usage, "SystemMemoryUsage");
	push_func!(system_total_memory, "SystemTotalMemory");
	push_func!(system_available_memory, "SystemAvailableMemory");
	push_func!(logical_cpus, "PhysicalCPUs");
	push_func!(physical_cpus, "LogicalCPUs");
	push_func!(all, "All");
	push_func!(all_system, "AllSystem");
	push_func!(all_process, "AllProcess");

	lua.set_field(-2, lua_string!("async"));
	lua.set_global(lua_string!("serverstat"));
	0
}

#[no_mangle]
pub unsafe extern "C-unwind" fn gmod13_close(lua: lua::State) -> i32 {
	lua.get_global(lua_string!("hook"));
	lua.get_field(-1, lua_string!("Remove"));
	lua.push_string("Tick");
	lua.push_string("gmsv_serverstat");
	lua.call(2, 0);
	lua.pop();
	0
}
