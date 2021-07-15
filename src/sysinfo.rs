use std::cell::RefCell;
use std::time::{Duration, Instant};

use ::sysinfo::*;

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
	static ref LOGICAL_CPUS: u16 = system!(sys, sys.processors().len()) as u16;
}

pub fn system_cpu_usage() -> f64 {
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

	TOTAL_CPU_USAGE.with(|cell| {
		let (ref mut cpu_usage, ref mut timestamp) = *cell.borrow_mut();
		if timestamp.elapsed() > CPU_REFRESH_INTERVAL {
			system!(mut sys, update_cpu_usage(cpu_usage, &mut *sys));
		}
		*cpu_usage
	})
}

pub fn system_memory_usage() -> f64 {
	system!(mut sys, {
		sys.refresh_memory();
		sys.used_memory() as f64 / 1024.
	})
}

pub fn system_total_memory() -> f64 {
	lazy_static! {
		static ref TOTAL_MEMORY: f64 = system!(mut sys, {
			sys.refresh_memory();
			sys.total_memory() as f64 / 1024.
		}) as f64;
	}
	*TOTAL_MEMORY
}

pub fn system_available_memory() -> f64 {
	system!(mut sys, {
		sys.refresh_memory();
		sys.available_memory() as f64 / 1024.
	})
}

pub fn process_cpu_usage() -> f64 {
	fn update_cpu_usage(cpu_usage: &mut f64, process: &Process) {
		*cpu_usage = (process.cpu_usage() as f64) / (*LOGICAL_CPUS as f64);
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

	PROCESS_CPU_USAGE.with(|cell| {
		let (ref mut cpu_usage, ref mut timestamp) = *cell.borrow_mut();
		if timestamp.elapsed() > CPU_REFRESH_INTERVAL {
			system!(mut sys, {
				let process = process!(sys);
				update_cpu_usage(cpu_usage, process);
			});
		}
		*cpu_usage
	})
}

pub fn process_memory_usage() -> f64 {
	system!(mut sys, {
		sys.refresh_memory();
		let process = process!(sys);
		process.memory() as f64 / 1024.
	})
}

pub fn logical_cpus() -> u16 {
	*LOGICAL_CPUS
}

pub fn physical_cpus() -> u16 {
	lazy_static! {
		static ref PHYSICAL_CPUS: u16 = system!(mut sys, {
			sys.refresh_cpu();
			sys.physical_core_count().unwrap_or(0)
		}) as u16;
	}
	*PHYSICAL_CPUS
}

#[derive(Copy, Clone, Debug)]
pub struct AllSystem {
	pub cpu_usage: f64,
	pub memory_usage: f64,
	pub total_memory: f64,
	pub available_memory: f64,
	pub logical_cpus: u16,
	pub physical_cpus: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct AllProcess {
	pub cpu_usage: f64,
	pub memory_usage: f64,
}
pub fn all() -> (AllSystem, AllProcess) {
	(all_system(), all_process())
}
pub fn all_system() -> AllSystem {
	AllSystem {
		cpu_usage: system_cpu_usage(),
		memory_usage: system_memory_usage(),
		total_memory: system_total_memory(),
		available_memory: system_available_memory(),
		logical_cpus: logical_cpus(),
		physical_cpus: physical_cpus(),
	}
}
pub fn all_process() -> AllProcess {
	AllProcess {
		cpu_usage: process_cpu_usage(),
		memory_usage: process_memory_usage(),
	}
}