
mod map;
mod player;

use {
    self::{
        player::Player,
        map::Map,
    },
    crate::{
        alg::{P2, V2},
        gfx::{shader, load_atlas_texture},
        Event,
    },
    std::{
        error::Error,
        mem,
        time::{Duration, Instant},
        vec::Vec,
    },
    gl::types::*,
};

#[derive(Debug)]
struct Camera {
    centre: P2,
    scale:  f32,
}

impl Camera {
    fn make_frustum(&self, screen_dims: V2) -> Frustum {
        let centre = self.centre;
        let half_dims = screen_dims * 0.5 * self.scale;

        Frustum { centre, half_dims }
    }
}

#[derive(Debug)]
struct Frustum {
    centre:    P2,
    half_dims: V2,
}

impl Frustum {
    fn int_bounds(&self, scale: f32) -> IntRect {
        let bottom_left = ((self.centre - self.half_dims) * scale)
            .coords
            .map(|x| x.floor());
        let top_right = ((self.centre + self.half_dims) * scale)
            .coords
            .map(|x| x.ceil());
        IntRect {
            left:   bottom_left.x as i32,
            bottom: bottom_left.y as i32,
            right:  top_right.x   as i32,
            top:    top_right.y   as i32,
        }
    }
}

#[derive(Debug)]
struct IntRect {
    left:   i32,
    bottom: i32,
    right:  i32,
    top:    i32,
}

pub fn main_thread(
    ctx: &glutin::WindowedContext<glutin::PossiblyCurrent>,
    event_receiver: &std::sync::mpsc::Receiver<Event>
)   -> Result<(), Box<dyn Error>>
{
    let map = Map::load("test.tmx")?;

    let player_texture = load_atlas_texture("player.png", 16, 16)?;

    let mut screen_dims = V2::new(1024.0, 1024.0);

    let mut player = Player::new(P2::new(100.0, 100.0));
    let mut inputs = player::Inputs::new();

    let mut time_accum = Duration::from_secs(0);
    let mut prev_now = Instant::now();

    const TICK_FREQ: u64 = 60;
    const TICK_DURATION: Duration = Duration::from_nanos(1_000_000_000 / TICK_FREQ);

    'main_loop: loop {
        // event processing
        'event_loop: loop {
            let event = match event_receiver.try_recv() {
                Err(error) => {
                    use std::sync::mpsc::TryRecvError::*;
                    match error {
                        Disconnected => break 'main_loop,
                        Empty        => break 'event_loop,
                    }
                }

                Ok(event) => event
            };

            use glutin::event::Event::*;
            match event {
                WindowEvent { event, .. } => {
                    use glutin::event::WindowEvent::*;
                    match event {
                        CloseRequested => break 'main_loop,

                        Resized(new_size) => {
                            let phys = new_size.to_physical(1.0);
                            ctx.resize(phys);
                            let (w, h): (f64, f64) = phys.into();
                            screen_dims = V2::new(w as f32, h as f32);
                        }

                        KeyboardInput {
                            input: glutin::event::KeyboardInput {
                                state, virtual_keycode: Some(vk), ..
                            },
                            ..
                        } => {
                            use glutin::event::VirtualKeyCode as VK;
                            let down = state == glutin::event::ElementState::Pressed;
                            match vk {
                                VK::A => inputs.left = down,
                                VK::D => inputs.right = down,
                                VK::Space => inputs.jump = down,
                                _ => { }
                            }
                        }

                        _ => (),
                    }
                }

                _ => (),
            }
        }

        // advance clock
        {   let now = Instant::now();
            time_accum += now - prev_now;
            prev_now = now;
        }

        // control update

        // game ticks
        while time_accum > TICK_DURATION {
            player.tick(&inputs, TICK_DURATION.as_secs_f32());
            time_accum -= TICK_DURATION;
        }

        //eprintln!("{:?}", player);

        let camera = Camera {
            centre: player.position,
            scale:   1.0 / 4.0
        };

        let frustum = camera.make_frustum(screen_dims);
        //eprint!("frustum: {:#?}", frustum);

        let bounds = frustum.int_bounds(1.0 / 16.0);
        //eprint!("bounds: {:#?}", bounds);

        for world_y in bounds.bottom..bounds.top {
            for world_x in bounds.left..bounds.right {
                if let Some((set, tile, index)) = map.tile_at(world_x, world_y) {
                    let x = world_x as f32 * 16.0;
                    let y = world_y as f32 * 16.0;
                    let rect = Rect::new(x, y, x + 16.0, y + 16.0);
                    sprites.push((set.texture(), Sprite{rect, texture_index: index}));
                }
            }
        }

        {   let p = player.position;
            let rect = Rect::new(p.x - 8.0, p.y, p.x + 8.0, p.y + 16.0);
            sprites.push((player_texture, Sprite{rect, texture_index: 0}));
            lines.extend(stroke(rect.verts(), 255, 255, 0, 255))
        }
    }

    Ok(())
}

