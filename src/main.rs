//#![no_std]
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
//fn mainCRTStartup()
fn main()
{
	unsafe { ExitProcess(winmain::real_main() as u32); }
}