use anyhow::Result;
use mlua::{Function, Lua};

use crate::{
    config::{get_config_value, load_config},
    unix::load_unix_library,
};

mod config;
mod unix;

pub fn run() -> Result<()> {
    let lua = Lua::new();
    let mut rl = rustyline::DefaultEditor::new()?;

    load_unix_library(&lua)?;
    let conf = load_config(&lua)?;

    loop {
        let prompt_fn: Function = get_config_value(&conf, "prompt")?;
        let prompt: String = prompt_fn.call(())?;
        let line = rl.readline(&prompt)?;
        if let Err(error) = lua.load(&line).set_name("=input").exec() {
            eprintln!("{}", error);
        }
    }
}
