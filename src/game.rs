
use windows_sys::core::s;
use windows_sys::Win32::Foundation::{HINSTANCE, HWND};
use super::tetris::{ Tetris, Direction };
use super::winc::WindowController;
use super::random::Random;

pub struct Game 
{
	timer_count: usize,
	tetris: Tetris,
	windows: WindowController,
	random: Random
}

impl Game
{
	pub const fn new() -> Self 
	{
		Game
		{ 
			timer_count: 0,
			tetris: Tetris::new(),
			windows: WindowController::empty(),
			random: Random::new()
		}
	}

	pub fn init(&mut self, hwnd: HWND, histance: HINSTANCE)
	{
		self.windows = WindowController::new(hwnd, histance);
		self.timer_count = 1;
		self.tetris.new_game(&self.windows, &self.random);
	}

	pub fn paint(&mut self)
	{
		self.windows.start_painting(&mut self.tetris);
	}

	pub fn destroy(&self)
	{
		self.windows.destroy();
	}

	pub fn timer_tick(&mut self)
	{
		self.timer_count+=1;
		if self.timer_count > 20
		{
			self.timer_count = 0;
			self.tetris.block_move(Direction::Down, &self.windows, &self.random);
		}
	}

	pub fn up(&mut self) 
	{
		self.tetris.block_rotate(&self.windows);
	}

	pub fn down(&mut self)
	{
		self.tetris.block_move(Direction::Down, &self.windows, &self.random);
	}
		
	pub fn left(&mut self)
	{
		self.tetris.block_move(Direction::Left, &self.windows, &self.random);
	}

	pub fn right(&mut self)
	{
		self.tetris.block_move(Direction::Right, &self.windows, &self.random);
	}

	pub fn pause(&mut self)
	{
		self.tetris.pause();
	}

	pub fn toggle_pause(&mut self)
	{
		self.tetris.toggle();
	}

	pub fn help(&mut self)
	{
		let paused = self.tetris.is_paused();
		self.tetris.pause();
		self.windows.message_box(s!("h - Help\ni - About\nspace, return - Pause\nn - New game"), s!("Help"));
		if !paused
		{
			self.tetris.unpause();
		}
	}

	pub fn about(&mut self)
	{
		let paused = self.tetris.is_paused();
		self.tetris.pause();
		self.windows.message_box(s!("\n\tTetris 2001-2024 Selvin\t\n"), s!("About..."));
		if !paused
		{
			self.tetris.unpause();
		}
	}

	pub fn new_game(&mut self)
	{
		self.tetris.new_game(&self.windows, &self.random);
	}

	pub fn random_init(&mut self) -> bool
	{
		return self.random.init();
	}

	pub fn random_release(&mut self)
	{
		return self.random.release();
	}
}