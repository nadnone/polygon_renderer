use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::maths::scale_vec3;
use crate::{misc::*, edge_function::EdgeFunc};
use crate::projection::projection;

pub fn gameloop(canvas: &mut Canvas<Window>, _event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{

    //let mut t = 0.0;
    let mut triangle = vec![
        [-100.0,   -100.0,   100.0],
        [100.0,   -100.0,   100.0],
        [-100.0,   100.0,   100.0]
    ];
   

    for i in 0..triangle.len() {

        triangle[i] = scale_vec3(triangle[i], 2.0);
        
    }


    loop 
    {
        //let t0 = std::time::Instant::now();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
    





        let projection_mat = projection(&triangle);
        EdgeFunc::draw(&projection_mat, canvas);

        
        
        
        
        canvas.present();
        



        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
        //t = t0.elapsed().as_secs_f32();
        //println!("{t}");
    
    }
}



