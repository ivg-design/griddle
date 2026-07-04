// Griddle — deterministic Rive MCP client. No AI, no agent: the frontend
// builds one JSON-RPC tools/call payload and this command POSTs it to the
// Rive editor's MCP server.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const MCP_URL: &str = "http://127.0.0.1:9791/mcp";

fn post(payload: &str) -> Result<String, String> {
    let resp = ureq::post(MCP_URL)
        .set("Content-Type", "application/json")
        .set("Accept", "application/json, text/event-stream")
        .send_string(payload)
        .map_err(|e| format!("MCP request failed: {e}"))?;
    resp.into_string()
        .map_err(|e| format!("MCP response read failed: {e}"))
}

// The Rive MCP server requires the initialize handshake after every editor
// restart; until then tools/call returns 400 "Server not initialized".
fn handshake() -> Result<(), String> {
    post(r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"griddle","version":"1.0.5"}}}"#)?;
    // Notification: server replies 202 with an empty body; errors are fine to ignore.
    let _ = post(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);
    Ok(())
}

#[tauri::command]
fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn mcp_call(payload: String) -> Result<String, String> {
    match post(&payload) {
        Ok(body) => Ok(body),
        Err(_) => {
            // Editor may have restarted since our last call — re-handshake
            // and retry the original request once.
            handshake()?;
            post(&payload)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![mcp_call, app_version])
        .run(tauri::generate_context!())
        .expect("error while running griddle");
}
