
pub struct Color {
	red: f32,
	green: f32,
	blue: f32,
	alpha: f32,
}

impl Color {

	pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		Color {
			red: red,
			green: green,
			blue: blue,
			alpha: alpha,
		}
	}

}

const WHITE: Color = Color{ 
	red: 1.0, 
	green: 1.0,
	blue: 1.0,
	alpha: 1.0,
};

const BLACK: Color = Color{ 
	red: 0.0, 
	green: 0.0,
	blue: 0.0,
	alpha: 1.0,
};

const RED: Color = Color{ 
	red: 1.0, 
	green: 0.0,
	blue: 0.0,
	alpha: 1.0,
};

const GREEN: Color = Color{ 
	red: 0.0, 
	green: 1.0,
	blue: 0.0,
	alpha: 1.0,
};

const BLUE: Color = Color{ 
	red: 0.0, 
	green: 0.0,
	blue: 1.0,
	alpha: 1.0,
};
