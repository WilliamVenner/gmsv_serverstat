#![feature(c_unwind)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod lua;

mod r#async;
mod sysinfo;
mod export;
mod realtime;

#[no_mangle]
pub unsafe extern "C-unwind" fn gmod13_open(lua: lua::State) -> i32 {
	lua.new_table(); // serverstat
	lua.new_table(); // async
	lua.new_table(); // realtime

	macro_rules! push_api_func {
		($func:ident, $name:literal) => {
			lua.push_function(export::sync::$func);
			lua.set_field(-4, lua_string!($name));

			lua.push_function(export::r#async::$func);
			lua.set_field(-3, lua_string!($name));
		}
	}

	macro_rules! push_realtime_api_func {
		($func:ident, $name:literal) => {
			lua.push_function(export::sync::$func);
			lua.set_field(-4, lua_string!($name));

			lua.push_function(export::r#async::$func);
			lua.set_field(-3, lua_string!($name));

			lua.push_function(realtime::$func);
			lua.set_field(-2, lua_string!($name));
		}
	}

	macro_rules! push_realtime_func {
		($func:ident, $name:literal) => {
			lua.push_function(realtime::$func);
			lua.set_field(-2, lua_string!($name));
		}
	}

	push_realtime_api_func!(process_cpu_usage, "ProcessCPUUsage");
	push_realtime_api_func!(process_memory_usage, "ProcessMemoryUsage");
	push_realtime_api_func!(system_cpu_usage, "SystemCPUUsage");
	push_realtime_api_func!(system_memory_usage, "SystemMemoryUsage");
	push_realtime_api_func!(system_available_memory, "SystemAvailableMemory");

	push_api_func!(all, "All");
	push_api_func!(all_system, "AllSystem");
	push_api_func!(all_process, "AllProcess");
	push_api_func!(system_total_memory, "SystemTotalMemory");
	push_api_func!(logical_cpus, "PhysicalCPUs");
	push_api_func!(physical_cpus, "LogicalCPUs");

	push_realtime_func!(start, "Start");
	push_realtime_func!(stop, "Stop");
	push_realtime_func!(all, "All");
	push_realtime_func!(all, "AllCopy");
	push_realtime_func!(all_system, "AllSystem");
	push_realtime_func!(all_system, "AllSystemCopy");
	push_realtime_func!(all_process, "AllProcess");
	push_realtime_func!(all_process, "AllProcessCopy");
	push_realtime_func!(set_interval, "SetInterval");

	lua.set_field(-3, lua_string!("realtime"));
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

	lua.get_global(lua_string!("timer"));
	lua.get_field(-1, lua_string!("Remove"));
	lua.push_string("gmsv_serverstat_realtime");
	lua.call(1, 0);
	lua.pop();

	0
}
