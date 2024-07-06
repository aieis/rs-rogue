use std::time;
use std::thread;

mod term;
mod world;
mod draw;

use draw::*;
use world::*;

fn main() {
    term::init();
    let mut canvas = Canvas::new(50, 50);
    
    let mut counter = 0;
    let et = time::Instant::now();
    loop {
        let tt = et.elapsed().as_millis();
        let fps = (counter as f64 / tt as f64) * 1000.0;
        let fps_text = format!("{:.3}", fps);
        draw_text(&mut canvas, &fps_text, 0, 0);

        draw_circle(&mut canvas, 2, 10, 30);        
        draw_square(&mut canvas, 3, 8, 20);
        clear_background();
        swap_buffer(&canvas);
        counter += 1;
        thread::sleep(time::Duration::from_millis(33));
    }
}
