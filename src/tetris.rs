use core::ptr;
use crate::pause_control::PauseControl;
use super::random;
use super::winc::WindowController;
use windows_sys::core::s;
use windows_sys::Win32::Foundation::RECT;

const FRAM: u32 = 2;
const X_MAX: u32 = 8;
const Y_MAX: u32 = 17;
const X_START: i32 = 3;
const Y_START: u32 = 0;
const BLOCK_SIZE: u32 = 17;

pub const BOARD_RECT: RECT = RECT
{
	left: 0,
	top: 0,
	right: (X_MAX * BLOCK_SIZE + FRAM) as i32,
	bottom: (Y_MAX * BLOCK_SIZE + FRAM) as i32
};

pub struct Tetris
{
	current_shape: i32,
	next_shape: i32,
	current_shape_type: usize,
	next_shape_type: usize,
	current_shape_variant: usize,
	current_shape_variants_count: i32,
	score: u32,
	ground : [[bool; Y_MAX as usize + 4]; X_MAX as usize + 5],
	x: i32,
	y: u32,
}

pub struct LocationSize
{
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
}

pub const fn get_next_location_size() -> LocationSize
{
	return LocationSize { x: (8 * BLOCK_SIZE + FRAM) as i32, y:  0, width: ((8 * BLOCK_SIZE + 6) / 2) as i32, height: 21 };
}

pub const fn get_score_location_size() -> LocationSize
{
	return LocationSize { x: (8 * BLOCK_SIZE + FRAM) as i32, y:  102, width: ((8 * BLOCK_SIZE + 6) / 2) as i32, height: 102 };
}

#[macro_export]
macro_rules! bit { ($number:expr, $x:expr, $y:expr) => { ($number & (1 << (15 - 4 * $y - $x)) != 0) } }

#[macro_export]
macro_rules! set_ground 
{ 
	($ground:expr, $x:expr, $y:expr, $value:expr) =>
	{
		$ground[($x as usize)][($y as usize)] = ($value);
	}
}

macro_rules! RECT 
{ 
	($left:expr, $top:expr, $right:expr, $bottom:expr) =>
	{
		RECT { left: ($left) as i32, top: ($top) as i32, right: ($right) as i32, bottom: ($bottom) as i32 }
	}
}

#[inline(always)]
fn safe_u32(i: i32) -> u32
{
	return if i > 0 { i as u32 } else { 0 };
}

impl Tetris
{
	pub const fn new() -> Self 
	{
		Tetris 
		{ 
			x: X_START, 
			y: Y_START,
			score: 0,
			current_shape: 0,
			next_shape: 0,
			current_shape_type: 0,
			next_shape_type: 0,
			current_shape_variant: 0,
			current_shape_variants_count: 0,
			ground: [[false; Y_MAX as usize + 4]; X_MAX as usize + 5],
		}
	}

	fn draw_current(&mut self, windows: &WindowController)
	{
		let rc: RECT = RECT!
		(
			FRAM / 2 + safe_u32(self.x) * BLOCK_SIZE,
			FRAM / 2 + self.y * BLOCK_SIZE,
			FRAM / 2 + safe_u32(self.x + 4) * BLOCK_SIZE,
			FRAM / 2 + (self.y + 4) * BLOCK_SIZE
		);
		windows.invalidate(&rc, false);
	}

	fn draw_next(&mut self, windows: &WindowController)
	{
		let rc: RECT = RECT!
		(
			FRAM / 2 + 156,
			FRAM / 2 + 21,
			FRAM / 2 + 156 + 4 * BLOCK_SIZE,
			FRAM / 2 + 21 + 4 * BLOCK_SIZE
		);
		windows.invalidate(&rc, true);
	}

	pub fn new_block(&mut self, windows: &WindowController)
	{
		self.x = X_START;
		self.y = Y_START;
		self.current_shape_type = self.next_shape_type;
		self.current_shape = self.next_shape;
		self.current_shape_variant = 0;
		self.current_shape_variants_count = SHAPES[self.current_shape_type][0];
		self.draw_current(windows);
		self.next_shape_type = (random::next_byte() % 7) as usize;
		self.next_shape = SHAPES[self.next_shape_type][1];
		self.draw_next(windows);
	}

	fn check_collision(&mut self, direction: &Direction, shape_to_check: i32) -> bool
	{
		let mut dx: usize = 0;
		let mut dy: usize = 0;
			//let shape_to_check = match direction { Direction::None => self.rotated_shape, _ => self.current_shape };
			
			
		match direction 
		{
			Direction::Down => 
			{
				dx = 1;
				dy = 1;
			},
			Direction::Right => 
			{
				dx = 2;
			},
			Direction::None => 
			{
				dx = 1;
			}
			_ => {}
		}

		for x in 0..4
		{
			for y in 0..4
			{
				if bit!(shape_to_check, x, y) && self.ground[safe_u32(self.x + x as i32 + dx as i32) as usize][y + self.y as usize+ dy]
				{
					return true;
				}
			}
		}
		return false;
	}

	pub fn draw(&mut self, windows: &WindowController) 
	{
		for x in 0..4u32
		{
			for y in 0..4u32
			{	
				if bit!(self.current_shape, x, y) 
				{
					windows.bit_blt(FRAM / 2 + safe_u32(self.x + x as i32) * BLOCK_SIZE, FRAM/ 2 + (self.y + y) * BLOCK_SIZE, BLOCK_SIZE);
				}
				if bit!(self.next_shape, x, y) 
				{
					windows.bit_blt(FRAM / 2 + 156 + x * BLOCK_SIZE, FRAM / 2 + 21 + y * BLOCK_SIZE, BLOCK_SIZE);
				}
			}
		}
		for x in 0..X_MAX
		{
			for y in 0..Y_MAX
			{	
				if self.ground[x as usize + 1][y as usize]
				{
					windows.bit_blt(FRAM / 2 + x * BLOCK_SIZE , FRAM / 2 + y * BLOCK_SIZE, BLOCK_SIZE);
				}
			}
		}
	}

	pub fn new_game(&mut self, windows: &WindowController) 
	{
		self.score = 0;
		windows.show_score(self.score);
		for y in 0..Y_MAX
		{
			for x in 0..X_MAX
			{	
				set_ground!(self.ground, x + 1, y, false);
			}
		}
		for y in 0..Y_MAX
		{
			set_ground!(self.ground, X_MAX + 1, y, true);
			set_ground!(self.ground, 0, y, true);
		}
		for x in 0..X_MAX
		{	
			set_ground!(self.ground, x + 1, Y_MAX, true);
		}
		self.next_shape_type = (random::next_byte() % 7) as usize;
		self.next_shape = SHAPES[self.next_shape_type][1];
		windows.invalidate(ptr::null(), true);
		self.new_block(windows);
	}

	pub fn block_rotate(&mut self, windows: &WindowController)
	{
		let rotated_variant = ((self.current_shape_variant as i32 + 1) % self.current_shape_variants_count) as usize;
		let rotated_shape = SHAPES[self.current_shape_type][rotated_variant + 1];
		if !self.check_collision(&Direction::None, rotated_shape)
		{
			self.current_shape_variant = rotated_variant;
			self.current_shape = rotated_shape;
			self.draw_current(windows);
		}
	}

	fn game_over(&mut self, pause_control: &mut PauseControl, windows: &WindowController)
	{
		pause_control.pause();
		windows.message_box(s!("You lose"), s!("Game Over"));
		self.new_game(windows);
		pause_control.unpause();
	}

	fn remove_lines(&mut self, size: usize, &lines_to_remove: &[usize; Y_MAX as usize + 1])
	{
		let mut i = size;
		loop
		{
			i-=1;
			for x in 0..X_MAX
			{
				for y in (1..=lines_to_remove[i]).rev()
				{
					set_ground!(self.ground, x + 1, y, if y > 0 { self.ground[x as usize + 1][y as usize - 1] } else { false });
				}
			}
			if i == 0
			{
				return;
			}
		}
	}

	fn check_line(&mut self, pause_control: &mut PauseControl, windows: &WindowController)
	{
		let mut size = 0usize;
		let mut lines_to_remove = [0usize; Y_MAX as usize + 1];
		for y in (1..Y_MAX).rev() 
		{
			let mut line = true;
			for x in 0..X_MAX
			{
				line = self.ground[x as usize + 1][y as usize] && line
			}
			if line
			{
				lines_to_remove[size] = y as usize;
				size+=1;
			}
		}
		if size > 0
		{
			self.remove_lines(size, &lines_to_remove);
			windows.invalidate(ptr::null(), true);
			self.score += (size * size * 10) as u32;
			windows.show_score(self.score);
		}
		for x  in 0..8 as usize
		{
			for y in 1..(Y_MAX+1) as usize
			{
				if (self.ground[x + 1][y] || y == Y_MAX as usize) && y < 2
				{
					self.game_over(pause_control, windows);
					return;
				}
			}
		}
	}

	pub fn block_move(&mut self, direction: Direction, pause_control: &mut PauseControl, windows: &WindowController)
	{
		if !self.check_collision(&direction, self.current_shape)
		{
			let rc = match direction
			{
				Direction::Left =>
				{
					self.x -= 1;
					RECT!
					(
						FRAM / 2 + safe_u32(self.x) * BLOCK_SIZE,
						FRAM / 2 + self.y * BLOCK_SIZE,
						FRAM / 2 + safe_u32(self.x + 5) * BLOCK_SIZE,
						FRAM / 2 + (self.y + 4) * BLOCK_SIZE
					)
				},
				Direction::Right =>
				{
					self.x += 1;
					RECT!
					(
						FRAM / 2 + safe_u32(self.x - 1) * BLOCK_SIZE,
						FRAM / 2 + self.y * BLOCK_SIZE,
						FRAM / 2 + safe_u32(self.x + 4) * BLOCK_SIZE,
						FRAM / 2 + (self.y + 4) * BLOCK_SIZE
					)
				},
				Direction::Down =>
				{
					self.y += 1;
					RECT!
					(
						FRAM / 2 + safe_u32(self.x) * BLOCK_SIZE,
						FRAM / 2 + (self.y - 1) * BLOCK_SIZE,
						FRAM / 2 + safe_u32(self.x + 4) * BLOCK_SIZE,
						FRAM / 2 + (self.y + 4) * BLOCK_SIZE
					)
				},				
				_ => { RECT!(0, 0, 0, 0) }
			};
			windows.invalidate(&rc, false);
		}
		else 
		{
			match direction
			{
				Direction::Down =>
				{
					for x in 0..4u32
					{
						for y in 0..4u32
						{	
							let ix = safe_u32(self.x + 1 + x as i32) as usize;
							let iy = (self.y + y) as usize;
							set_ground!(self.ground, ix, iy,
								bit!(self.current_shape, x, y) |
								self.ground[ix][iy]);
						}
					}					
					self.check_line(pause_control, windows);
					self.new_block(windows);
				},
				_ => {}
			}
		}
	}
}

pub enum Direction 
{
	None,
	Down,
	Left,
	Right
}

macro_rules! shape
{
	($line0:tt $line1:tt $line2:tt $line3:tt) =>
	{
		($line0 << 12) |
		($line1 << 	8) |
		($line2 <<  4) |
		($line3 <<  0)
	}
}
const SHAPES:  [[i32; 5]; 7] =
[
	[
		2,
		shape!(
			0b_0100
			0b_0100
			0b_0100
			0b_0100
		),
		shape!(
			0b_0000
			0b_1111
			0b_0000
			0b_0000
		), 0, 0
	],
	[
		2,
		shape!(
			0b_0000
			0b_1100
			0b_0110
			0b_0000
		),
		shape!(
			0b_0100
			0b_1100
			0b_1000
			0b_0000
		), 0, 0
	],
	[
		2,
		shape!(
			0b_0000
			0b_0110
			0b_1100
			0b_0000
		),
		shape!(
			0b_1000
			0b_1100
			0b_0100
			0b_0000
		), 0, 0
	],
	[
		4,
		shape!(
			0b_0110
			0b_0100
			0b_0100
			0b_0000
		),
		shape!(
			0b_0000
			0b_1110
			0b_0010
			0b_0000
		),
		shape!(
			0b_0100
			0b_0100
			0b_1100
			0b_0000
		),
		shape!(
			0b_1000
			0b_1110
			0b_0000
			0b_0000
		)
	],
	[
		4,
		shape!(
			0b_1100
			0b_0100
			0b_0100
			0b_0000
		),
		shape!(
			0b_0010
			0b_1110
			0b_0000
			0b_0000
		),
		shape!(
			0b_0100
			0b_0100
			0b_0110
			0b_0000
		),
		shape!(
			0b_0000
			0b_1110
			0b_1000
			0b_0000 
		)
	],
	[
		1,
		shape!(
			0b_1100
			0b_1100
			0b_0000
			0b_0000
		), 0, 0, 0
	],
	[
		4,
		shape!(
			0b_0100
			0b_1110
			0b_0000
			0b_0000
		),
		shape!(
			0b_0100
			0b_0110
			0b_0100
			0b_0000
		),
		shape!(
			0b_0000
			0b_1110
			0b_0100
			0b_0000
		),
		shape!(
			0b_0100
			0b_1100
			0b_0100
			0b_0000
		)
	]
];