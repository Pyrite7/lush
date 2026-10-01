use anyhow::Result;
use mlua::Lua;

mod config;

pub fn run() -> Result<()> {
    let lua = Lua::new();
    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        let line = rl.readline(">> ")?;
        if let Err(error) = lua.load(&line).set_name("=input").exec() {
            eprintln!("{}", error);
        }
    }
}
