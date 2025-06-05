mod initialize;
use initialize::Initialize;

mod list_tools;
use list_tools::ListTools;

mod call_tools;
use crate::WarpEngine;
pub use call_tools::{CallTools, ToolParameters};

macro_rules! BuildRequests {
    ([ $( ($name: ident, $alias: expr) ), *]) => {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(tag = "method")]
        /// Generic type used to deserialize incoming messages from the MCP client.
        pub enum Request {
            $(
                #[serde(alias= $alias)]
            $name($name),

            )*
        }

        impl Request {
            pub fn process(&self, warp_engine: &mut WarpEngine) -> String {
                match self {
                    $(
                        Self::$name(payload) => payload.process(warp_engine),
                    )*
                }
            }
        }
    };
}

BuildRequests!([
    (Initialize, "initialize"),
    (ListTools, "tools/list"),
    (CallTools, "tools/call")
]);
