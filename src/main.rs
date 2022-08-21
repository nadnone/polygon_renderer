
mod constants;
mod gameloop;
mod rasterizer;
mod maths_vectors_helper;
mod projection;
mod wavefront_parser;
mod pseudo_shader;

use crate::{gameloop::gameloop, constants::*};


pub fn main()
{
    

    println!("Loading window");
    let mut sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let wind = video_subsystem.window("Polygon Render", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        //fullscreen_desktop()
        .build()
        .unwrap();
    
    println!("Initialization.");

    let mut canvas = wind.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_logical_size(WIDTH_LOGIC as u32, HEIGHT_LOGIC as u32).unwrap();

    println!("Start!");
    gameloop(&mut canvas, &mut event_pump, &mut sdl_context);
    
   


 

}