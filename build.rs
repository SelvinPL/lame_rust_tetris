extern crate embed_resource;
#[path = "src/res.rs"]
mod res;

fn main() 
{
	embed_resource::compile("assets/tetris.rc", res::RESOURCES).manifest_required().unwrap();
	let profile = std::env::var("PROFILE").unwrap();
	let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
	let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();
	if target_env == "gnu" {
		println!("cargo::rustc-link-arg-bins=-Wl,--major-subsystem-version,5");
		println!("cargo::rustc-link-arg-bins=-Wl,--minor-subsystem-version,0");
	}
	if profile != "debug" {
		if target_env == "gnu" {
			println!("cargo::rustc-link-arg-bins=-Wl,--strip-all");
			println!("cargo::rustc-link-arg-bins=-Wl,--gc-sections");
			println!("cargo::rustc-link-arg-bins=-Wl,--build-id=none");
			println!("cargo::rustc-link-arg-bins=-Wl,--no-insert-timestamp");
			println!("cargo::rustc-link-arg-bins=-nostartfiles");
			println!("cargo::rustc-link-arg-bins=-nodefaultlibs");
			println!("cargo::rustc-link-arg-bins=-nostdlib");
			if target_arch == "x86_64" {
				println!("cargo::rustc-link-arg-bins=-Wl,--file-alignment=8");
				println!("cargo::rustc-link-arg-bins=-Wl,--section-alignment=8");
			} else {
			}
			println!("cargo::rustc-link-arg-bins=-Wl,-e,mainCRTStartup");
		} else {
			println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS,5.0");
			println!("cargo:rustc-link-arg-bins=/LTCG");
			println!("cargo:rustc-link-arg-bins=/OPT:REF");
			println!("cargo:rustc-link-arg-bins=/OPT:ICF");
			println!("cargo:rustc-link-arg-bins=/MERGE:.rdata=.text");
			println!("cargo:rustc-link-arg-bins=/MERGE:.pdata=.text");
			if target_arch == "x86_64" {
				println!("cargo:rustc-link-arg-bins=/ALIGN:16");
			} else {
				println!("cargo:rustc-link-arg-bins=/ALIGN:512");
			}
		}
	}
}