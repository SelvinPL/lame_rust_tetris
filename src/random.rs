
use core::ptr;
use windows_sys::core::{ PCSTR, s };
use windows_sys::Win32::Foundation::{ GetLastError, NTE_BAD_KEYSET };
use windows_sys::Win32::Security::Cryptography::
{
	CryptAcquireContextA, CryptReleaseContext, CryptGenRandom,
	PROV_RSA_FULL, CRYPT_NEWKEYSET
};

const CRYPTO_CONTAINER: PCSTR = s!("MyKeyContainer");
static mut HCRYPTPROV: usize = 0;

pub fn init() -> bool
{
	unsafe 
	{
		if CryptAcquireContextA(ptr::addr_of_mut!(HCRYPTPROV), CRYPTO_CONTAINER, ptr::null(), PROV_RSA_FULL, 0) == 0
		{
			if GetLastError() == NTE_BAD_KEYSET as u32
			{
				if CryptAcquireContextA(ptr::addr_of_mut!(HCRYPTPROV), CRYPTO_CONTAINER, ptr::null(), PROV_RSA_FULL, CRYPT_NEWKEYSET) == 0
				{
					return false;
				}
			}
			return false;
		}
		return true;
	}
}

pub fn release()
{
	unsafe
	{
		CryptReleaseContext(HCRYPTPROV, 0);
	}
}

pub fn next_byte() -> u8
{
	let mut rand = [0u8; 1]; 
	unsafe 
	{
		CryptGenRandom(HCRYPTPROV, 1, rand.as_mut_ptr());
	}
	return rand[0];
}