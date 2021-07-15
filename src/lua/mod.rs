mod import;
pub use import::*;

mod lua_state;
pub use lua_state::LuaState as State;

#[macro_export]
macro_rules! lua_string {
	( $str:literal ) => {
		#[allow(unused_unsafe)]
		unsafe {
			std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($str, "\0").as_bytes()).as_ptr()
		}
	};

	( $str:expr ) => {
		std::ffi::CString::new($str)
			.expect("Tried to create a Lua string from a string that contained a NUL byte (\\0)!")
			.as_ptr()
	};
}
