use std::fs;
use regex::Regex;


pub fn load(filename: &str) -> (Vec<[f32; 3]>, Vec<[u8; 3]>)
{

    let mut data = fs::read_to_string(filename).unwrap();
    let mut str = &data[..];



    // capture de faces data
    let mut re = Regex::new(r"f\s(\d+).(\d+).(\d+)\s(\d+).(\d+).(\d+)\s(\d+).(\d+).(\d+)").unwrap();

    let mut vertices_indices: Vec<[i32; 3]> = Vec::new();
    let mut textures_indices: Vec<[i32; 3]> = Vec::new();

    for capture in re.captures_iter(str) {


        let v1: i32 = capture[1].to_string().parse().unwrap();
        let v2: i32 = capture[4].to_string().parse().unwrap();
        let v3: i32 = capture[7].to_string().parse().unwrap();

        vertices_indices.push([v1, v2, v3]);


        let t1: i32 = capture[2].to_string().parse().unwrap();
        let t2: i32 = capture[5].to_string().parse().unwrap();
        let t3: i32 = capture[8].to_string().parse().unwrap();

        textures_indices.push([t1, t2, t3]);


    }

    // capture the vertices
    re = Regex::new(r"v\s([0-9\.\-]+)\s([0-9\.\-]+)\s([0-9\.\-]+)").unwrap();

    let mut vertices_mess: Vec<[f32; 3]> = Vec::new();

    for capture in re.captures_iter(str) {

        let v1: f32 = capture[1].to_string().parse().unwrap();
        let v2: f32 = capture[2].to_string().parse().unwrap();
        let v3: f32 = capture[3].to_string().parse().unwrap();

        vertices_mess.push([v1, v2, v3]);

    }

    /*
    // capture the textures coordinates
    re = Regex::new(r"vt\s([0-9\.\-]+)\s([0-9\.\-]+)").unwrap();

    let mut textures_coordinates_mess: Vec<[f32; 2]> = Vec::new();

    for capture in re.captures_iter(str) {
        
        let t1: f32 = capture[1].to_string().parse().unwrap();
        let t2: f32 = capture[2].to_string().parse().unwrap();

        textures_coordinates_mess.push([t1, t2]);

    }
    */

    // the materials and the mtl file
    let mut re_1 = Regex::new(r"mtllib\s([a-z\.0-9A-Z]+)").unwrap().find(str).unwrap();
    let mtl_filename = &data[re_1.start()+7..re_1.end()];





    // tri all data
    
    // triangles
    let mut triangles: Vec<[f32; 3]> = Vec::new();

    for i in 0..vertices_indices.len() {

        for j in 0..3 {
            
            let v = vertices_indices[i][j] as usize;

            triangles.push( vertices_mess[v-1] );
        }

                
    }

/*
    // textures coordinates
    let mut triangles_colors_coordinates: Vec<[f32; 2]> = Vec::new();

    for i in 0..textures_indices.len() {
        
        for j in 0..3 {

            let t = vertices_indices[i][j] as usize;

            triangles_colors_coordinates.push( textures_coordinates_mess[t] );
        }

    }

*/

    // colors 
    
    //get the pwd
    re_1 = Regex::new(r".\w+\.obj").unwrap().find(filename).unwrap();
    let obj_name =  &filename[re_1.start()..re_1.end()];
    let pwd = filename.replace(obj_name, "");

    // read the mtl file
    data = fs::read_to_string(format!("{}/{}", pwd, mtl_filename)).unwrap();
    str = &data[..];



    // take the first material
    re = Regex::new(r"newmtl\s(\w+)\n.*\nKa\s([0-9\.]+)\s([0-9\.]+)\s([0-9\.]+)\s").unwrap();

    let mut material: Vec<[u8; 3]> = Vec::new();

    for capture in re.captures_iter(str) {

        

        let mut r: f32 = capture[2].to_string().parse().unwrap();
        let mut g: f32 = capture[2].to_string().parse().unwrap();
        let mut b: f32 = capture[2].to_string().parse().unwrap();

        r *= 255.0; g *= 255.0; b *= 255.0; 

        material.push( [r as u8, g as u8, b as u8] );


    }





    return (triangles, material);


}