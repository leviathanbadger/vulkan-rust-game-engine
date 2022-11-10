use anyhow::Result;
use winit::{
    dpi::{LogicalSize},
    window::{Window, WindowBuilder},
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent}
};

#[derive(Debug)]
pub struct App {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window
}

impl App {
    pub fn create(initial_title: &str, default_size: LogicalSize<i32>) -> Result<Self> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(initial_title)
            .with_inner_size(default_size)
            .build(&event_loop)?;

        Ok(Self {
            event_loop: Some(event_loop),
            window
        })
    }

    pub fn run(mut self) -> ! {
        let mut destroying = false;
        //TODO: Don't abuse Option<> in the struct in order to call run on the event loop without causing an ownership error
        self.event_loop.take().unwrap().run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::MainEventsCleared if !destroying => {
                    self.render().unwrap();
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    destroying = true;
                    *control_flow = ControlFlow::Exit;
                    self.destroy();
                }
                _ => { }
            }
        });
    }

    pub fn render(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn destroy(&mut self) { }
}
