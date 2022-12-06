mod bootstrap;
mod buffer;
mod game;
mod scenes;
mod shader_input;
mod util;

mod app_data;
mod app;
mod builder;
mod frame_info;

use anyhow::{Result};
use winit::dpi::{LogicalSize};

use builder::{HasHeapBuilder};
use app::{App};

use scenes::{marbles};

#[macro_use] extern crate log;

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut builder = App::builder()
        .initial_title("Vulkan-rust Test App")
        .default_size(LogicalSize::new(1920, 1080))
        .add_default_bootstrap_loaders();

    if VALIDATION_ENABLED {
        builder = builder.add_validation();
    }

    let mut app = builder.build()?;
    marbles::create_scene(&mut app.scene)?;
    app.run()
}
