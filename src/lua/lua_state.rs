use crate::lua::*;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct LuaState(*const std::ffi::c_void);
unsafe impl Send for LuaState {}
impl LuaState {
	#[inline]
	pub unsafe fn push_integer(&self, int: LuaInt) {
		(LUA_SHARED.lua_pushinteger)(*self, int)
	}

	#[inline]
	pub unsafe fn push_number(&self, num: LuaNumber) {
		(LUA_SHARED.lua_pushnumber)(*self, num)
	}

	#[inline]
	pub unsafe fn pop(&self) {
		self.pop_n(1);
	}

	#[inline]
	pub unsafe fn pop_n(&self, count: i32) {
		self.set_top(-count - 1);
	}

	#[inline]
	pub unsafe fn set_top(&self, index: i32) {
		(LUA_SHARED.lua_settop)(*self, index)
	}

	#[inline]
	pub unsafe fn push_function(&self, func: LuaFunction) {
		(LUA_SHARED.lua_pushcclosure)(*self, func, 0)
	}

	#[inline]
	pub unsafe fn push_string(&self, data: &str) {
		(LUA_SHARED.lua_pushlstring)(*self, data.as_ptr() as LuaString, data.len())
	}

	#[inline]
	pub unsafe fn get_field(&self, index: i32, k: LuaString) {
		(LUA_SHARED.lua_getfield)(*self, index, k)
	}

	#[inline]
	pub unsafe fn set_field(&self, index: i32, k: LuaString) {
		(LUA_SHARED.lua_setfield)(*self, index, k)
	}

	pub unsafe fn get_global(&self, name: LuaString) {
		(LUA_SHARED.lua_getfield)(*self, LUA_GLOBALSINDEX, name)
	}

	#[inline]
	pub unsafe fn set_global(&self, name: LuaString) {
		(LUA_SHARED.lua_setfield)(*self, LUA_GLOBALSINDEX, name)
	}

	#[inline]
	pub unsafe fn create_table(&self, seq_n: i32, hash_n: i32) {
		(LUA_SHARED.lua_createtable)(*self, seq_n, hash_n)
	}

	#[inline]
	pub unsafe fn new_table(&self) {
		(LUA_SHARED.lua_createtable)(*self, 0, 0)
	}

	#[inline]
	pub unsafe fn reference(&self) -> i32 {
		(LUA_SHARED.lual_ref)(*self, LUA_REGISTRYINDEX)
	}

	#[inline]
	pub unsafe fn dereference(&self, r#ref: LuaReference) {
		(LUA_SHARED.lual_unref)(*self, LUA_REGISTRYINDEX, r#ref)
	}

	#[inline]
	pub unsafe fn check_type(&self, index: i32, r#type: i32) {
		(LUA_SHARED.lual_checktype)(*self, index, r#type)
	}

	#[inline]
	pub unsafe fn check_function(&self, index: i32) {
		self.check_type(index, LUA_TFUNCTION)
	}

	#[inline]
	pub unsafe fn pcall(&self, nargs: i32, nresults: i32, errfunc: i32) -> i32 {
		(LUA_SHARED.lua_pcall)(*self, nargs, nresults, errfunc)
	}

	#[inline]
	pub unsafe fn call(&self, nargs: i32, nresults: i32) {
		(LUA_SHARED.lua_call)(*self, nargs, nresults)
	}

	#[inline]
	pub unsafe fn push_value(&self, index: i32) {
		(LUA_SHARED.lua_pushvalue)(*self, index)
	}

	#[inline]
	pub unsafe fn raw_geti(&self, t: i32, index: i32) {
		(LUA_SHARED.lua_rawgeti)(*self, t, index)
	}

	pub unsafe fn load_string(&self, src: LuaString) -> Result<(), i32> {
		let lua_error_code = (LUA_SHARED.lual_loadstring)(*self, src);
		if lua_error_code == 0 {
			Ok(())
		} else {
			Err(lua_error_code)
		}
	}
}
impl std::ops::Deref for LuaState {
	type Target = *const std::ffi::c_void;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
