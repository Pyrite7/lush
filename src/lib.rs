use anyhow::Result;
use mlua::{Function, Lua};

use crate::config::{get_config_value, load_config};

mod config;

pub fn run() -> Result<()> {
    let lua = Lua::new();
    let mut rl = rustyline::DefaultEditor::new()?;
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
