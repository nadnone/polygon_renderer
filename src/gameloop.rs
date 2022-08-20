use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::maths_vectors_helper::{scalair, rotate};
use crate::wavefront_parser;
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;

pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{


    let mut data_cube = wavefront_parser::load("./assets/plane_animation.obj");

    for i in 0..data_cube.0.len() {

        data_cube.0[i] = scalair(data_cube.0[i], 150.0);
        
    }

    data_cube.0 = rotate(&data_cube.0, 3.1415, 'z');
    data_cube.0 = rotate(&data_cube.0, 15.0 * 3.1415 / 180.0, 'x');
    data_cube.0 = rotate(&data_cube.0, 45.0 * 3.1415 / 180.0, 'y');

    loop 
    {

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); 

        // lecture des events
        event_pump.poll_event();


        // transformations
        //data_cube.0 = rotate(&data_cube.0, 3.1415 / 180.0, 'x');
        data_cube.0 = rotate(&data_cube.0, 2. * 3.1415 / 180.0, 'y');
        data_cube.0 = rotate(&data_cube.0, 3.1415 / 180.0, 'z');


        // colorisation
        let for_projection = Rasterizer::draw(&mut data_cube);

        // projection
        projection(for_projection.0, for_projection.1, canvas);


        // affichage
        canvas.present();
        



        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
    
    }
}



