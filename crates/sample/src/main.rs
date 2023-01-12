mod components;
mod scenes;
mod shader_input;

use anyhow::{Result};
use winit::dpi::{LogicalSize};

use engine::{
    builder::{HasHeapBuilder},
    app::{App}
};

use scenes::{marbles};

#[macro_use] extern crate log;

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

fn main() -> Result<()> {
    pretty_env_logger::init();
    info!("Sample app starting up...");

    let mut builder = App::builder()
        .initial_title("Rust Engine Sample App")
        .default_size(LogicalSize::new(1920, 1080))
        .add_default_bootstrap_loaders()
        .add_dlss();

    if VALIDATION_ENABLED {
        builder = builder.add_validation();
    }

    let mut app = builder.build()?;
    marbles::create_scene(&mut app.scene)?;
    app.run()
}
