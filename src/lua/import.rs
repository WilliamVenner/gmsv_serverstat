use libloading::{Library, Symbol};

use super::State as LuaState;

pub type LuaInt = std::os::raw::c_int;
pub type LuaSize = usize;
pub type LuaString = *const std::os::raw::c_char;
pub type LuaFunction = unsafe extern "C-unwind" fn(state: LuaState) -> i32;
pub type LuaNumber = std::os::raw::c_double;
pub type LuaReference = LuaInt;

pub const LUA_GLOBALSINDEX: LuaInt = -10002;
pub const LUA_REGISTRYINDEX: LuaInt = -10000;

pub const LUA_TFUNCTION: LuaInt = 6;

lazy_static! {
	pub(super) static ref LUA_SHARED: LuaShared = LuaShared::import();
}

pub(super) struct LuaShared {
	pub lua_setfield: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: LuaInt, k: LuaString)>,
	pub lua_createtable: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, narr: LuaInt, nrec: LuaInt)>,
	pub lua_settop: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, count: LuaInt)>,
	pub lua_pushcclosure: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, func: LuaFunction, upvalues: LuaInt)>,
	pub lua_pushinteger: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, int: LuaInt)>,
	pub lua_pushnumber: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, int: LuaNumber)>,
	pub lual_ref: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: LuaInt) -> LuaInt>,
	pub lual_unref: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: LuaInt, r#ref: LuaInt)>,
	pub lua_getfield: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: LuaInt, k: LuaString)>,
	pub lua_pushlstring: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, data: LuaString, length: LuaSize)>,
	pub lual_checktype: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: LuaInt, r#type: LuaInt)>,
	pub lua_call: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, nargs: LuaInt, nresults: LuaInt)>,
	pub lua_pcall: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, nargs: LuaInt, nresults: LuaInt, errfunc: LuaInt) -> LuaInt>,
	pub lua_pushvalue: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: LuaInt)>,
	pub lua_rawgeti: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, t: LuaInt, index: LuaInt)>,
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
				lual_ref: find_symbol!("luaL_ref"),
				lual_unref: find_symbol!("luaL_unref"),
				lua_getfield: find_symbol!("lua_getfield"),
				lua_pushlstring: find_symbol!("lua_pushlstring"),
				lual_checktype: find_symbol!("luaL_checktype"),
				lua_call: find_symbol!("lua_call"),
				lua_pcall: find_symbol!("lua_pcall"),
				lua_pushvalue: find_symbol!("lua_pushvalue"),
				lua_rawgeti: find_symbol!("lua_rawgeti"),
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
		Library::new("lua_shared.dll").expect("Failed to load lua_shared")
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
