
pub mod shader;

use {
    std::error::Error,
    gl::types::*,
    image::GenericImageView,
};

pub fn load_texture(path: impl AsRef<std::path::Path>) -> Result<GLuint, Box<dyn Error>> {
    let im = image::open(path)?.to_rgba();
    let width = im.width() as i32;
    let height = im.height() as i32;

    unsafe {
        let mut tex: GLuint = 0;
        gl::CreateTextures(gl::TEXTURE_2D, 1, &mut tex);

        let params: [(GLenum, GLuint); 5] = [
            (gl::TEXTURE_MIN_FILTER, gl::NEAREST),
            (gl::TEXTURE_MAG_FILTER, gl::NEAREST),
            (gl::TEXTURE_MAX_LEVEL,  0),
            (gl::TEXTURE_WRAP_S,     gl::CLAMP_TO_EDGE),
            (gl::TEXTURE_WRAP_T,     gl::CLAMP_TO_EDGE),
        ];
        for (pname, val) in &params {
            gl::TextureParameteriv(tex, *pname, &(*val as i32));
        }

        gl::TextureStorage2D(tex, 1, gl::RGBA8, width, height);
        gl::TextureSubImage2D(
            tex, 0,
            0, 0, width, height,
            gl::RGBA, gl::UNSIGNED_BYTE, im.into_raw().as_ptr() as *const std::ffi::c_void
        );

        Ok(tex)
    }
}

pub fn load_atlas_texture(
    path: impl AsRef<std::path::Path>,
    tile_width:  i32,
    tile_height: i32,
//  base_index:  u32,
)
    -> Result<GLuint, Box<dyn Error>>
{
    assert!(tile_width > 1 && tile_height > 1, "Invalid parameters");

    eprintln!("loading {}", path.as_ref().display());
    let im = image::open(path)?.to_rgba();
    let width  = im.width()  as i32;
    let height = im.height() as i32;

    if width % tile_width != 0 || height % tile_height != 0 {
        // warn about margin?
    }

    let columns = width  / tile_width;
    let rows    = height / tile_height;
    let tile_count = columns * rows;

    unsafe {
        let mut tex: GLuint = 0;
        gl::CreateTextures(gl::TEXTURE_2D_ARRAY, 1, &mut tex);

        let params: [(GLenum, GLuint); 5] = [
            (gl::TEXTURE_MIN_FILTER, gl::NEAREST),
            (gl::TEXTURE_MAG_FILTER, gl::NEAREST),
            (gl::TEXTURE_MAX_LEVEL,  0),
            (gl::TEXTURE_WRAP_S,     gl::CLAMP_TO_EDGE),
            (gl::TEXTURE_WRAP_T,     gl::CLAMP_TO_EDGE),
        ];
        for (pname, val) in &params {
            gl::TextureParameteriv(tex, *pname, &(*val as i32));
        }

        gl::TextureStorage3D(tex, 1, gl::RGBA8, tile_width, tile_height, tile_count);

        for tile_y in 0..rows {
            for tile_x in 0..columns {
                let index = tile_y * columns + tile_x;
                let tile_image_buf = im
                    .view(
                        (tile_x * tile_width) as u32,
                        (tile_y * tile_height) as u32,
                        tile_width as u32,
                        tile_height as u32,
                    )
                    .to_image()
                    .into_vec();

                gl::TextureSubImage3D(
                    tex, 0,
                    0, 0, index,
                    tile_width, tile_height, 1,
                    gl::RGBA, gl::UNSIGNED_BYTE,
                    tile_image_buf.as_ptr() as *const std::ffi::c_void
                );
            }
        }

        Ok(tex)
    }
}

