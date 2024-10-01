#[cfg(panic = "abort")]
mod panic 
{
	use core::panic::PanicInfo;
	use core::ptr::null_mut;
	use windows_sys::core::{PCSTR, s};
	use windows_sys::Win32::System::Threading::ExitProcess;
	use windows_sys::Win32::UI::WindowsAndMessaging::*;

	#[panic_handler]
	fn panic_handler(_: &PanicInfo<'_>) -> ! 
	{
		const PANIC: PCSTR = s!("fatal error");
		unsafe 
		{
			MessageBoxA(null_mut(), PANIC, PANIC, MB_ICONERROR);
			ExitProcess(1);
		}
	}
}