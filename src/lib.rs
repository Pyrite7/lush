use anyhow::Result;
use mlua::{Function, Lua};

use crate::config::{get_config_value, load_config};

mod config;

pub fn run() -> Result<()> {
    let lua = unsafe { Lua::unsafe_new() };
    let mut rl = rustyline::DefaultEditor::new()?;
    let conf = load_config(&lua)?;

    loop {
        let prompt_fn: Function = get_config_value(&conf, "prompt")?;
        let prompt: String = prompt_fn.call(())?;
        let line = rl.readline(&prompt)?;
        // Opt-out emergency mechanism to exit, runs before any input handling by users
        let disable_emergency_exit: bool =
            get_config_value(&conf, "disableEmergencyExit").unwrap_or(false);
        if &line == "exit" && !disable_emergency_exit {
            return Ok(());
        }
        let line_handler: Function = get_config_value(&conf, "lineHandler")?;
        let _: () = line_handler.call(line)?;
        // if let Err(error) = lua.load(&line).set_name("=input").exec() {
        //     eprintln!("{}", error);
        // }
    }
}
