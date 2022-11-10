mod app;

use anyhow::Result;
use winit::dpi::LogicalSize;

use app::{App};

fn main() -> Result<()> {
    let app = App::create("Vulkan-rust Test App", LogicalSize::new(1920, 1080))?;
    app.run()
}
