use crate::warp_engine::WarpEngine;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Initialize {
    pub id: serde_json::Value,
}

impl Initialize {
    pub fn process(&self, _warp_engine: &mut WarpEngine) -> String {
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "result": {
                "protocolVersion": "2.0",
                "capabilities": {
                    "tools": {
                        "listChanged": true
                    },
                    "resources": {
                        "listChanged": true,
                        "subscribe": true
                    }
                },
                "serverInfo": {
                    "name": "mcp-warp-engine",
                    "version": "1.0.0"
                }
            }
        })
        .to_string()
    }
}
