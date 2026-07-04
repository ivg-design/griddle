// Griddle — deterministic Rive MCP client. No AI, no agent: the frontend
// builds one JSON-RPC tools/call payload and this command POSTs it to the
// Rive editor's MCP server.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const MCP_URL: &str = "http://127.0.0.1:9791/mcp";

#[tauri::command]
fn mcp_call(payload: String) -> Result<String, String> {
    let resp = ureq::post(MCP_URL)
        .set("Content-Type", "application/json")
        .set("Accept", "application/json, text/event-stream")
        .send_string(&payload)
        .map_err(|e| format!("MCP request failed: {e}"))?;
    resp.into_string()
        .map_err(|e| format!("MCP response read failed: {e}"))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![mcp_call])
        .run(tauri::generate_context!())
        .expect("error while running griddle");
}
