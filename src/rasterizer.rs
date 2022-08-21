//use image::{DynamicImage, GenericImageView};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{pseudo_shader, constants::{WIDTH_LOGIC, HEIGHT_LOGIC}};

pub struct Rasterizer;


impl Rasterizer {

    pub fn draw(canvas: &mut Canvas<Window>, data: &(Vec<[f32; 3]>, Vec<[f32; 3]>, /*(DynamicImage, Vec<[f32; 2]>),*/ (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>, f32)))
    {

        let m = &data.0;
        let normals = &data.1;
        //let image_texture_data = &data.2;
        let phong_data = &data.2;

        //let image_texture = &image_texture_data.0;
        //let _image_texture_coordinates = &image_texture_data.1;

        for i in (0..m.len()).step_by(3) {
            


            let v0 = m[i + 0];
            let v1 = m[i + 1];
            let v2 = m[i + 2];

            
            // to check less pixels at time
            let (min_x, max_x, min_y, max_y) = Self::_check_min_max(v0, v1, v2);



            for px in min_x..max_x+1 {
                
                for py in min_y..max_y+1 {


                    // pixel coordinate
                    let p = [px as f32, py as f32, 0.0];


                    // check if pixel is inside triangle
                    if Self::_is_in_triangle(p, v0, v1, v2) 
                    {


                        // A REVOIR (texture mapping)
                        /*
                        let mut rel_px = (px - min_x) as f32 / (max_x - min_x) as f32;
                        let mut rel_py = (py - min_y) as f32 / (max_y - min_y) as f32;
 
                        rel_px *= image_texture.width() as f32 - 1.0;
                        rel_py *= image_texture.height() as f32 - 1.0;



                        // texels
                        let tx = rel_px as u32;
                        let ty = rel_py  as u32;

                        let r = image_texture.get_pixel(tx, ty)[0];
                        let g = image_texture.get_pixel(tx, ty)[1];
                        let b = image_texture.get_pixel(tx, ty)[2];

                        */
                        let rgb = pseudo_shader::pseudo_shader(normals, p, phong_data, i);

                        canvas.set_draw_color(sdl2::pixels::Color::RGB(rgb[0], rgb[1], rgb[2]));
                        canvas.draw_point(sdl2::rect::Point::new(px as i32 + WIDTH_LOGIC /2, py as i32 + HEIGHT_LOGIC /2)).unwrap();

                    }
      
                }

            }


        }

    }

    fn _is_in_triangle(p: [f32; 3], a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> bool
    {
        // positifs
        let mut check_pos = true;
        check_pos &= Self::_edge_check(p, c, a) > 0.0;
        check_pos &= Self::_edge_check(p, a, b) > 0.0;
        check_pos &= Self::_edge_check(p, b, c) > 0.0;

        // negatifs
        let mut check_neg = true;
        check_neg &= Self::_edge_check(p, c, a) < 0.0;
        check_neg &= Self::_edge_check(p, a, b) < 0.0;
        check_neg &= Self::_edge_check(p, a, c) < 0.0;

        
        return check_pos | check_neg; 

    }

    fn _edge_check(p: [f32; 3], a: [f32; 3], b: [f32; 3]) -> f32
    {
        // calcul du determinant
        return (a[0] - p[0]) * (b[1] - p[1]) - (a[1] - p[1]) * (b[0] - p[0]);
    }

    fn _check_min_max(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> (i32, i32, i32, i32)
    {
        let max_x = *vec![v0[0] as i32, v1[0] as i32, v2[0] as i32].iter().max().unwrap();
        let min_x = *vec![v0[0] as i32, v1[0] as i32, v2[0] as i32].iter().min().unwrap();

        let max_y = *vec![v0[1] as i32, v1[1] as i32, v2[1] as i32].iter().max().unwrap();
        let min_y = *vec![v0[1] as i32, v1[1] as i32, v2[1] as i32].iter().min().unwrap();

        return (min_x as i32, max_x as i32, min_y as i32, max_y as i32)
    }

}
