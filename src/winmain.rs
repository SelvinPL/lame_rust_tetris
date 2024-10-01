use core::{ u32, u8 };
use core::ptr::{ null, null_mut };
use windows_sys::Win32::Graphics::Gdi::{ UpdateWindow, COLOR_WINDOW };
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{ VK_RIGHT, VK_LEFT, VK_UP, VK_DOWN, VK_SPACE, VK_RETURN, VK_H, VK_I, VK_N };
use windows_sys::Win32::UI::WindowsAndMessaging::
{
	CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, LoadAcceleratorsA, 
	LoadIconA, LoadStringA, RegisterClassA, ShowWindow, TranslateAcceleratorA, TranslateMessage, 
	WNDCLASSA, HACCEL, MSG,
	CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, SW_SHOWDEFAULT, WS_SYSMENU, WS_VISIBLE, 
	WM_CREATE, WM_DESTROY, WM_PAINT, WM_TIMER, WM_KEYDOWN, WM_KILLFOCUS
};
use windows_sys::Win32::Foundation::{ POINT, HWND, WPARAM, LPARAM, LRESULT };
use super::game::Game;
use super::random;
use super::res;

const MAX_LOADSTRING: usize = 256;

pub unsafe fn real_main() -> i32
{
	let hinstance = GetModuleHandleA(null());
	let mut msg = MSG{ hwnd: null_mut(), message: 0, wParam: 0, lParam: 0, time: 0, pt: POINT {x:0, y:0}};
	
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
		hCursor: null_mut(),
		hbrBackground : COLOR_WINDOW as *mut ::core::ffi::c_void,
		lpszMenuName: null(),
		lpszClassName: window_class,
		lpfnWndProc:  Some(wnd_proc),
		hIcon: LoadIconA(hinstance, res::IDI_TETRIS  as *const u8)
	};
	RegisterClassA(&mut wndclass);

	let hwnd = CreateWindowExA(0, window_class, title, WS_VISIBLE | WS_SYSMENU,
		CW_USEDEFAULT, CW_USEDEFAULT, 230, 330, null_mut(), null_mut(), hinstance, null());
	if hwnd == null_mut()
	{
		random::release();
		return -1;
	}
	ShowWindow(hwnd, SW_SHOWDEFAULT);
	UpdateWindow(hwnd);

	let hacceltable: HACCEL = LoadAcceleratorsA(hinstance, res::IDC_TETRIS as *const u8);
	while GetMessageA(&mut msg, null_mut(), 0, 0) != 0
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

static mut GAME: Game = Game::new();

unsafe extern "system" fn wnd_proc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT
{
	match message 
	{
		WM_CREATE =>
		{
			GAME.init(hwnd, lparam);
		},
		WM_PAINT =>
		{
			GAME.paint();
		},
		WM_TIMER => 
		{
			GAME.timer_tick();
		},
		WM_DESTROY => 
		{
			GAME.destroy();
		},
		WM_KEYDOWN =>
		{
			match wparam as u16
			{
				VK_UP =>
				{
					GAME.up();
				},
				VK_DOWN =>
				{
					GAME.down();
				},
				VK_LEFT =>
				{
					GAME.left();
				},
				VK_RIGHT =>
				{
					GAME.right();
				},
				VK_RETURN | VK_SPACE =>
				{
					GAME.toggle_pause();
				},
				VK_H =>
				{
					GAME.help();
				},
				VK_I =>
				{
					GAME.about();
				},
				VK_N =>
				{
					GAME.new_game();
				},
				_ => {}
			}
		},
		WM_KILLFOCUS =>
		{
			GAME.pause();
		},
		_ => return DefWindowProcA(hwnd, message, wparam, lparam),
	}
	return 0;
}