use anyhow::{Result, bail};
use mlua::{FromLua, Lua, Table};

#[derive(Debug)]
pub struct Config(Table);

pub fn load_config(lua: &Lua) -> Result<Config> {
    load_from_str(lua, include_str!("default_config.lua"))
}

pub fn get_config_value<V: FromLua>(config: &Config, key: &str) -> Result<V> {
    Ok(config.0.get(key)?)
}

fn load_from_str(lua: &Lua, s: &str) -> Result<Config> {
    Ok(Config(lua.load(s).call(())?))
}

#[cfg(test)]
mod tests {
    use crate::config::load_from_str;

    use super::*;

    #[test]
    fn test_load_from_str() {
        let lua = Lua::new();
        assert!(load_from_str(&lua, "return { key1 = 2, key2 = \"value\" }").is_ok());
    }

    #[test]
    fn test_get_config_value() {
        let lua = Lua::new();
        let config = load_from_str(&lua, "return { key1 = 2, key2 = \"value\" }").unwrap();
        let _key1: usize = get_config_value(&config, "key1")
            .expect("config should contain key1 with an integer value");
        let _key2: String = get_config_value(&config, "key2")
            .expect("config should contain key2 with a string value");
    }
}
