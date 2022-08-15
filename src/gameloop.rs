use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::maths::{scale_vec3, rotate};
use crate::wavefront_parser;
use crate::{misc::*, edge_function::EdgeFunc};
use crate::projection::projection;

pub fn gameloop(canvas: &mut Canvas<Window>, _event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{

    //let mut t = 0.0;
    let mut _triangle = vec![
        [-100.0,   -100.0,   100.0],
        [100.0,   -100.0,   100.0],
        [-100.0,   100.0,   100.0]
    ];
   

  

    let data_cube = wavefront_parser::load("./res/cube.obj");
    let mut cube_vertives = data_cube.0;


    for i in 0..cube_vertives.len() {

        cube_vertives[i] = scale_vec3(cube_vertives[i], 100.0);
        
    }



    loop 
    {
        //let t0 = std::time::Instant::now();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); 
    



        cube_vertives = rotate(cube_vertives, 0.1, 'x');
        cube_vertives = rotate(cube_vertives, 0.1, 'y');



        let for_projection = EdgeFunc::draw(&cube_vertives, &data_cube.1);


        projection(for_projection, canvas);
        canvas.present();
        



        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
        //t = t0.elapsed().as_secs_f32();
        //println!("{t}");
    
    }
}



