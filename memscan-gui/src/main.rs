mod app;
mod app_error;
mod data_types;
mod endian;
mod process_picker;
mod search;
mod search_scope;
mod settings;

use std::process::Command;

use app::TemplateApp;

fn main() -> eframe::Result {
    // if escalate_if_needed() {
    //     return Ok(());
    // }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(TemplateApp::new(cc)))),
    )
}

pub fn escalate_if_needed() -> bool {
    use nix::libc::{geteuid, getuid, setuid};
    match unsafe { (getuid(), geteuid()) } {
        (0, 0) => {
            println!("Running as root");
        }
        (_, 0) => {
            println!("Setuid to root");
            unsafe { setuid(0) };
        }
        (_, _) => {
            println!("Not running as root");
            println!("Attempting to run pkexec");
            let mut args: Vec<_> = std::env::args().collect();
            if let Some(absolute_path) = std::env::current_exe()
                .ok()
                .and_then(|p| p.to_str().map(|p| p.to_string()))
            {
                args[0] = absolute_path;
            }
            let mut command = Command::new("pkexec");
            command.arg("env");

            for (name, value) in std::env::vars() {
                command.arg(format!("{}={}", name, value));
            }

            command
                .args(args)
                .spawn()
                .expect("Failed to run pkexec")
                .wait()
                .expect("Failed to wait for pkexec");
            return false;
        }
    }
    return true;
}
