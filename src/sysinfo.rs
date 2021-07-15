use std::cell::RefCell;
use std::time::{Duration, Instant};

use ::sysinfo::*;

use crate::lua;

const CPU_REFRESH_INTERVAL: Duration = Duration::from_millis(200);

lazy_static! {
	static ref PID: usize = sysinfo::get_current_pid().expect("Failed to get process ID") as usize;
}

thread_local! {
	static SYSTEM: RefCell<System> = RefCell::new(System::default());
}
macro_rules! system {
	(mut $sys:ident, $code:block) => {
		SYSTEM.with(|sys| {
			let mut $sys = sys.borrow_mut();
			$code
		})
	};

	(mut $sys:ident, $code:expr) => {
		SYSTEM.with(|sys| {
			let mut $sys = sys.borrow_mut();
			$code
		})
	};

	($sys:ident, $code:block) => {
		SYSTEM.with(|sys| {
			let $sys = sys.borrow_mut();
			$code
		})
	};

	($sys:ident, $code:expr) => {
		SYSTEM.with(|sys| {
			let $sys = sys.borrow_mut();
			$code
		})
	};
}

macro_rules! process {
	($sys:ident) => {{
		$sys.refresh_process(*PID as _);
		$sys.process(*PID as _).expect("Failed to get process information")
	}};
}

lazy_static! {
	static ref LOGICAL_CPUS: i32 = system!(sys, sys.processors().len()) as i32;
}

pub unsafe extern "C-unwind" fn system_cpu_usage(lua: lua::State) -> i32 {
	fn update_cpu_usage(cpu_usage: &mut f64, sys: &mut System) {
		sys.refresh_cpu();
		*cpu_usage = sys.global_processor_info().cpu_usage() as f64;
	}

	thread_local! {
		static TOTAL_CPU_USAGE: RefCell<(f64, Instant)> = RefCell::new(({
			let mut cpu_usage: f64 = 0.;
			system!(mut sys, update_cpu_usage(&mut cpu_usage, &mut *sys));
			cpu_usage
		}, Instant::now()));
	}

	lua.push_number(TOTAL_CPU_USAGE.with(|cell| {
		let (ref mut cpu_usage, ref mut timestamp) = *cell.borrow_mut();
		if timestamp.elapsed() > CPU_REFRESH_INTERVAL {
			system!(mut sys, update_cpu_usage(cpu_usage, &mut *sys));
		}
		*cpu_usage
	}));

	1
}

pub unsafe extern "C-unwind" fn system_memory_usage(lua: lua::State) -> i32 {
	system!(mut sys, {
		sys.refresh_memory();
		lua.push_number(sys.used_memory() as f64 / 1024.);
	});
	1
}

pub unsafe extern "C-unwind" fn system_total_memory(lua: lua::State) -> i32 {
	lazy_static! {
		static ref TOTAL_MEMORY: f64 = system!(mut sys, {
			sys.refresh_memory();
			sys.total_memory() as f64 / 1024.
		}) as f64;
	}
	lua.push_number(*TOTAL_MEMORY);
	1
}

pub unsafe extern "C-unwind" fn system_available_memory(lua: lua::State) -> i32 {
	system!(mut sys, {
		sys.refresh_memory();
		lua.push_number(sys.available_memory() as f64 / 1024.);
	});
	1
}

pub unsafe extern "C-unwind" fn process_cpu_usage(lua: lua::State) -> i32 {
	fn update_cpu_usage(cpu_usage: &mut f64, process: &Process) {
		*cpu_usage = (process.cpu_usage() / (*LOGICAL_CPUS as f32)) as f64;
	}

	thread_local! {
		static PROCESS_CPU_USAGE: RefCell<(f64, Instant)> = RefCell::new(({
			lazy_static::initialize(&LOGICAL_CPUS);

			let mut cpu_usage: f64 = 0.;
			system!(mut sys, {
				let process = process!(sys);
				update_cpu_usage(&mut cpu_usage, process);
			});
			cpu_usage

		}, Instant::now()));
	}

	lua.push_number(PROCESS_CPU_USAGE.with(|cell| {
		let (ref mut cpu_usage, ref mut timestamp) = *cell.borrow_mut();
		if timestamp.elapsed() > CPU_REFRESH_INTERVAL {
			system!(mut sys, {
				let process = process!(sys);
				update_cpu_usage(cpu_usage, process);
			});
		}
		*cpu_usage
	}));

	1
}

pub unsafe extern "C-unwind" fn process_memory_usage(lua: lua::State) -> i32 {
	system!(mut sys, {
		sys.refresh_memory();
		let process = process!(sys);
		lua.push_number(process.memory() as f64 / 1024.);
	});
	1
}

pub unsafe extern "C-unwind" fn logical_cpus(lua: lua::State) -> i32 {
	lua.push_integer(*LOGICAL_CPUS);
	1
}

pub unsafe extern "C-unwind" fn physical_cpus(lua: lua::State) -> i32 {
	lazy_static! {
		static ref PHYSICAL_CPUS: i32 = system!(mut sys, {
			sys.refresh_cpu();
			sys.physical_core_count().unwrap_or(0)
		}) as i32;
	}
	lua.push_integer(*PHYSICAL_CPUS);
	1
}