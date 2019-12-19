
#[macro_use]
extern crate static_assertions;

use {
    std::{ error::Error, ptr, ffi, slice, str },
    glutin::{
        event_loop::EventLoop,
        platform::unix::EventLoopWindowTargetExtUnix
    },
    gl::types::*,

    rust_game::*,
};

extern "system" fn on_gl_debug(
    _source:   GLenum,
    _type:     GLenum,
    _id:       GLuint,
    _severity: GLenum,
    length:    GLsizei,
    message:   *const GLchar,
    _user:     *mut ffi::c_void)
{
    let msg_slice = unsafe {
        slice::from_raw_parts(message as *const u8, length as usize)
    };

    eprintln!(
        "GL debug: {}",
        str::from_utf8(msg_slice)
                .unwrap_or("<error parsing debug message>")
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_q = EventLoop::new();
    eprintln!(
        "Running on {}",
        if event_q.is_wayland() { "Wayland" } else { "X11" }
    );

    let ctx = {
        let win_builder = glutin::window::WindowBuilder::new()
            .with_title("rust game :)");

        glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl_debug_flag(true)
            .with_depth_buffer(24)
        //  .with_double_buffer(Some(true))
            .with_vsync(true)
            .build_windowed(win_builder, &event_q)?
    };

    let (event_sender, event_receiver) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let ctx = unsafe {
            ctx.make_current()
               .expect("Error making GL context current")
        };

        gl::load_with(|sym| { ctx.get_proc_address(sym) as *const _ });

        {   let mut major: GLint = 0;
            let mut minor: GLint = 0;
            unsafe {
                gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
                gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
            }
            eprintln!("Using OpenGL {}.{} Core profile", major, minor);
        }

        unsafe {
            gl::DebugMessageControl(
                gl::DONT_CARE,
                gl::DONT_CARE,
                gl::DONT_CARE,
                0, ptr::null(),
                gl::TRUE
            );
            gl::DebugMessageCallback(on_gl_debug, ptr::null());
        }

        game::main_thread(&ctx, &event_receiver).unwrap();
    });

    event_q.run(move |event, _, flow| {
        use glutin::event_loop::ControlFlow::*;
        *flow = match event_sender.send(event) {
            Err(_) => Exit,
            Ok(_)  => Wait
        };
    });
}

