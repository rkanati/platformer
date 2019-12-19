
use {
    crate::{
        alg::{V2, P2},
    },
};

#[derive(Clone, Copy, Debug)]
enum PhysState {
    Walking { vx: f32 },
    Falling { velocity: V2 },
}

#[derive(Clone, Copy, Debug)]
pub struct Inputs {
    pub left:  bool,
    pub right: bool,
    pub jump:  bool,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs { left: false, right: false, jump: false }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub position: P2,
    phys_state: PhysState,
}

impl Player {
    pub fn new(position: P2) -> Player {
        Player {
            position,
            phys_state: PhysState::Falling { velocity: V2::new(0.0, 0.0) }
        }
    }

    pub fn tick(&mut self, inputs: &Inputs, dt: f32) {
        const AY_GRAVITY: f32 = -750.0;
        const Y_MAX_JUMP: f32 = 56.0;
        let   VY_JUMP = (-2.0 * AY_GRAVITY * Y_MAX_JUMP).sqrt();
        const VY_TERMINAL: f32 = -200.0;

        const AX_AIR:       f32 = 250.0;
        const AX_WALK:      f32 = 750.0;
        const VX_MAX_AIR:   f32 = 250.0;
        const VX_MAX_WALK:  f32 = 150.0;
        const WALK_DAMPING: f32 = 20.0;

        const Y_FLOOR: f32 = -32.0;

        match self.phys_state {
            PhysState::Walking { vx } => {
                let vx = {
                    let mut ax = 0.0;
                    if inputs.left { ax -= AX_WALK; }
                    if inputs.right { ax += AX_WALK; }
                    let vx = (vx + ax * dt).clamp(-VX_MAX_WALK, VX_MAX_WALK);
                    //let vx = vx * 0.8;
                    if vx.abs() < 0.001 { 0.0 } else { vx }
                };
                self.position.x += vx * dt;

                self.phys_state = if inputs.jump {
                    let velocity = V2::new(vx, VY_JUMP);
                    PhysState::Falling { velocity }
                }
                else {
                    PhysState::Walking { vx }
                };
            }

            PhysState::Falling { velocity } => {
                let vx = {
                    let mut ax = 0.0;
                    if inputs.left { ax -= AX_AIR; }
                    if inputs.right { ax += AX_AIR; }
                    (velocity.x + ax * dt).clamp(-VX_MAX_AIR, VX_MAX_AIR)
                };

                let vy = (velocity.y + AY_GRAVITY * dt).max(VY_TERMINAL);

                let velocity = V2::new(vx, vy);
                self.position += velocity * dt;

                self.phys_state = if self.position.y < Y_FLOOR {
                    self.position.y = Y_FLOOR;
                    PhysState::Walking { vx }
                }
                else {
                    PhysState::Falling { velocity }
                };
            }
        }
    }
}

