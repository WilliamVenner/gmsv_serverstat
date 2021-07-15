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
	pub unsafe fn set_field(&self, index: i32, k: LuaString) {
		(LUA_SHARED.lua_setfield)(*self, index, k)
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
}
impl std::ops::Deref for LuaState {
	type Target = *const std::ffi::c_void;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
