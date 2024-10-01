use super::tetris::{ Tetris, Direction };
use super::winc::WindowController;
use super::pause_control::PauseControl;

use windows_sys::core::s;

use windows_sys::Win32::Foundation::HWND;

pub struct Game 
{
	timer_count: usize,
	tetris: Tetris,
	windows: WindowController,
	pause_control: PauseControl
}

impl Game
{
	pub const fn new() -> Self 
	{
		Game
		{ 
			timer_count: 0,
			tetris: Tetris::new(),
			pause_control: PauseControl::new(),
			windows: WindowController::empty()
		}
	}

	pub fn init(&mut self, hwnd: HWND, lparam: isize)
	{
		self.windows = WindowController::new(hwnd, lparam);
		self.timer_count = 1;
		self.tetris.new_game(&self.windows);
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
		if self.pause_control.is_not_paused()
		{
			self.timer_count+=1;
			if self.timer_count > 20
			{
				self.timer_count = 0;
				self.tetris.block_move(Direction::Down, &mut self.pause_control, &self.windows);
			}
		}
	}

	pub fn up(&mut self) 
	{
		if self.pause_control.is_not_paused()
		{
			self.tetris.block_rotate(&self.windows);
		}
	}

	pub fn down(&mut self)
	{
		if self.pause_control.is_not_paused()
		{
			self.tetris.block_move(Direction::Down, &mut self.pause_control, &self.windows);
		}
	}
		
	pub fn left(&mut self)
	{
		if self.pause_control.is_not_paused()
		{
			self.tetris.block_move(Direction::Left, &mut self.pause_control, &self.windows);
		}
	}

	pub fn right(&mut self)
	{
		if self.pause_control.is_not_paused()
		{
			self.tetris.block_move(Direction::Right, &mut self.pause_control, &self.windows);
		}
	}

	pub fn pause(&mut self)
	{
		self.pause_control.pause();
	}

	pub fn toggle_pause(&mut self)
	{
		self.pause_control.toggle();
	}

	pub fn help(&mut self)
	{
		let paused = self.pause_control.is_paused();
		self.pause_control.pause();
		self.windows.message_box(s!("h - Help\ni - About\nspace, return - Pause\nn - New game"), s!("Help"));
		if !paused
		{
			self.pause_control.unpause();
		}
	}

	pub fn about(&mut self)
	{
		let paused = self.pause_control.is_paused();
		self.pause_control.pause();
		self.windows.message_box(s!("\n\tTetris 2001-2024 Selvin\t\n"), s!("About..."));
		if !paused
		{
			self.pause_control.unpause();
		}
	}

	pub fn new_game(&mut self)
	{
		self.tetris.new_game(&self.windows);
		self.pause_control.unpause();
	}
}