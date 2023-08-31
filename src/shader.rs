use gl::*;
use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Read,
};

pub struct OpenglShader {
    pub id: u32,
}

fn read_file(path: &str) -> String {
    println!("Reading file: {}", path);
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    contents
}

fn read_file_to_c_string(path: &str) -> CString {
    CString::new(read_file(path)).expect("Failed to read file")
}

fn shader_comp_status(status: i32) -> &'static str {
    if status == 1 {
        "OK"
    } else {
        "FAIL"
    }
}

impl Drop for OpenglShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl OpenglShader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> OpenglShader {
        let mut shader = OpenglShader { id: 0 };
        shader.id = shader.create_shader(vertex_path, fragment_path);
        shader
    }

    fn compile_shader(code: &CStr, kind: gl::types::GLenum) -> u32 {
        unsafe {
            let id = CreateShader(kind);
            ShaderSource(id, 1, &code.as_ptr(), std::ptr::null());
            CompileShader(id);

            let mut success = -1;
            GetShaderiv(id, COMPILE_STATUS, &mut success);

            let status = shader_comp_status(success);
            println!("Shader compiled with status '{}'", status);

            if success == 0 {
                let mut info_log = vec![0; 512];
                GetShaderInfoLog(
                    id,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut gl::types::GLchar,
                );

                println!(
                    "Shader compile error: {}",
                    String::from_utf8(info_log).unwrap()
                );
            }

            id
        }
    }

    fn link_program(vertex: u32, frag: u32) -> u32 {
        unsafe {
            let id = CreateProgram();
            AttachShader(id, frag);
            AttachShader(id, vertex);

            DeleteShader(vertex);
            DeleteShader(frag);

            id
        }
    }

    fn create_shader(&self, vertex_path: &str, fragment_path: &str) -> u32 {
        println!("Compiling shader: {} {}", vertex_path, fragment_path);

        let vertex_code = read_file_to_c_string(vertex_path);
        let fragment_code = read_file_to_c_string(fragment_path);

        let vertex_shader = OpenglShader::compile_shader(&vertex_code, VERTEX_SHADER);
        let fragment_shader = OpenglShader::compile_shader(&fragment_code, FRAGMENT_SHADER);

        let shader_id = OpenglShader::link_program(vertex_shader, fragment_shader);

        shader_id
        // shader_program
    }
}
