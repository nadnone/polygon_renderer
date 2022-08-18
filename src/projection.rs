use crate::constants::*;
use sdl2::render::Canvas;
use sdl2::video::Window;


pub fn projection(m: Vec<[f32; 3]>, rgb: Vec<[u8; 3]>, canvas: &mut Canvas<Window>) -> Vec<[f32; 3]>
{

    let f: f32 = (FOV/2.0).tan() / 2.0;

    let mut m_out: Vec<[f32; 3]> = Vec::new();

    for i in 0..m.len() {
        

        let x0 = m[i][0];
        let y0 = m[i][1];
        let z0 = m[i][2]; 

        let colors = rgb[i];


        let x = (HEIGHT/WIDTH) * f * x0;
        let y = f * y0;
        let z = LAMBDA * (z0 - 1.0); // 1.0 = Z_NEAR

        
        m_out.push([x, y, z]);

        
        canvas.set_draw_color(sdl2::pixels::Color::RGB(colors[0], colors[1], colors[2]));
        canvas.draw_point(sdl2::rect::Point::new(x as i32 + WIDTH_LOGIC /2, y as i32 + HEIGHT_LOGIC /2)).unwrap();
    }

    return m_out;
}