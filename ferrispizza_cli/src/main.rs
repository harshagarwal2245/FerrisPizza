mod cli;
mod commands;
mod input;
mod printer;
mod app;

use crate::app::FerrisPizzaApp;

fn main() {
    if let Err(e) = FerrisPizzaApp::new().run() {
        eprintln!(" App crashed: {}", e);
    }
}
