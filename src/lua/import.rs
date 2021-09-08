use libloading::{Library, Symbol};

use super::State as LuaState;

pub type LuaInt = isize;
pub type LuaSize = usize;
pub type LuaString = *const std::os::raw::c_char;
pub type LuaFunction = unsafe extern "C-unwind" fn(state: LuaState) -> i32;
pub type LuaNumber = std::os::raw::c_double;
pub type LuaReference = i32;

pub const LUA_GLOBALSINDEX: i32 = -10002;
pub const LUA_REGISTRYINDEX: i32 = -10000;

pub const LUA_TFUNCTION: i32 = 6;

lazy_static! {
	pub(super) static ref LUA_SHARED: LuaShared = LuaShared::import();
}

pub(super) struct LuaShared {
	pub lua_setfield: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32, k: LuaString)>,
	pub lua_createtable: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, narr: i32, nrec: i32)>,
	pub lua_settop: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, count: i32)>,
	pub lua_pushcclosure: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, func: LuaFunction, upvalues: i32)>,
	pub lua_pushinteger: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, int: LuaInt)>,
	pub lua_pushnumber: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, int: LuaNumber)>,
	pub lual_ref: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> i32>,
	pub lual_unref: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32, r#ref: i32)>,
	pub lua_getfield: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32, k: LuaString)>,
	pub lua_pushlstring: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, data: LuaString, length: LuaSize)>,
	pub lual_checktype: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32, r#type: i32)>,
	pub lua_call: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, nargs: i32, nresults: i32)>,
	pub lua_pcall: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, nargs: i32, nresults: i32, errfunc: i32) -> i32>,
	pub lua_pushvalue: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32)>,
	pub lua_rawgeti: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, t: i32, index: i32)>,
	pub lual_loadstring: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, path: LuaString) -> i32>,
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
				lual_loadstring: find_symbol!("luaL_loadstring"),
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
