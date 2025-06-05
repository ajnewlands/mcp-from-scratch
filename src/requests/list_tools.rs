use crate::warp_engine::WarpEngine;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ListTools {
    pub id: serde_json::Value,
}

impl ListTools {
    pub fn process(&self, _warp_engine: &mut WarpEngine) -> String {
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "result": {
                "tools": [
                    {
                        "name": "set_warp_factor",
                        "description": "Order the helmsman to set the current speed, measured in warp factors, of the starship Enterprise",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "warp_factor": {
                                    "type": "number",
                                    "description": "Speed expresed in warp factors"
                                }
                            },
                            "required": ["warp_factor"]
                        }
                    },
                    {
                        "name": "get_warp_factor",
                        "description": "Ask the helmsman to confirm the current speed, measured in warp factors, of the starship Enterprise",
                        "inputSchema": {
                            "type": "object",
                            "properties": {},
                            "required": []
                        }

                    },
                ]
            }
        }).to_string()
    }
}
