mod bootstrap;
mod app;
mod builder;

use anyhow::{Result};
use bootstrap::bootstrap_validation_loader::BootstrapValidationLoader;
use winit::dpi::{LogicalSize};

use builder::{HasHeapBuilder};
use app::{App};

#[macro_use] extern crate log;

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut builder = App::builder()
        .initial_title("Vulkan-rust Test App")
        .default_size(LogicalSize::new(1920, 1080));

    if VALIDATION_ENABLED {
        let validation_loader = Box::new(BootstrapValidationLoader::new());
        builder = builder.add_bootstrap_loader(validation_loader);
    }

    let app = builder.build()?;
    app.run()
}
