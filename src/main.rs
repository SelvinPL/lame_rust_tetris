#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

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
fn mainCRTStartup()
{
	unsafe { ExitProcess(winmain::real_main() as u32); }
}

#[no_mangle]
unsafe extern "cdecl" fn memset(dest: isize, _:i32, _: usize) -> isize
{
	return dest;
}