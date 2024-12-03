extern crate embed_resource;
#[path = "src/res.rs"]
mod res;

fn main() 
{
    embed_resource::compile("assets/tetris.rc", res::RESOURCES).manifest_required().unwrap();
    #[cfg(target_env="gnu")]
    println!("cargo::rustc-link-arg=-nostartfiles");
}