use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::maths_vectors_helper::{scalair, rotate};
use crate::wavefront_parser;
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;

pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{


    let data_cube = wavefront_parser::load("./assets/cube.obj");
    let mut cube_vertices = data_cube.0;

    for i in 0..cube_vertices.len() {

        cube_vertices[i] = scalair(cube_vertices[i], 150.0);
        
    }

    cube_vertices = rotate(cube_vertices, 3.1415, 'z');
    cube_vertices = rotate(cube_vertices, 15.0 * 3.1415 / 180.0, 'x');
    cube_vertices = rotate(cube_vertices, 45.0 * 3.1415 / 180.0, 'y');

    loop 
    {

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); 

        // lecture des events
        event_pump.poll_event();


        // transformations
        //cube_vertices = rotate(cube_vertices, 3.1415 / 180.0, 'x');
        cube_vertices = rotate(cube_vertices, 2. * 3.1415 / 180.0, 'y');
        //cube_vertices = rotate(cube_vertices, 3.1415 / 180.0, 'z');


        // colorisation
        let for_projection = Rasterizer::draw(&cube_vertices, &data_cube.1,&data_cube.2);

        // projection
        projection(for_projection, canvas);

        // affichage
        canvas.present();
        



        std::thread::sleep(std::time::Duration::from_secs_f32(FPS));
    
    }
}



