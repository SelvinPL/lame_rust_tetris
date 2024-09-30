use windows_sys::core::
{
	PCSTR, s
};
use windows_sys::Win32::Graphics::Gdi::{BitBlt, InvalidateRect, HBRUSH, HBITMAP, HDC, HGDIOBJ};
use windows_sys::Win32::UI::WindowsAndMessaging::
{
	CreateWindowExA, KillTimer, MessageBoxA, PostQuitMessage, SetTimer, SetWindowTextA, CREATESTRUCTA, WS_CHILD, WS_VISIBLE
};
use windows_sys::Win32::Foundation::{HINSTANCE, HWND, RECT};
use windows_sys::Win32::Graphics::Gdi::
{
	Rectangle, BeginPaint, EndPaint, SelectObject, CreateCompatibleDC, 
	DeleteDC, LoadBitmapA, CreateSolidBrush, DeleteObject,
	PAINTSTRUCT, SRCCOPY, NULL_PEN,
};
use windows_sys::Win32::System::SystemServices::SS_CENTER;
use core::ptr::{null, null_mut};
use core::{u32, u8};
use super::tetris;
use super::res;
struct  WindowController
{
	hwnd: HWND,
	current_hdc: HDC,
	bitmap_hdc: HDC,
	brush: HBRUSH,
	bitmap: HBITMAP,
	score_window: HWND,
	htimer: usize,
}

trait Empty<T>
{
	fn empty() -> T;
}

impl Empty<PAINTSTRUCT> for PAINTSTRUCT 
{
	fn empty() -> PAINTSTRUCT
	{
		return PAINTSTRUCT { hdc: null_mut(), fErase: 0, rcPaint: RECT::empty(), fRestore : 0, fIncUpdate:0, rgbReserved: [0; 32]  };
	}
}

impl Empty<RECT> for RECT 
{
	fn empty() -> RECT
	{
		return  RECT{ top:0, bottom:0, left: 0, right: 0 };
	}
}

macro_rules! RGB 
{
	($r:expr, $g:expr, $b:expr) => {
		$r|$g<<8|$b<<16
	}
}

const STATIC_CLASS: PCSTR = s!("static");
const EMPTY: PCSTR = s!("");
const TEXT_NEXT: PCSTR = s!("Next:");

static mut CONTROLER:  WindowController = default();

const fn default() -> WindowController
{ 
	return WindowController 
	{ 
		hwnd: null_mut(), 
		current_hdc: null_mut(), 
		bitmap_hdc: null_mut(), 
		brush: null_mut(), 
		bitmap: null_mut(), 
		score_window: null_mut(), 
		htimer: 0
	}
}

pub fn init(hwnd: HWND, lparam: isize)
{
	unsafe 
	{
		let create_struct = *(lparam as *const CREATESTRUCTA); 
		let hinstance: HINSTANCE = create_struct.hInstance;
		let dwstyle = WS_CHILD | WS_VISIBLE | SS_CENTER;
		let next_location = tetris::get_next_location_size();
		let score_location = tetris::get_score_location_size();
		CreateWindowExA(0, STATIC_CLASS,TEXT_NEXT, dwstyle, 
			next_location.x, 
			next_location.y,
			next_location.width,
			next_location.height,
			hwnd, null_mut(), hinstance, null());			
		CONTROLER = WindowController 
		{ 
			hwnd: hwnd,
			htimer: SetTimer(hwnd, 1, 0, None),
			current_hdc: null_mut(), 
			bitmap_hdc : null_mut(), 
			brush: CreateSolidBrush(RGB!(207, 217, 255)), 
			bitmap: LoadBitmapA(hinstance, res::IDB_RECT as *const u8), 
			score_window: CreateWindowExA(0, STATIC_CLASS,EMPTY, dwstyle, 
				score_location.x, 
				score_location.y,
				score_location.width,
				score_location.height,
				hwnd, null_mut(), hinstance, null())
		};
	}
}

pub fn message_box(message: PCSTR, caption: PCSTR)
{
	unsafe 
	{
		MessageBoxA(CONTROLER.hwnd, message, caption, 0);
	}
}

pub fn show_score(score_in: u32)
{
	const BASE: u32 = 10;
	const MAX: u32 = 256;
	const NUMBERS: PCSTR= s!("0123456789");
	const SCORE: PCSTR= s!("\nScore: \n\n");
	let mut score = score_in;
	let mut buffer: [u8; MAX as usize] = [0u8; MAX as usize];
	let mut i: usize = MAX as usize - 2;
	loop
	{
		unsafe 
		{
			buffer[i] = *NUMBERS.wrapping_add((score % BASE) as usize);
		}
		i-=1;
		score = score / BASE;
		if score == 0 || i == 0
		{
			break;
		}
	}
	let mut s: usize = 9;
	loop 
	{
		s-=1;
		unsafe 
		{			
			buffer[i] = *SCORE.wrapping_add(s);
		}
		i-=1;
		if s == 0 || i == 0
		{
			break;
		}
	}
	unsafe 
	{
		SetWindowTextA(CONTROLER.score_window, buffer[i + 1..].as_ptr());
	}
}

pub fn destroy()
{
	unsafe 
	{
		KillTimer(CONTROLER.hwnd, CONTROLER.htimer);
		DeleteObject(CONTROLER.bitmap);
		DeleteObject(CONTROLER.brush);
		PostQuitMessage(0);
	}
}

pub fn start_painting()
{
	let mut ps  = PAINTSTRUCT::empty();
	unsafe 
	{
		CONTROLER.current_hdc = BeginPaint(CONTROLER.hwnd, &mut ps);
		SelectObject(CONTROLER.current_hdc, CONTROLER.brush);
		SelectObject(CONTROLER.current_hdc, NULL_PEN as HGDIOBJ);
		Rectangle(CONTROLER.current_hdc, tetris::BOARD_RECT.left, tetris::BOARD_RECT.top, tetris::BOARD_RECT.right, tetris::BOARD_RECT.bottom); 
		CONTROLER.bitmap_hdc = CreateCompatibleDC(CONTROLER.current_hdc);
		SelectObject(CONTROLER.bitmap_hdc, CONTROLER.bitmap);
		tetris::draw();
		DeleteDC(CONTROLER.bitmap_hdc);
		CONTROLER.bitmap_hdc = null_mut();
		EndPaint(CONTROLER.hwnd, &mut ps);
	}
}

pub fn bit_blt(x: u32, y: u32, size: u32) {
	unsafe 
	{
		BitBlt(CONTROLER.current_hdc, x as i32, y as i32, size as i32, size as i32, CONTROLER.bitmap_hdc, 0, 0, SRCCOPY);
	}
}

pub fn invalidate(rc: *const RECT, erase: bool)
{
	unsafe
	{
		InvalidateRect(CONTROLER.hwnd, rc, if erase { 1 } else { 0 });
	}
}
