use crate::requests::ToolParameters;

pub struct WarpEngine {
    current_speed: f32,
}

impl Default for WarpEngine {
    fn default() -> Self {
        WarpEngine { current_speed: 1.0 }
    }
}

impl WarpEngine {
    pub fn handle_order(&mut self, order: &ToolParameters) -> Result<String, String> {
        match order {
            ToolParameters::SetWarpFactor(params) => {
                let warp_factor = params.arguments.warp_factor;
                if warp_factor < 1.0 {
                    return Err(
                        "Mr Scott says that order violates the laws of physics, Captain!"
                            .to_string(),
                    );
                } else if warp_factor > 10.0 {
                    return Err(
                        "Mr Scott says if he gives her any more she'll blow, Captain!".to_string(),
                    );
                }
                self.current_speed = warp_factor;
                Ok(format!("Ahead, warp factor {}, sir!", warp_factor))
            }
            ToolParameters::GetWarpFactor => Ok(format!(
                "Current warp factor is {} sir!",
                self.current_speed
            )),
        }
    }
}
