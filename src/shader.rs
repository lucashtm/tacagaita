extern crate glfw;
extern crate gl;
use std::ffi::{CString};

pub struct Shader {
    id: gl::types::GLuint,
}

use crate::helpers::*;

impl Shader {

    fn id(&self) -> gl::types::GLuint {
        return self.id;
    }

    fn from_src(source: &str, kind: gl::types::GLuint) -> Result<Shader, std::ffi::NulError> {
        let id: u32 = unsafe { gl::CreateShader(kind) };
        let app_c_str = CString::new(source)?;

        unsafe {
            gl::ShaderSource(id, 1, &app_c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            Shader::check_compile_errors(id);
        }

        return Ok(Shader{id});
    }

    fn check_compile_errors(shader_id: u32) {
        let mut check_error = 0;
        unsafe { gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut check_error); }

        if check_error == 0 {
            println!("Compilation error");
            let mut error_length: i32 = 0;
            unsafe { gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut error_length); }
            let error_string = c_str_with_size(error_length as usize);

            unsafe {
                gl::GetShaderInfoLog(shader_id, error_length, std::ptr::null_mut(),
                    error_string.as_ptr() as *mut gl::types::GLchar);
            }

            println!("{:?}", error_string);
        }
    }

    pub fn from_vertex_src(source: &str) -> Result<Shader, std::ffi::NulError> {
        return Shader::from_src(source, gl::VERTEX_SHADER);
    }

    pub fn from_fragment_src(source: &str) -> Result<Shader, std::ffi::NulError> {
        return Shader::from_src(source, gl::FRAGMENT_SHADER);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}
pub struct GLProgram {
    id: gl::types::GLuint,
}

impl GLProgram {

    pub fn id(&self) -> gl::types::GLuint {
        return self.id;
    }

    pub fn activate(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn print_uniforms(&self) {
        let mut count : gl::types::GLint = 0;
        unsafe { gl::GetProgramiv(self.id, gl::ACTIVE_UNIFORMS, &mut count); }

        println!("Active Uniforms for program with id:{0}: {1:?}", self.id, count);

        let mut length : gl::types::GLsizei = 0;
        let mut size: gl::types::GLint = 0;
        let mut uniform_type: gl::types::GLenum = 0;
        let buf_size = 16; // largest name allowed in glsl
        let name = c_str_with_size(16);

        for i in 0..count {
            unsafe {
                gl::GetActiveUniform(
                    self.id, i as u32,
                    buf_size,
                    &mut length,
                    &mut size,
                    &mut uniform_type,
                    name.as_ptr() as *mut gl::types::GLchar
                );
            }
            println!("Uniform {0} Type: {1} Name: {2:?}\n", i, uniform_type, name);
        }
        println!("Finished printing the uniforms");
    }

    pub fn set_bool(&self, var: &str, value: bool) {
        let var_location = self.get_location(var).expect("erhn");
        unsafe { gl::Uniform1i(var_location, value as gl::types::GLint); }
    }

    pub fn set_float(&self, var: &str, value: f32) {
        let var_location = self.get_location(var).expect("erhn");
        unsafe { gl::Uniform1f(var_location, value as gl::types::GLfloat); }
    }

    pub fn set_int(&self, var: &str, value: i32) {
        let var_location = self.get_location(var).expect("erhn");
        unsafe { gl::Uniform1i(var_location, value as gl::types::GLint); }
    }

    // I seriously need to learn how to handle errors in rust.
    pub fn get_location(&self, var: &str) -> Result<gl::types::GLint, String> {
        let cstr_name = CString::new(var)
        .expect("Could not create variable");

        let var_location = unsafe { gl::GetUniformLocation(self.id, cstr_name.as_ptr()) };
        if var_location == -1 {
            println!("Error setting variable {:?}, not found in program.", var);
            return Err(String::new());
        }
        return Ok(var_location);
    }

    pub fn from_shaders(shaders: &[&Shader]) -> Result<GLProgram, bool> {
        let shader_program_id : u32 = unsafe { gl::CreateProgram() };
        println!("Creating shader program with id: {0}", shader_program_id);

        for shader in shaders {
            unsafe { gl::AttachShader(shader_program_id, shader.id()); }
        }

        unsafe { gl::LinkProgram(shader_program_id); }

        let has_errors = GLProgram::has_link_errors(shader_program_id);

        for shader in shaders {
            unsafe { gl::DetachShader(shader_program_id, shader.id()); }
        }

        // TODO: Get the link errors and return here.
        if has_errors {
            return Err(false);
        }

        return Ok(GLProgram{id: shader_program_id});
    }


    fn has_link_errors(program_id: u32) -> bool {
        let mut check_error = 0;
        unsafe {gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut check_error); }

        if check_error == 0 {
            println!("link errors");
            let mut error_length: i32 = 0;
            unsafe { gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut error_length); }
            let error_string = c_str_with_size(error_length as usize);

            unsafe {
                gl::GetShaderInfoLog(program_id, error_length, std::ptr::null_mut(),
                    error_string.as_ptr() as *mut gl::types::GLchar);
            }

            println!("{:?}", error_string);
            return true;
        }
        return false;
    }
}

impl Drop for GLProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}
