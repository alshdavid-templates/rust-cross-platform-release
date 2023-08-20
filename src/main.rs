use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

fn main() {
    println!("Hello, world from {}! I was compiled on {}.", CURRENT_PLATFORM, COMPILED_ON);
}
