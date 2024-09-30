use super::winc;
use super::tetris;

use windows_sys::core::s;

use windows_sys::Win32::Foundation::HWND;

struct Game 
{
	timer_count: usize,
	pause: bool
}

static mut GAME: Game = Game { timer_count: 0, pause: false };

pub fn init(hwnd: HWND, lparam: isize)
{
	winc::init(hwnd, lparam);
	tetris::new_game();
	unsafe
	{
		GAME.timer_count = 1;
	}
}

pub fn paint()
{
	winc::start_painting();
}

pub fn destroy()
{
	winc::destroy();
}

pub unsafe fn timer_tick()
{
	unsafe
    {
		if !GAME.pause
		{
			GAME.timer_count+=1;
			if GAME.timer_count > 20
			{
				GAME.timer_count = 0;
				tetris::block_move(tetris::Direction::Down);
			}
		}
	}
}

pub fn up() 
{
	unsafe
	{
		if !GAME.pause
		{
			tetris::block_rotate();
		}
	}
}

pub fn down()
{
	unsafe
	{
		if !GAME.pause
		{
			tetris::block_move(tetris::Direction::Down);
		}
	}
}
	
pub fn left()
{
	unsafe
	{
		if !GAME.pause
		{
			tetris::block_move(tetris::Direction::Left);
		}
	}
}

pub fn right()
{
	unsafe
	{
		if !GAME.pause
		{
			tetris::block_move(tetris::Direction::Right);
		}
	}
}

pub fn set_pause(pause: Pause)  
{
	unsafe 
	{
		match pause
		{
			Pause::True => GAME.pause = true,
			Pause::False => GAME.pause = false,
			Pause::Toggle => GAME.pause = !GAME.pause,
		}
	}
}

pub fn help()
{
	unsafe
	{
		let paused = GAME.pause;
		GAME.pause = true;
		winc::message_box(s!("h - Help\ni - About\nspace, return - Pause\nn - New game"), s!("Help"));
		GAME.pause = paused;
	}
}

pub fn about()
{
	unsafe
	{
		let paused = GAME.pause;
		GAME.pause = true;
		winc::message_box(s!("\n\tTetris 2001-2024 Selvin\t\n"), s!("About..."));
		GAME.pause = paused;
	}
}

pub fn new_game()
{
	unsafe
	{
		tetris::new_game();
		GAME.pause = false;
	}
}

pub enum Pause
{
	True,
	False,
	Toggle
}