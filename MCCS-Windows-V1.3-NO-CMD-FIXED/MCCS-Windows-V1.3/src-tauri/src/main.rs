#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let url = "https://miran-commercial-control-system.vercel.app/"
                .parse()
                .expect("valid MCCS URL");
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("MCCS - Miran Commercial Control System")
                .inner_size(1440.0, 900.0)
                .min_inner_size(1024.0, 700.0)
                .resizable(true)
                .maximized(true)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running MCCS");
}
