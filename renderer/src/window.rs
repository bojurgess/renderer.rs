use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Window {
    event_loop: EventLoop<()>,
    window:     winit::window::Window,
}

impl Window {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        Self { event_loop, window }
    }

    pub fn run<F>(self, mut draw_fn: F) -> Result<(), EventLoopError>
    where
        F: 'static + FnMut(),
    {
        self.event_loop.set_control_flow(ControlFlow::Poll);

        self.event_loop.run(move |event, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.exit();
            }
            Event::AboutToWait => {
                draw_fn();
            }
            _ => (),
        })
    }
}
