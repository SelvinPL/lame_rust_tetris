#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

mod winmain;
mod game;
mod panic;
mod random;
mod tetris;
mod winc;
mod res;

#[no_mangle]
#[allow(non_snake_case)]
unsafe fn mainCRTStartup()
{
	use windows_sys::Win32::System::Threading::ExitProcess;
	ExitProcess(winmain::real_main() as u32);
}