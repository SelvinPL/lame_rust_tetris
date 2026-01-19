use core::ptr::{ null , addr_of_mut};
use windows_sys::Win32::Security::Cryptography::
{
	CryptAcquireContextA, CryptReleaseContext, CryptGenRandom,
	PROV_RSA_FULL, CRYPT_VERIFYCONTEXT
};

pub struct Random
{
	hcryptprov: usize
}

impl Random
{
	pub const fn new() -> Self { Random { hcryptprov: 0} }

	pub fn init(&mut self) -> bool
	{
		unsafe 
		{
			if CryptAcquireContextA(addr_of_mut!(self.hcryptprov), null(), null(), PROV_RSA_FULL, CRYPT_VERIFYCONTEXT) == 0
			{
				return false;
			}
			return true;
		}
	}

	pub fn release(&self)
	{
		unsafe
		{
			CryptReleaseContext(self.hcryptprov, 0);
		}
	}

	pub fn next_byte(&self) -> u8
	{
		let mut rand = [0u8; 1]; 
		unsafe 
		{
			CryptGenRandom(self.hcryptprov, 1, rand.as_mut_ptr());
		}
		return rand[0];
	}
}