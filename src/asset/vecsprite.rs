use cgmath::*;
use ::util::Color;

pub struct VecSprite {
	pub size: Vector2<f32>,
	pub anchor: Vector2<f32>,
	pub lines: Vec<VecSpriteLine>,
	pub fillings: Vec<VecSpriteFilling>,
}

pub struct VecSpriteLine {
	pub position: [Vector2<f32>; 2],
	pub thickness: f32,
	pub color: Color,
}

pub struct VecSpriteFilling {
	pub position: [Vector2<f32>; 3],
	pub color: Color,
}

impl VecSprite {

	pub fn empty() -> Self {
		VecSprite {
			size: vec2(0.0, 0.0),
			anchor: vec2(0.0, 0.0),
			lines: Vec::new(),
			fillings: Vec::new(),
		}
	}

}
