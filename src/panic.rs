#[cfg(panic = "abort")]
mod panic {
	use core::panic::PanicInfo;
	use windows_sys::core::PCSTR;
	use windows_sys::Win32::System::Threading::ExitProcess;
	use windows_sys::Win32::UI::WindowsAndMessaging::*;

	#[panic_handler]
	fn panic_handler(_: &PanicInfo<'_>) -> ! {
		const PANIC: PCSTR = windows_sys::s!("fatal error");
		unsafe {
			MessageBoxA(0, PANIC, PANIC, MB_ICONERROR);
			ExitProcess(1);
		}
	}
}