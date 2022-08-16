use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::maths::{scalair, rotate};
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
    let mut cube_vertices = data_cube.0;

    for i in 0..cube_vertices.len() {

        cube_vertices[i] = scalair(cube_vertices[i], 100.0);
        
    }


    loop 
    {
        //let t0 = std::time::Instant::now();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); 
    



        cube_vertices = rotate(cube_vertices, 3.1415 / 180.0, 'x');
        cube_vertices = rotate(cube_vertices, 3.1415 / 180.0, 'y');



        let for_projection = EdgeFunc::draw(&cube_vertices, &data_cube.1, &data_cube.2);

        projection(for_projection, canvas);
        canvas.present();
        



        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
        //t = t0.elapsed().as_secs_f32();
        //println!("{t}");
    
    }
}



