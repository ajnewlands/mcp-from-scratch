use crate::warp_engine::WarpEngine;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CallTools {
    pub id: serde_json::Value,
    pub params: ToolParameters,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "name")]
pub enum ToolParameters {
    #[serde(rename = "set_warp_factor")]
    SetWarpFactor(SetWarpFactor),
    #[serde(rename = "get_warp_factor")]
    GetWarpFactor,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SetWarpFactor {
    pub arguments: SetWarpFactorArgs,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SetWarpFactorArgs {
    pub warp_factor: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetWarpFactor {}

impl CallTools {
    pub fn process(&self, warp_engine: &mut WarpEngine) -> String {
        let text: String;
        let is_error: bool;

        match warp_engine.handle_order(&self.params) {
            Ok(result_string) => {
                text = result_string;
                is_error = false;
            }
            Err(error_string) => {
                text = error_string;
                is_error = true;
            }
        }

        serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "result": {
                "content": [{
                    "type": "text",
                    "text": text
                }],
                "isError": is_error
            }
        })
        .to_string()
    }
}
