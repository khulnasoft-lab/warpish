use mlua::prelude::*;

/// Initializes the Lua scripting environment.
pub fn initialize_lua() -> LuaResult<Lua> {
    println!("[Scripting] Initializing Lua 5.4 environment...");
    let lua = Lua::new();

    // Expose some application functions to Lua scripts
    let globals = lua.globals();
    
    let log_message = lua.create_function(|_, msg: String| {
        println!("[LUA] {}", msg);
        Ok(())
    })?;

    globals.set("log", log_message)?;

    println!("[Scripting] Lua environment ready.");
    Ok(lua)
}

/// Executes a user script.
pub fn run_script(lua: &Lua, script: &str) -> LuaResult<()> {
    println!("[Scripting] Running script...");
    lua.load(script).exec()?;
    Ok(())
}