
#![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), no_std, no_main)]

mod winmain;
mod game;
mod panic;
mod random;
mod tetris;
mod winc;
mod res;

use windows_sys::Win32::System::Threading::ExitProcess;
use winmain::real_main;
#[cfg(not(debug_assertions))]
use core::ptr;

#[no_mangle]
#[allow(non_snake_case)]
#[cfg(not(debug_assertions))]
fn mainCRTStartup()
{
	unsafe { ExitProcess(real_main() as u32); }
}

#[cfg(debug_assertions)]
fn main()
{
	unsafe { ExitProcess(real_main() as u32); }
}

#[cfg(not(debug_assertions))]
#[no_mangle]
unsafe extern "C" fn memset(s: *mut u8, c: u8, n: usize) 
{
	let s = ptr::read_volatile(&s);
	let c = ptr::read_volatile(&c);
	let n = ptr::read_volatile(&n);

	for i in 0..n {
		ptr::write(s.add(i), c);
	}

	let _ = ptr::read_volatile(&s);
}