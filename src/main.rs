extern crate glfw;

use core::mem::{size_of, size_of_val};
use gl::*;
use glfw::{Action, Context, Key};

mod shader;
use shader::OpenglShader;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw
        .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_size_polling(true);
    window.make_current();

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    // the supplied function must be of the type:
    // `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    unsafe {
        gl::Hint(gl::MAJOR_VERSION, 4);
        gl::Hint(gl::MINOR_VERSION, 3);
        gl::Hint(gl::CONTEXT_CORE_PROFILE_BIT, gl::CONTEXT_CORE_PROFILE_BIT);
    }

    // loading a specific function pointer
    gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s));

    let rect_shader = OpenglShader::new("shader/test.vert", "shader/test.frag");

    let vertices: [[f32; 3]; 3] = [[-0.5, 0.5, 0.0], [0.5, 0.5, 0.0], [0.0, -0.5, 0.0]];
    let vertex_size: types::GLsizei = size_of::<[f32; 3]>().try_into().unwrap();

    let mut vbo = 0;
    let mut vao = 0;

    unsafe {
        GenBuffers(1, &mut vbo);
        GenVertexArrays(1, &mut vao);

        BindVertexArray(vao);
        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        VertexAttribPointer(0, 3, FLOAT, FALSE, vertex_size, 0 as *const _);
        EnableVertexAttribArray(0);

        ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    while !window.should_close() {
        glfw.poll_events();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(rect_shader.id);
            gl::BindVertexArray(vao);
            gl::DrawArrays(TRIANGLES, 0, 3);
        };

        window.swap_buffers();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Size(width, height) => handle_size_callback(window, width, height),
        _ => {}
    }
}

fn handle_size_callback(_window: &mut glfw::Window, width: i32, height: i32) {
    println!("Window resized to {}x{}", width, height);

    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
