use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::maths_vectors_helper::{scalair, rotate};
use crate::wavefront_parser;
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;

pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{


    let object_data = wavefront_parser::load("./assets/plane_animation.obj");

    

    //data_cube.0 = rotate(&data_cube.0, 3.1415, 'z');
    //data_cube.0 = rotate(&data_cube.0, 15.0 * 3.1415 / 180.0, 'x');
    //data_cube.0 = rotate(&data_cube.0, 45.0 * 3.1415 / 180.0, 'y');

    let mut i = 0.0;

    loop 
    {

        let mut object = object_data.clone();

        for i in 0..object.0.len() {

            object.0[i] = scalair(object.0[i], 150.0);
            
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); 

        // lecture des events
        event_pump.poll_event();


        // transformations
        object.0 = rotate(&object.0, i * 3.1415 / 180.0, 'x');
        object.0 = rotate(&object.0, i * 3.1415 / 180.0, 'y');
        object.0 = rotate(&object.0, i * 3.1415 / 180.0, 'z');




        


        // projection
        projection(&mut object.0);

        // colorisation
        Rasterizer::draw(canvas, &object);

       

        // affichage
        canvas.present();
        



        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
    

        i += 1.0;
        i %= 360.0;
    }
}



