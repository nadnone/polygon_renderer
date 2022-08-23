use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::gltf_file_loader;
use crate::transformations::{scale, rotate};
use crate::{constants::*, rasterizer::Rasterizer};
use crate::projection::projection;

pub fn gameloop(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl)
{


    let mut object_data = gltf_file_loader::GLTFLoader::load("./assets/cube.glb");

    // pre transformations
    object_data.0 = scale(&object_data.0, 150.0);

    let mut i = 0.0;

    loop 
    {

        let mut object = object_data.clone();



        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear(); 

        // lecture des events
        event_pump.poll_event();


        // transformations
        object.0 = rotate(&object.0, i * PI / 180.0, 'y');
        object.0 = rotate(&object.0, i * PI / 180.0, 'x');




        


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



