mod app;

use anyhow::Result;
use winit::dpi::LogicalSize;

use app::{App};

#[macro_use] extern crate log;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let app: App;
    unsafe {
        app = App::create("Vulkan-rust Test App", LogicalSize::new(1920, 1080))?;
    }
    app.run()
}
