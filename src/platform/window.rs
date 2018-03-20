use glium::{self, glutin};

pub struct Window {
	events_loop: glutin::EventsLoop,
	display: glium::Display,
}

impl Window {

	pub fn new() -> Self {
		let events_loop = glutin::EventsLoop::new();
		let window = glutin::WindowBuilder::new();
		let context = glutin::ContextBuilder::new();
		let display = glium::Display::new(window, context, &events_loop).unwrap();

		Window {
			events_loop: events_loop,
			display: display,
		}
	}

	pub fn run(&mut self) {
		let mut closed = false;
		while !closed {
			self.events_loop.poll_events(|ev| {
				match ev {
					glutin::Event::WindowEvent { event, .. } => match event {
						glutin::WindowEvent::Closed => closed = true,
						_ => (),
					},
					_ => (),
				}
			});
		}
	}

}
