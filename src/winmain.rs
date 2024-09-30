use super::game;
use super::random;
use super::res;

use core::{u32, u8, ptr};
use windows_sys::Win32::Graphics::Gdi::{UpdateWindow, COLOR_WINDOW};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::UI::WindowsAndMessaging::
{
	CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, LoadAcceleratorsA, 
	LoadIconA, LoadStringA, RegisterClassA, ShowWindow, TranslateAcceleratorA, TranslateMessage, 
	WNDCLASSA, HACCEL, MSG,
	CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, SW_SHOWDEFAULT, WS_SYSMENU, WS_VISIBLE, 
	WM_CREATE, WM_DESTROY, WM_PAINT, WM_TIMER, WM_KEYDOWN, WM_KILLFOCUS
};
use windows_sys::Win32::Foundation::
{
	POINT, HWND, WPARAM, LPARAM, LRESULT
};

const MAX_LOADSTRING: usize = 256;

pub unsafe fn real_main() -> i32
{
	let hinstance = GetModuleHandleA(ptr::null());
	let mut msg = MSG{ hwnd: 0, message: 0, wParam: 0, lParam: 0, time: 0, pt: POINT {x:0, y:0}};
	
	let title = [0u8; MAX_LOADSTRING].as_mut_ptr();
	let window_class = [0u8; MAX_LOADSTRING].as_mut_ptr(); 
	LoadStringA(hinstance, res::IDS_APP_TITLE, title, MAX_LOADSTRING as i32);
	LoadStringA(hinstance, res::IDC_TETRIS, window_class, MAX_LOADSTRING as i32);

	if !random::init()
	{
		return -1;
	}

	let mut wndclass =  WNDCLASSA
	{
		style:  CS_HREDRAW | CS_VREDRAW,
		cbClsExtra : 0,
		cbWndExtra : 0,
		hInstance : hinstance,
		hCursor: 0,
		hbrBackground : COLOR_WINDOW as isize,
		lpszMenuName: ptr::null(),
		lpszClassName: window_class,
		lpfnWndProc:  Some(wnd_proc),
		hIcon: LoadIconA(hinstance, res::IDI_TETRIS  as *const u8)
	};
	RegisterClassA(&mut wndclass);

	let hwnd = CreateWindowExA(0, window_class, title, WS_VISIBLE | WS_SYSMENU,
		CW_USEDEFAULT, CW_USEDEFAULT, 230, 330, 0, 0, hinstance, ptr::null());
	if hwnd == 0
	{
		random::release();
		return -1;
	}
	ShowWindow(hwnd, SW_SHOWDEFAULT);
	UpdateWindow(hwnd);

	let hacceltable: HACCEL = LoadAcceleratorsA(hinstance, res::IDC_TETRIS as *const u8);
	while GetMessageA(&mut msg, 0, 0, 0) != 0
	{
		if TranslateAcceleratorA(msg.hwnd, hacceltable, &msg) == 0
		{
			TranslateMessage(&mut msg);
			DispatchMessageA(&mut msg);
		}
	}
	random::release();
	return 0;
}


unsafe extern "system" fn wnd_proc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT
{
	match message 
	{
		WM_CREATE => 
		{
			game::init(hwnd, lparam);
		},
		WM_PAINT => 
		{
			game::paint();
		},
		WM_TIMER => 
		{
			game::timer_tick();
		},
		WM_DESTROY => 
		{
			game::destroy();
		},
		WM_KEYDOWN =>
		{
			match wparam as u16
			{
				VK_UP =>
				{
					game::up();
				},
				VK_DOWN =>
				{
					game::down();
				},
				VK_LEFT =>
				{
					game::left();
				},
				VK_RIGHT =>
				{
					game::right();
				},
				VK_RETURN | VK_SPACE =>
				{
					game::set_pause(game::Pause::Toggle);
				},
				VK_H =>
				{
					game::help();
				},
				VK_I =>
				{
					game::about();
				},
				VK_N =>
				{
					game::new_game();
				},
				_ => {}
			}
		},
		WM_KILLFOCUS =>
		{
			game::set_pause(game::Pause::True);
		},
		_ => return DefWindowProcA(hwnd, message, wparam, lparam),
	}
	return 0;
}