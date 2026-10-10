use std::{env, fs, path::Path};

use anyhow::{Result, anyhow};
use mlua::{FromLua, Lua, Table};

#[derive(Debug)]
pub struct Config(Table);

pub fn load_config(lua: &Lua) -> Result<Config> {
    load_from_env(lua).or_else(|_| load_default_config(lua))
}

pub fn get_config_value<V: FromLua>(config: &Config, key: &str) -> Result<V> {
    Ok(config.0.get(key)?)
}

fn load_default_config(lua: &Lua) -> Result<Config> {
    load_from_str(lua, include_str!("default_config.lua"), "=<default config>")
}

const LUSH_CONFIG_ENV_VARIABLE_NAME: &str = "LUSH_CONFIG";

fn load_from_env(lua: &Lua) -> Result<Config> {
    env::var(LUSH_CONFIG_ENV_VARIABLE_NAME)
        .map_err(|e| anyhow!(e))
        .and_then(|path| load_from_file(lua, &path))
}

fn load_from_file(lua: &Lua, path: &impl AsRef<Path>) -> Result<Config> {
    fs::read_to_string(path)
        .map_err(|e| anyhow!(e))
        .and_then(|contents| load_from_str(lua, &contents, &path.as_ref().to_string_lossy()))
}

fn load_from_str(lua: &Lua, s: &str, chunk_name: &str) -> Result<Config> {
    Ok(Config(lua.load(s).set_name(chunk_name).call(())?))
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use crate::config::load_from_str;

    use super::*;

    #[test]
    fn test_load_from_str() {
        let lua = unsafe { Lua::unsafe_new() };
        assert!(load_from_str(&lua, "return { key1 = 2, key2 = \"value\" }", "=<test>").is_ok());
    }

    #[test]
    fn test_get_config_value() {
        let lua = unsafe { Lua::unsafe_new() };
        let config =
            load_from_str(&lua, "return { key1 = 2, key2 = \"value\" }", "=<test>").unwrap();
        let _key1: usize = get_config_value(&config, "key1")
            .expect("config should contain key1 with an integer value");
        let _key2: String = get_config_value(&config, "key2")
            .expect("config should contain key2 with a string value");
    }

    #[test]
    fn test_default_config() {
        let lua = unsafe { Lua::unsafe_new() };
        let _config = load_default_config(&lua).expect("default config should be valid");
    }
}
