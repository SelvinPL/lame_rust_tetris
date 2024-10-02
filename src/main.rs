
#![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), no_std, no_main)]

use windows_sys::Win32::System::Threading::ExitProcess;

mod winmain;
mod game;
mod panic;
mod random;
mod tetris;
mod winc;
mod res;

#[no_mangle]
#[allow(non_snake_case)]
#[cfg(not(debug_assertions))]
fn mainCRTStartup()
{
	unsafe { ExitProcess(winmain::real_main() as u32); }
}

#[cfg(debug_assertions)]
fn main()
{
	unsafe { ExitProcess(winmain::real_main() as u32); }
}

#[cfg(not(debug_assertions))]
#[no_mangle]
unsafe extern "cdecl" fn memset(dest: isize, _:i32, _: usize) -> isize
{
	return dest;
}