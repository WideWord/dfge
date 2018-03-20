#[macro_use] extern crate glium;
extern crate cgmath;

mod asset;
mod platform;
mod util;

fn main() {
	let mut window = platform::Window::new();
	window.run();
}
