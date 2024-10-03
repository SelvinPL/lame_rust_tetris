use core::{ u32, u8 };
use core::ptr::{ null, null_mut };
use windows_sys::Win32::Graphics::Gdi::{ UpdateWindow, COLOR_WINDOW };
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleA;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{ VK_RIGHT, VK_LEFT, VK_UP, VK_DOWN, VK_SPACE, VK_RETURN, VK_H, VK_I, VK_N };
use windows_sys::Win32::UI::WindowsAndMessaging::
{
	CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, GetWindowLongPtrA, LoadAcceleratorsA, LoadIconA, LoadStringA, 
	RegisterClassA, SetWindowLongPtrA, ShowWindow, TranslateAcceleratorA, TranslateMessage, 
	CREATESTRUCTA, HACCEL, MSG, WNDCLASSA, 
	CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, SW_SHOWDEFAULT, WM_CREATE, WM_DESTROY, WM_KEYDOWN, WM_KILLFOCUS, WM_PAINT, 
	WM_TIMER, WS_SYSMENU, WS_VISIBLE, WS_MINIMIZEBOX
};
use windows_sys::Win32::Foundation::{ POINT, HWND, WPARAM, LPARAM, LRESULT };
use super::game::Game;
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
	let mut game = Game::new();

	if !game.random_init()
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
	let game_ptr = &game as *const Game;
	let hwnd = CreateWindowExA(0, window_class, title, WS_VISIBLE | WS_SYSMENU | WS_MINIMIZEBOX,
		CW_USEDEFAULT, CW_USEDEFAULT, 230, 330, null_mut(), null_mut(), hinstance,  game_ptr as *const core::ffi::c_void);
	if hwnd == null_mut()
	{
		game.random_release();
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
	game.random_release();
	return 0;
}

#[macro_export]
macro_rules! get_game 
{ 
	($hwnd:expr) =>
	{
		{
			let game_ptr = GetWindowLongPtrA($hwnd, GWLP_USERDATA);
			if game_ptr == 0 { panic!() } else { &mut *(game_ptr as *mut Game) }
		}
	}
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT
{
	match message 
	{
		WM_CREATE =>
		{
			let create_struct = *(lparam as *const CREATESTRUCTA);
			let game = &mut *(create_struct.lpCreateParams as *mut Game);
			game.init(hwnd, create_struct.hInstance);
			SetWindowLongPtrA(hwnd, GWLP_USERDATA, create_struct.lpCreateParams as isize);
		},
		WM_PAINT =>
		{
			get_game!(hwnd).paint();
		},
		WM_TIMER => 
		{
			get_game!(hwnd).timer_tick();
		},
		WM_DESTROY => 
		{
			get_game!(hwnd).destroy();
		},
		WM_KEYDOWN =>
		{
			let game = get_game!(hwnd);
			match wparam as u16
			{
				VK_UP =>
				{
					game.up();
				},
				VK_DOWN =>
				{
					game.down();
				},
				VK_LEFT =>
				{
					game.left();
				},
				VK_RIGHT =>
				{
					game.right();
				},
				VK_RETURN | VK_SPACE =>
				{
					game.toggle_pause();
				},
				VK_H =>
				{
					game.help();
				},
				VK_I =>
				{
					game.about();
				},
				VK_N =>
				{
					game.new_game();
				},
				_ => {}
			}
		},
		WM_KILLFOCUS =>
		{
			get_game!(hwnd).pause();
		},
		_ => return DefWindowProcA(hwnd, message, wparam, lparam),
	}
	return 0;
}