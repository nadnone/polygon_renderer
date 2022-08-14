use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{maths::{soustraction_vectors, cross_product_2x2}, misc::{HALF_WIDTH, HALF_HEIGHT}};



pub struct EdgeFunc;


impl EdgeFunc {

    pub fn draw(m: &Vec<[f32; 3]>, canvas: &mut Canvas<Window>)
    {

        for i in (0..m.len()).step_by(3) {
            


            let v0 = m[i + 0];
            let v1 = m[i + 1];
            let v2 = m[i + 2];

            let (min_x, max_x, min_y, max_y) = Self::_check_min_max(v0, v1, v2);


            for px in min_x..max_x+1 {
                
                for py in min_y..max_y+1 {


                    // pixel coordinate
                    let p = [px as f32, py as f32, 0.0];


                    // check if pixel is inside triangle
                    let mut inside_triangle = true;

                    inside_triangle &= Self::_edge_check(p, v0, v1);
                    inside_triangle &= Self::_edge_check(p, v1, v2);
                    inside_triangle &= Self::_edge_check(p, v2, v0);

                    if inside_triangle 
                    {

                        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
                        canvas.draw_point(sdl2::rect::Point::new(px, py)).unwrap();

                        
                    }
      
                }

            }


        }

    }

    fn _edge_check(p: [f32; 3], a: [f32; 3], b: [f32; 3]) -> bool
    {

        let delta_ab = soustraction_vectors(b, a);
        let delta_ap = soustraction_vectors(p, a);

        let normal = cross_product_2x2(delta_ab, delta_ap);

        return normal > 0.0;
    }

    fn _check_min_max(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> (i32, i32, i32, i32)
    {
        let mut max_x = 0.0;
        let mut min_x = 0.0;

        let mut max_y = 0.0;
        let mut min_y = 0.0;

        if v0[0] < v1[0] { max_x = v1[0]; }
        if max_x < v2[0] { max_x = v2[0]; }

        if v0[0] > v1[0] { min_x = v1[0]; }
        if min_x > v2[0] { min_x = v2[0]; }


        if v0[1] < v1[1] { max_y = v1[1]; }
        if max_y < v2[1] { max_y = v2[1]; }

        if v0[1] > v1[1] { min_y = v1[1]; }
        if min_y > v2[1] { min_y = v2[1]; }


        return (min_x as i32, max_x as i32, min_y as i32, max_y as i32)
    }

}
