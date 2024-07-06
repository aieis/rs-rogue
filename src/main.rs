use std::io;

mod term;
mod world;
mod draw;

use draw::*;
use world::*;

// const FOV_X: f64 = 65.0;
// const FOV_Y: f64 = 45.0;
const FOCAL_LENGTH: i32 = 10;

fn main() {
    term::init();
    let mut canvas = Canvas::new(50, 50);
    let mut world = World {camera: Vec3::new(0, 0, 0), cubes: Vec::new()};
    world.cubes.push(Cube {
        position: Vec3::new(-5, -5, 20),
        length: 10
    });

    let mut ev = std::iter::repeat("  ").take(10).collect::<String>();
    loop {
        canvas.clear();
        for cube in world.cubes.iter() {
            //project each cube into the 2-D view relative to the camera
            let camera = &world.camera;

            // top-left corner
            let dx = cube.position.x - camera.x;
            let dy = cube.position.y - camera.y;

            let odx = dx + cube.length;
            let ody = dy + cube.length;

            // since the camera is facing forward?
            let z_orth = cube.position.z - camera.z;
            let dx_proj = dx * FOCAL_LENGTH / z_orth;
            let dy_proj = dy * FOCAL_LENGTH / z_orth;
            let odx_proj = odx * FOCAL_LENGTH / z_orth;
            let ody_proj = ody * FOCAL_LENGTH / z_orth;

            let mut x = canvas.w() / 2 + dx_proj;
            let mut y = canvas.h() / 2 + ody_proj;
            let mut height = odx_proj - dx_proj;
            let mut width = ody_proj - dy_proj;

            if x >= canvas.w() {
                continue;
            }

            if y >= canvas.h() {
                continue;
            }

            if x + width < 0 {
                continue;
            }

            if y + height < 0 {
                continue;
            }

            if x < 0 {
                width = width + x;
                x = 0;
            }

            if y < 0 {
                height = height + y;
                y = 0;
            }

            if x + width >= canvas.w() {
                width = width - (x + width - canvas.w() - 1);
            }

            if y + height >= canvas.h() {
                height = height - (y + height - canvas.h() - 1);
            }

            draw_rectangle(&mut canvas, width as usize, height as usize, x as usize, y as usize);
        }

        draw_text(&mut canvas, &ev, 0, 0);

        clear_background();
        swap_buffer(&canvas);

        let mut valid_ev = false;
        while !valid_ev {
            let key = term::read_key();            
            if key.is_none() {
                continue;
            }

            let key = key.unwrap();            
            valid_ev = true;
            match key {
                'w' => {
                    world.camera.z = world.camera.z + 1;
                }

                's' => {
                    world.camera.z = world.camera.z - 1;
                }

                'a' => {
                    world.camera.x = world.camera.x - 1;
                }

                'd' => {
                    world.camera.x = world.camera.x + 1;
                }

                _ => {                    
                    valid_ev = false;
                }
            }

            ev.remove(0);
            ev.remove(1);

            let s = format!(" {key}");
            ev.push_str(&s);            
        }
    }
}
