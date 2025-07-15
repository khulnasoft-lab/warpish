use mlua::{Lua, prelude::LuaResult}; // FIX: Correctly import LuaResult

/// Initializes the Lua scripting environment.
pub fn initialize_lua() -> LuaResult<Lua> {
    println!("[Scripting] Initializing Lua 5.4 environment...");
    let lua = Lua::new();

    // Expose some application functions to Lua scripts
    {
        let globals = lua.globals();
        
        let log_message = lua.create_function(|_, msg: String| {
            println!("[LUA] {}", msg);
            Ok(())
        })?;

        globals.set("log", log_message)?;
    }

    println!("[Scripting] Lua environment ready.");
    Ok(lua)
}

/// Executes a user script.
pub fn run_script(lua: &Lua, script: &str) -> LuaResult<()> {
    println!("[Scripting] Running script...");
    lua.load(script).exec()?;
    Ok(())
}

pub fn init_lua_context() -> LuaResult<Lua> {
    let lua = Lua::new();
    {
        let _globals = lua.globals();
        // You can set up your initial Lua environment here.
    } // `_globals` is dropped here.

    // FIX: Explicitly drop `_globals` before returning `lua` to satisfy borrow checker.
    // While the scope drop is usually enough, being explicit can solve tricky cases.
    // In this case, the scope `{}` block already solves it, but `drop` is another tool.
    
    Ok(lua)
}