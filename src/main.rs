use eframe::{egui, App, CreationContext, Frame, NativeOptions};
use egui_logger::logger_ui;
use log::{error, info, LevelFilter};
use std::io::{self, BufRead};
use std::thread;

use crate::warp_engine::WarpEngine;

mod requests;
mod warp_engine;

struct McpLoggerApp {}

impl App for McpLoggerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Display the logger UI
        egui::CentralPanel::default().show(ctx, |ui| {
            logger_ui().show(ui);
        });

        // Request a repaint to keep updating the UI
        ctx.request_repaint();
    }
}

fn main() {
    // Initialize the logger
    egui_logger::builder()
        .max_level(LevelFilter::Info)
        .init()
        .expect(&format!("Failed to initialize logger"));
    info!("Warp Engine MCP server started");

    // Spawn a thread to read from stdin
    thread::spawn(move || {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let mut warp_engine = WarpEngine::default();

        for line in handle.lines() {
            if let Ok(line) = line {
                info!("Received: {}", line);
                let maybe_request = serde_json::from_str::<serde_json::Value>(&line);
                if let Ok(request) = maybe_request {
                    // Process "real" requests, which include an ID field (we already logged notifications)
                    if request.get("id").is_some() {
                        match serde_json::from_value::<requests::Request>(request) {
                            Err(e) => error!("Failed to parse request: {:?}", e),
                            Ok(request) => {
                                let response = request.process(&mut warp_engine);
                                info!("Sent: {}", response);
                                // Remembering that stdio is connected back to the MCP client.
                                println!("{}", response);
                            }
                        }
                    }
                } else {
                    error!("Failed to parse request: {:?}", maybe_request.err());
                }
            }
        }
    });

    // Create and run the egui application
    let app = McpLoggerApp {};
    let options = NativeOptions::default();

    eframe::run_native(
        "Warp Engine MCP Log",
        options,
        Box::new(|_cc: &CreationContext| Ok(Box::new(app))),
    )
    .expect("Failed to run gui");
}
