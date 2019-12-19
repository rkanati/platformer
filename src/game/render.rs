

mod shader_src {
    pub static SPRITE_V: &'static str = include_str!("../sprite-vert.glsl");
    pub static SPRITE_F: &'static str = include_str!("../sprite-frag.glsl");

    pub static LINE_V: &'static str = include_str!("../line-vert.glsl");
    pub static LINE_F: &'static str = include_str!("../line-frag.glsl");
}

// graphics buffer formats
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct Rect {
    left:   f32,
    bottom: f32,
    right:  f32,
    top:    f32,
}

static_assertions::assert_eq_size!(Rect, [f32; 4]);

impl Rect {
    fn new(left: f32, bottom: f32, right: f32, top: f32) -> Rect {
        Rect { left, bottom, right, top }
    }

    fn verts<'a>(&'a self) -> impl Iterator<Item = P2> + Clone + 'a {
        let mut index = 0;
        let next_vert = move || {
            let vert = match index {
                0 => P2::new(self.left,  self.bottom),
                1 => P2::new(self.right, self.bottom),
                2 => P2::new(self.right, self.top),
                3 => P2::new(self.left,  self.top),
                _ => { return None; }
            };
            index += 1;
            Some(vert)
        };
        std::iter::from_fn(next_vert)
    }
}

fn stroke(vs: impl Iterator<Item = P2> + Clone, r: u8, g: u8, b: u8, a: u8) -> Vec<LineVert> {
    let verts = vs
        .map(|v| LineVert { x: v.x, y: v.y, r, g, b, a })
        .cycle();

    let pairs: Vec<_> = verts.clone().zip(verts.skip(1))
        .map(|(a, b)| [a, b])
        .take(4)
        .collect();

    pairs.iter()
        .flat_map(|vs| vs)
        .map(|v| *v)
        .collect()
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct Sprite {
    rect:          Rect,
    texture_index: u32
}

static_assertions::assert_eq_size!(Sprite, [u32; 5]);

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct LineVert {
    x: f32, y: f32,
    r: u8, g: u8, b: u8, a: u8
}

static_assertions::assert_eq_size!(LineVert, [u32; 3]);




fn () {
    let sprite_prog = {
        let v_shader = shader::compile(shader::Stage::Vertex,   shader_src::SPRITE_V)?;
        let f_shader = shader::compile(shader::Stage::Fragment, shader_src::SPRITE_F)?;
        shader::link(&[v_shader, f_shader])?
    };

    let line_prog = {
        let v_shader = shader::compile(shader::Stage::Vertex,   shader_src::LINE_V)?;
        let f_shader = shader::compile(shader::Stage::Fragment, shader_src::LINE_F)?;
        shader::link(&[v_shader, f_shader])?
    };

    unsafe {
        gl::ClearColor(0.1, 0.0, 0.1, 1.0);
        gl::Disable(gl::CULL_FACE);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);
        gl::LineWidth(2.0);
    }

    let (sprite_buf, sprite_vao) = unsafe {
        let mut vao: GLuint = 0;
        gl::CreateVertexArrays(1, &mut vao);

        let mut sprite_buf: GLuint = 0;
        gl::CreateBuffers(1, &mut sprite_buf);
        gl::NamedBufferData(
            sprite_buf,
            mem::size_of::<Sprite>() as isize * 64 * 1024,
            0 as *const _,
            gl::DYNAMIC_DRAW
        );

        gl::EnableVertexArrayAttrib(vao, 0);
        gl::VertexArrayAttribFormat(vao, 0, 4, gl::FLOAT, gl::FALSE, 0);
        gl::VertexArrayAttribBinding(vao, 0, 0);

        gl::EnableVertexArrayAttrib(vao, 1);
        gl::VertexArrayAttribIFormat(vao, 1, 1, gl::UNSIGNED_INT, 16);
        gl::VertexArrayAttribBinding(vao, 1, 0);

        gl::VertexArrayVertexBuffer(
            vao, 0,
            sprite_buf, 0, mem::size_of::<Sprite>() as i32
        );
        gl::VertexArrayBindingDivisor(vao, 0, 1);

        (sprite_buf, vao)
    };

    let (line_buf, line_vao) = unsafe {
        let mut vao: GLuint = 0;
        gl::CreateVertexArrays(1, &mut vao);

        let mut buf: GLuint = 0;
        gl::CreateBuffers(1, &mut buf);
        gl::NamedBufferData(
            buf,
            mem::size_of::<LineVert>() as isize * 8 * 1024,
            0 as *const _,
            gl::DYNAMIC_DRAW
        );

        gl::EnableVertexArrayAttrib(vao, 0);
        gl::VertexArrayAttribFormat(vao, 0, 2, gl::FLOAT, gl::FALSE, 0);
        gl::VertexArrayAttribBinding(vao, 0, 0);

        gl::EnableVertexArrayAttrib(vao, 1);
        gl::VertexArrayAttribFormat(vao, 1, 4, gl::UNSIGNED_BYTE, gl::TRUE, 8);
        gl::VertexArrayAttribBinding(vao, 1, 0);

        gl::VertexArrayVertexBuffer(
            vao, 0,
            buf, 0, mem::size_of::<LineVert>() as i32
        );
    //  gl::VertexArrayBindingDivisor(vao, 0, 0);

        (buf, vao)
    };

    let mut sprites:    Vec<(GLuint, Sprite)> = Vec::new();
    let mut sprites_temp: Vec<Sprite> = Vec::new();
    let mut tex_tracker: Vec<(GLuint, usize)> = Vec::new();

    let mut lines:      Vec<LineVert> = Vec::new();
}

fn begin_draw() {

        // draw
        sprites.clear();
        lines.clear();
}

fn end_draw() {

        unsafe {
            gl::Viewport(0, 0, screen_dims.x as i32, screen_dims.y as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        if !sprites.is_empty() {
            sprites.sort_unstable_by_key(|(tex, _)| *tex);

            sprites_temp.clear();
            sprites_temp.reserve(sprites.len());
            tex_tracker.clear();

            sprites.iter().for_each(|(texture, sprite)| {
                sprites_temp.push(*sprite);
                match tex_tracker.last() {
                    Some((prev_texture, count)) if prev_texture == texture => {
                        *tex_tracker.last_mut().unwrap() = (*texture, count + 1);
                    }
                    _ => {
                        tex_tracker.push((*texture, 1));
                    }
                }
            });

            unsafe {
                gl::NamedBufferSubData(
                    sprite_buf,
                    0,
                    (sprites_temp.len () * mem::size_of::<Sprite>()) as isize,
                    (&sprites_temp).as_ptr() as *const _
                );

                sprite_prog.bind();
                gl::Uniform2f(
                    1,
                    2.0 / (camera.scale * screen_dims.x),
                    2.0 / (camera.scale * screen_dims.y),
                );
                gl::Uniform2f(2, -camera.centre.x, -camera.centre.y);

                gl::BindVertexArray(sprite_vao);
            }

            let mut base = 0;
            for (texture, count) in tex_tracker.iter() {
                unsafe {
                    gl::BindTextureUnit(0, *texture);
                    gl::DrawArraysInstancedBaseInstance(
                        gl::TRIANGLE_FAN,
                        0, 4,
                        *count as GLsizei,
                        base   as GLuint
                    );
                }
                base += count;
            }
        }

        if !lines.is_empty() {
            unsafe {
                gl::NamedBufferSubData(
                    line_buf,
                    0,
                    (lines.len () * mem::size_of::<LineVert>()) as isize,
                    (&lines).as_ptr() as *const _
                );

                line_prog.bind();
                gl::Uniform2f(
                    1,
                    2.0 / (camera.scale * screen_dims.x),
                    2.0 / (camera.scale * screen_dims.y),
                );
                gl::Uniform2f(2, -camera.centre.x, -camera.centre.y);

                gl::BindVertexArray(line_vao);
                gl::DrawArrays(gl::LINES, 0, lines.len() as i32);
            }
        }

        // flip
        ctx.swap_buffers()?;
}

