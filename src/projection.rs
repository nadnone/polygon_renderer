use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::misc::*;


pub fn projection(m: Vec<(f32, f32, f32, [u8; 3])>, canvas: &mut Canvas<Window>)
{

    let f: f32 = (FOV/2.0).tan() / 2.0;

    for i in 0..m.len() {
        

        let x0 = m[i].0;
        let y0 = m[i].1;
        let z0 = m[i].2; 

        let colors = m[i].3;


        let x = (HEIGHT/WIDTH) * f * x0;
        let y = f * y0;
        let _z = LAMBDA * z0 - 1.0;

        canvas.set_draw_color(sdl2::pixels::Color::RGB(colors[0], colors[1], colors[2]));
        canvas.draw_point(sdl2::rect::Point::new(x as i32 + HALF_WIDTH/2, y as i32 + HALF_HEIGHT/2)).unwrap();


    }

}