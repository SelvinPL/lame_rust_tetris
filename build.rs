extern crate embed_resource;

fn main() 
{
    // Compile and link checksums.rc
    embed_resource::compile("assets/tetris.rc", embed_resource::NONE);
    #[cfg(target_env="gnu")]
    println!("cargo::rustc-link-arg=-nostartfiles");
    // println!("cargo::rustc-link-arg=-nodefaultlibs");
    // println!("cargo::rustc-link-arg=-Wl,--exclude-libs=mingw32");
    // println!("cargo::rustc-link-arg=-Wl,--exclude-libs=libmingw32");
    // println!("cargo::rustc-link-arg=-Wl,--exclude-libs=libmsvcrt");
    // println!("cargo::rustc-link-arg=-Wl,--exclude-libs=libpthread.a");
    // println!("cargo::rustc-link-arg=-Wl,--exclude-libs=libgcc");
    // println!("cargo::rustc-link-arg=-Wl,--exclude-libs=libmingwex");
    // println!("cargo::rustc-link-arg=-Wl,--exclude-libs=libgcc_eh");
    // println!("cargo::rustc-link-arg=-Wl,--exclude-libs=libwindows.0.52.0");
    // println!("cargo::rustc-link-arg=--verbose");
    //"-lwindows.0.52.0" "-lgcc_eh" "-l:libpthread.a" "-lmsvcrt" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-lmingwex"
}