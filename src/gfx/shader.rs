extern crate gl;

use {
    std::error::Error,
    std::vec::Vec,
    std::ptr,
    std::str,
    std::fmt,

    gl::types::*,
};

pub enum Stage { Vertex, Fragment }

#[derive(Debug)]
pub struct Unit { handle: GLuint }

#[derive(Debug)]
pub struct CompileError(Unit);

impl Drop for Unit {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handle); }
    }
}

pub fn compile(stage: Stage, src: &str)
    -> Result<Unit, CompileError>
{
    let gl_stage = match stage {
        Stage::Vertex   => gl::VERTEX_SHADER,
        Stage::Fragment => gl::FRAGMENT_SHADER,
    };

    let handle = unsafe { gl::CreateShader(gl_stage) };

    let src_ptr = src.as_ptr() as *const i8;
    let src_len = src.len() as GLint;

    unsafe {
        gl::ShaderSource(
            handle,
            1,
            &src_ptr as *const *const i8,
            &src_len as *const GLint
        );

        gl::CompileShader(handle);
    }

    let ok = unsafe {
        let mut status: GLint = 0;
        gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut status);
        status != (gl::FALSE as i32)
    };

    let shader = Unit { handle };
    if ok {
        Ok(shader)
    }
    else {
        Err(CompileError(shader))
    }
}

impl Unit {
    fn info_log(&self) -> String {
        let mut log_length: GLint = 0;
        unsafe {
            gl::GetShaderiv(self.handle, gl::INFO_LOG_LENGTH, &mut log_length);
        }

        if log_length <= 0 {
            return "<no info log>".to_string();
        }

        let mut log_buffer: Vec<u8> = Vec::new();
        log_buffer.resize(log_length as usize, 0);
        unsafe {
            gl::GetShaderInfoLog(
                self.handle,
                log_length,
                ptr::null_mut(),
                log_buffer.as_mut_ptr() as *mut i8
            );
        }

        String::from_utf8_lossy(&log_buffer).into()
    }
}

impl CompileError {
    fn info_log(&self) -> String {
        let CompileError(shader) = self;
        shader.info_log()
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.info_log().fmt(f)
    }
}

impl Error for CompileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct Program { handle: GLuint }

#[derive(Debug)]
pub struct LinkError(Program);

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.handle); }
    }
}

pub fn link(stages: &[Unit])
    -> Result<Program, LinkError>
{
    let handle = unsafe { gl::CreateProgram() };

    unsafe {
        for stage in stages {
            gl::AttachShader(handle, stage.handle);
        }

        gl::LinkProgram(handle);
    }

    let ok = unsafe {
        let mut status: GLint = 0;
        gl::GetProgramiv(handle, gl::LINK_STATUS, &mut status);
        status != (gl::FALSE as i32)
    };

    let program = Program { handle };
    if ok {
        Ok(program)
    }
    else {
        Err(LinkError(program))
    }
}

impl Program {
    pub fn info_log(&self) -> String {
        let mut log_length: GLint = 0;
        unsafe {
            gl::GetProgramiv(self.handle, gl::INFO_LOG_LENGTH, &mut log_length);
        }

        if log_length <= 0 {
            return "<no info log>".to_string();
        }

        let mut log_buffer: Vec<u8> = Vec::new();
        log_buffer.resize(log_length as usize, 0);
        unsafe {
            gl::GetProgramInfoLog(
                self.handle,
                log_length,
                ptr::null_mut(),
                log_buffer.as_mut_ptr() as *mut i8
            );
        }

        String::from_utf8_lossy(&log_buffer).into()
    }

    pub unsafe fn bind(&self) {
        gl::UseProgram(self.handle);
    }
}

impl LinkError {
    fn info_log(&self) -> String {
        let LinkError(program) = self;
        program.info_log()
    }
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.info_log().fmt(f)
    }
}

impl Error for LinkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

