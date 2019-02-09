
extern crate glutin;
extern crate gl;

use {
  std::{ error::Error, vec::Vec, ptr, mem, ffi, slice, str },
  glutin::{ GlContext, os::unix::EventsLoopExt },
  gl::types::*,

  rust_game::{ *, gfx::*, unit::Unit },
};

static V_SHADER_SRC: &'static str = "\
#version 450

in vec2 attr_coords;

void main() {
  gl_Position = vec4(attr_coords, -0.5, 1.0);
}
";

static F_SHADER_SRC: &'static str = "\
#version 450

out vec4 frag;

void main() {
  frag = vec4(1,1,1,1);
}
";

#[repr(C)]
struct Vert(f32, f32);

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
  let vulns = damage::Scales {
    kinetic:   0.9,
    thermal:   0.6,
    explosive: 1.0,
  };

  let mut u = Unit::Alive {
    name: "test".to_string(),
    hp: 100,
    vulns,
  };

  let dmg = damage::Values {
    kinetic:   45,
    thermal:   0,
    explosive: 0,
  };
  println!("{:?}", dmg);

  loop {
    u = u.deal_damage(dmg);
    println!("{:?}", u);
    if let Unit::Dead = u { break; }
  }

  let mut event_q = glutin::EventsLoop::new();
  eprintln!(
    "Running on {}",
    if event_q.is_wayland() { "Wayland" } else { "X11" }
  );

  let win = glutin::WindowBuilder::new()
    .with_title("rust game :)");

  let ctx = glutin::ContextBuilder::new()
    .with_gl(glutin::GlRequest::Latest)
    .with_gl_profile(glutin::GlProfile::Core)
    .with_gl_debug_flag(true)
    .with_depth_buffer(24)
  //.with_double_buffer(Some(true))
    .with_vsync(true)
    ;

  let gl_win = glutin::GlWindow::new(win, ctx, &event_q)?;

  gl::load_with(|sym| { gl_win.get_proc_address(sym) as *const _ });

  unsafe {
    gl_win.make_current()?;
  }

  { let mut major: GLint = 0;
    let mut minor: GLint = 0;
    unsafe {
      gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
      gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
    }
    eprintln!("Using OpenGL {}.{} Core profile", major, minor);
  }

  unsafe {
    gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, ptr::null(), gl::TRUE);
    gl::DebugMessageCallback(on_gl_debug, ptr::null());
  }

  let v_shader = shader::compile(shader::Stage::Vertex,   V_SHADER_SRC)?;
  let f_shader = shader::compile(shader::Stage::Fragment, F_SHADER_SRC)?;
  let prog = shader::link(&[v_shader, f_shader])?;

  unsafe {
    prog.bind();
    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    gl::Disable(gl::CULL_FACE);
  }

  let verts: [Vert; 3] = [
    Vert( 0.0,  0.3),
    Vert(-0.2, -0.1),
    Vert( 0.2, -0.1),
  ];

  unsafe {
    let mut vao: GLuint = 0;
    gl::CreateVertexArrays(1, &mut vao);

    let mut vert_buf: GLuint = 0;
    gl::CreateBuffers(1, &mut vert_buf);
    gl::NamedBufferData(
      vert_buf,
      mem::size_of_val(&verts) as isize,
      verts.as_ptr() as *const _,
      gl::STATIC_DRAW
    );

    gl::EnableVertexArrayAttrib(vao, 0);
    gl::VertexArrayAttribFormat(vao, 0, 2, gl::FLOAT, gl::FALSE, 0);
    gl::VertexArrayAttribBinding(vao, 0, 0);
    gl::VertexArrayVertexBuffer(
      vao, 0,
      vert_buf, 0, mem::size_of::<Vert>() as i32
    );

    gl::BindVertexArray(vao);
  }

  let mut events: Vec<glutin::Event> = Vec::new();

  'main_loop: loop {
    // event processing
    events.clear();
    event_q.poll_events(|e| { events.push(e) });
    for e in &events {
      match e {
        glutin::Event::WindowEvent { event, .. } => match event {
          glutin::WindowEvent::CloseRequested => break 'main_loop,
          glutin::WindowEvent::Resized(new_size) => {
            let phys = new_size.to_physical(1.0);
            gl_win.resize(phys);
            let (w, h): (u32, u32) = phys.into();
            unsafe { gl::Viewport(0, 0, w as i32, h as i32); }
          },
          _ => (),
        },
        _ => (),
      }
    }

    unsafe {
      gl::Clear(gl::COLOR_BUFFER_BIT);
      gl::DrawArrays(gl::TRIANGLES, 0, verts.len() as i32);
    }

    // flip
    gl_win.swap_buffers()?;
  }

  Ok(())
}

