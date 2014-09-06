extern crate glfw;
extern crate nice_glfw;
extern crate gl;

use std::c_str::CString;
use glfw::Context;

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (window, _) = nice_glfw::WindowBuilder::new(&glfw)
        .try_modern_context_hints()
        .size(800, 600)
        .create().expect("Couldn't create window :(");

    window.make_current();

    gl::load_with(|p| glfw.get_proc_address(p));

    let version = gl::GetString(gl::VERSION);
    let cs = unsafe { CString::new(version as *const i8, false) };
    println!("OpenGL Version: {}", cs);
}
