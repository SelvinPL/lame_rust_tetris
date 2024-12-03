extern crate embed_resource;

fn main() 
{
    embed_resource::compile("assets/tetris.rc", embed_resource::NONE);
    #[cfg(target_env="gnu")]
    println!("cargo::rustc-link-arg=-nostartfiles");
}