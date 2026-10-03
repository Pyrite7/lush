use std::env;

use anyhow::Result;
use mlua::{FromLuaMulti, IntoLuaMulti, Lua, MaybeSend, Table};

pub fn load_unix_library(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    add_fn(lua, &globals, "get_cwd", get_cwd)?;
    add_fn(lua, &globals, "set_cwd", set_cwd)?;
    Ok(())
}

fn add_fn<F, A, R>(lua: &Lua, table: &Table, name: &str, func: F) -> Result<()>
where
    F: Fn(&Lua, A) -> mlua::Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
{
    let wrapped = lua.create_function(func)?;
    table.set(name, wrapped)?;
    Ok(())
}

fn get_cwd(_lua: &Lua, _: ()) -> mlua::Result<String> {
    Ok(env::current_dir()?.to_string_lossy().to_string())
}

fn set_cwd(_lua: &Lua, path: String) -> mlua::Result<()> {
    Ok(env::set_current_dir(&path)?)
}
