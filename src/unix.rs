use std::{env, path::PathBuf, process::Command};

use anyhow::{Result, anyhow};
use mlua::{FromLuaMulti, IntoLua, IntoLuaMulti, Lua, MaybeSend, Table};

pub fn load_unix_library(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    add_fn(lua, &globals, "get_cwd", get_cwd)?;
    add_fn(lua, &globals, "set_cwd", set_cwd)?;
    add_fn(lua, &globals, "get_env_var", get_env_var)?;
    add_fn(lua, &globals, "set_env_var", set_env_var)?;
    add_fn(lua, &globals, "remove_env_var", remove_env_var)?;
    add_fn(lua, &globals, "spawn_process", spawn_process)?;
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

fn get_env_var(_lua: &Lua, name: String) -> mlua::Result<Option<String>> {
    match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(err) => Err(mlua::Error::external(err)),
    }
}

fn set_env_var(_lua: &Lua, (name, value): (String, String)) -> mlua::Result<()> {
    // TODO: implement in a (thread-)safe way with an internal hashtable instead of mutating the
    // actual environment of the lush process
    unsafe {
        env::set_var(name, value);
    }
    Ok(())
}

fn remove_env_var(_lua: &Lua, name: String) -> mlua::Result<()> {
    // TODO: implement in a (thread-)safe way with an internal hashtable instead of mutating the
    // actual environment of the lush process
    unsafe {
        env::remove_var(name);
    }
    Ok(())
}

fn spawn_process(_lua: &Lua, (args, options): (Vec<String>, Option<Table>)) -> mlua::Result<()> {
    let mut cmd =
        Command::new(args.first().ok_or_else(|| {
            mlua::Error::external(anyhow!("no program provided for spawn_process"))
        })?);
    cmd.args(args.into_iter().skip(1));

    if let Some(options) = options {
        if let Some(cwd) = options.get::<Option<PathBuf>>("cwd")? {
            cmd.current_dir(cwd);
        }
    }

    cmd.spawn().map_err(mlua::Error::external)?.wait()?;
    Ok(())
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn test_load_unix_library() {
        let lua = Lua::new();
        load_unix_library(&lua).expect("unix library should be loaded successfully");
        let fns = [
            "get_cwd",
            "set_cwd",
            "get_env_var",
            "set_env_var",
            "remove_env_var",
        ];
        for f in fns {
            assert!(
                lua.globals()
                    .get::<mlua::Value>(f)
                    .is_ok_and(|val| val.is_function())
            );
        }
    }

    #[test]
    fn test_get_missing_env_var() {
        let lua = Lua::new();
        load_unix_library(&lua).expect("unix library should be loaded successfully");
        let val: mlua::Value = lua
            .load("get_env_var('LZIUNEAHFYAETBHJAEZJHBA')")
            .eval()
            .expect("expression should be evaluated successfully");
        assert_eq!(val, mlua::Value::Nil);
    }
}
