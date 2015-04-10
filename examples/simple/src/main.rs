extern crate glfw;
extern crate nice_glfw;
extern crate gl;
extern crate libc;

use std::io::Write;
use glfw::Context;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, _) = nice_glfw::WindowBuilder::new(&mut glfw)
        .try_modern_context_hints()
        .size(800, 600)
        .create().expect("Couldn't create window :(");

    window.make_current();

    gl::load_with(|p| glfw.get_proc_address(p));

    let version = unsafe { gl::GetString(gl::VERSION) };
    print!("OpenGL Version: ");
    let _ = std::io::stdout().write(unsafe { std::slice::from_raw_parts(version, libc::strlen(version as *const _) as usize) });
}
