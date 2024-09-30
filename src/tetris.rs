use core::ptr;

use crate::game;

use super::random;
use super::winc;
use windows_sys::core::s;
use windows_sys::Win32::Foundation::RECT;
const FRAM: u32 = 2;
const X_MAX: u32 = 8;
const Y_MAX: u32 = 17;
const X_START: u32 = 3;
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
	x: u32,
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
	($x:expr, $y:expr, $value:expr) =>
	{
		TETRIS.ground[($x as usize)][($y as usize)] = ($value);
	}
}

macro_rules! RECT 
{ 
	($left:expr, $top:expr, $right:expr, $bottom:expr) =>
	{
		RECT { left: ($left) as i32, top: ($top) as i32, right: ($right) as i32, bottom: ($bottom) as i32 }
	}
}

static mut TETRIS: Tetris = Tetris 
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
};

fn draw_current()
{
	unsafe 
	{
		let rc: RECT = RECT!
		(
			FRAM / 2 + TETRIS.x * BLOCK_SIZE,
			FRAM / 2 + TETRIS.y * BLOCK_SIZE,
			FRAM / 2 + (TETRIS.x + 4) * BLOCK_SIZE,
			FRAM / 2 + (TETRIS.y + 4) * BLOCK_SIZE
		);
		winc::invalidate(&rc, false);
	}
}

fn draw_next()
{
	let rc: RECT = RECT!
	(
		FRAM / 2 + 156,
		FRAM / 2 + 21,
		FRAM / 2 + 156 + 4 * BLOCK_SIZE,
		FRAM / 2 + 21 + 4 * BLOCK_SIZE
	);
	winc::invalidate(&rc, true);
}

pub fn new_block()
{
	unsafe 
	{
		TETRIS.x = X_START;
		TETRIS.y = Y_START;
		TETRIS.current_shape_type = TETRIS.next_shape_type;
		TETRIS.current_shape = TETRIS.next_shape;
		TETRIS.current_shape_variant = 0;
		TETRIS.current_shape_variants_count = SHAPES[TETRIS.current_shape_type][0];
		draw_current();
		TETRIS.next_shape_type = (random::next_byte() % 7) as usize;
		TETRIS.next_shape = SHAPES[TETRIS.next_shape_type][1];
		draw_next();
	}
}

fn check_collision(direction: &Direction, shape_to_check: i32) -> bool {
	let mut dx: usize = 0;
	let mut dy: usize = 0;
		//let shape_to_check = match direction { Direction::None => TETRIS.rotated_shape, _ => TETRIS.current_shape };
		
		
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
			unsafe 
			{
				if bit!(shape_to_check, x, y) && TETRIS.ground[TETRIS.x as usize + x + dx][y + TETRIS.y as usize+ dy]
				{
					return true;
				}
			}
		}
	}
	return false;
}

pub fn draw() 
{
	for x in 0..4u32
	{
		for y in 0..4u32
		{	
			unsafe 
			{
				if bit!(TETRIS.current_shape, x, y) 
				{
					winc::bit_blt(FRAM / 2 + TETRIS.x * BLOCK_SIZE + x * BLOCK_SIZE , FRAM/ 2 + TETRIS.y * BLOCK_SIZE + y * BLOCK_SIZE, BLOCK_SIZE);
				}
				if bit!(TETRIS.next_shape, x, y) 
				{
					winc::bit_blt(FRAM / 2 + 156 + x * BLOCK_SIZE, FRAM / 2 + 21 + y * BLOCK_SIZE, BLOCK_SIZE);
				}
			}
		}
	}
	for x in 0..X_MAX
	{
		for y in 0..Y_MAX
		{	
			unsafe
			{
				if TETRIS.ground[x as usize + 1][y as usize]
				{
					winc::bit_blt(FRAM / 2 + x * BLOCK_SIZE , FRAM / 2 + y * BLOCK_SIZE, BLOCK_SIZE);
				}
			}
		}
	}
}

pub fn new_game() 
{
	unsafe 
	{
		TETRIS.score = 0;
		winc::show_score(TETRIS.score);
		for y in 0..Y_MAX
		{
			for x in 0..X_MAX
			{	
				set_ground!(x + 1, y, false);
			}
		}
		for y in 0..Y_MAX
		{
			set_ground!(X_MAX + 1, y, true);
			set_ground!(0, y, true);
		}
		for x in 0..X_MAX
		{	
			set_ground!(x + 1, Y_MAX, true);
		}
		TETRIS.next_shape_type = (random::next_byte() % 7) as usize;
		TETRIS.next_shape = SHAPES[TETRIS.next_shape_type][1];
		winc::invalidate(ptr::null(), true);
		new_block();
	}
}

pub fn block_rotate()
{
	unsafe 
	{
		let rotated_variant = ((TETRIS.current_shape_variant as i32 + 1) % TETRIS.current_shape_variants_count) as usize;
		let rotated_shape = SHAPES[TETRIS.current_shape_type][rotated_variant + 1];
		if !check_collision(&Direction::None, rotated_shape)
		{
			TETRIS.current_shape_variant = rotated_variant;
			TETRIS.current_shape = rotated_shape;
			draw_current();
		}
	}
}

fn game_over()
{
	game::set_pause(game::Pause::True);
	winc::message_box(s!("You lose"), s!("Game Over"));
	new_game();
	game::set_pause(game::Pause::False);
}

fn remove_lines(size: usize, &lines_to_remove: &[usize; Y_MAX as usize + 1])
{
	let mut i = size;
	loop
	{
		i-=1;
		for x in 0..X_MAX
		{
			for y in (1..=lines_to_remove[i]).rev()
			{
				unsafe 
				{
					set_ground!(x + 1, y, if y > 0 { TETRIS.ground[x as usize + 1][y as usize - 1] } else { false });	
				}
			}
		}
		if i == 0
		{
			return;
		}
	}
}

fn check_line()
{
	let mut size = 0usize;
	let mut lines_to_remove = [0usize; Y_MAX as usize + 1];
	for y in (1..Y_MAX).rev() 
	{
		let mut line = true;
		for x in 0..X_MAX
		{
			unsafe
			{
				line = TETRIS.ground[x as usize + 1][y as usize] && line
			}
		}
		if line
		{
			lines_to_remove[size] = y as usize;
			size+=1;
		}
	}
	if size > 0
	{
		remove_lines(size, &lines_to_remove);
		winc::invalidate(ptr::null(), true);
		unsafe
		{
			TETRIS.score += (size * size * 10) as u32;
			winc::show_score(TETRIS.score);
		}
	}
	for x  in 0..8 as usize
	{
		for y in 1..(Y_MAX+1) as usize
		{
			unsafe
			{
				if (TETRIS.ground[x + 1][y] || y == Y_MAX as usize) && y < 2
				{
					game_over();
					return;
				}
			}
		}
	}
}

pub fn block_move(direction: Direction)
{
	unsafe 
	{
		if !check_collision(&direction, TETRIS.current_shape)
		{
			let rc = match direction
			{
				Direction::Left =>
				{
					TETRIS.x -= 1;
					RECT!
					(
						FRAM / 2 + TETRIS.x * BLOCK_SIZE,
						FRAM / 2 + TETRIS.y * BLOCK_SIZE,
						FRAM / 2 + (TETRIS.x + 5) * BLOCK_SIZE,
						FRAM / 2 + (TETRIS.y + 4) * BLOCK_SIZE
					)
				},
				Direction::Right =>
				{
					TETRIS.x += 1;
					RECT!
					(
						FRAM / 2 + (TETRIS.x - 1) * BLOCK_SIZE,
						FRAM / 2 + TETRIS.y * BLOCK_SIZE,
						FRAM / 2 + (TETRIS.x + 4) * BLOCK_SIZE,
						FRAM / 2 + (TETRIS.y + 4) * BLOCK_SIZE
					)
				},
				Direction::Down =>
				{
					TETRIS.y += 1;
					RECT!
					(
						FRAM / 2 + TETRIS.x * BLOCK_SIZE,
						FRAM / 2 + (TETRIS.y - 1) * BLOCK_SIZE,
						FRAM / 2 + (TETRIS.x + 4) * BLOCK_SIZE,
						FRAM / 2 + (TETRIS.y + 4) * BLOCK_SIZE
					)
				},				
				_ => { RECT!(0, 0, 0, 0) }
			};
			winc::invalidate(&rc, false);
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
							let ix = (TETRIS.x + 1 + x) as usize;
							let iy = (TETRIS.y + y) as usize;
							set_ground!(ix, iy,
								bit!(TETRIS.current_shape, x, y) |
								TETRIS.ground[ix][iy]);
						}
					}					
					check_line();
					new_block();
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

macro_rules! shape {
	($line0:tt $line1:tt $line2:tt $line3:tt) => {
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
