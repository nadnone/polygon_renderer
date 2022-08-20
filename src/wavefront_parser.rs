use std::fs;
use regex::Regex;
use image::{self, DynamicImage};


pub fn load(filename: &str) -> (Vec<[f32; 3]>, Vec<[f32; 3]>,/* (DynamicImage, Vec<[f32; 2]>),*/ (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>, f32))
{

    let mut data = fs::read_to_string(filename).unwrap();
    let mut str = &data[..];



    // capture de faces data
    let mut re = Regex::new(r"f\s(\d+).(\d+).(\d+)\s(\d+).(\d+).(\d+)\s(\d+).(\d+).(\d+)").unwrap();

    let mut vertices_indices: Vec<[i32; 3]> = Vec::new();
    let mut normals_indices: Vec<[i32; 3]> = Vec::new();
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
        

        let n1: i32 = capture[3].to_string().parse().unwrap();
        let n2: i32 = capture[6].to_string().parse().unwrap();
        let n3: i32 = capture[9].to_string().parse().unwrap();

        normals_indices.push([n1, n2, n3]);
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


    // vertex normals 
    re = Regex::new(r"vn\s([0-9\.\-]+)\s([0-9\.\-]+)\s([0-9\.\-]+)").unwrap();

    let mut normals_mess: Vec<[f32; 3]> = Vec::new();

    for capture in re.captures_iter(str) {

        let n1: f32 = capture[1].to_string().parse().unwrap();
        let n2: f32 = capture[2].to_string().parse().unwrap();
        let n3: f32 = capture[3].to_string().parse().unwrap();

        normals_mess.push([n1, n2, n3]);

    }




    
    // capture the textures coordinates
    re = Regex::new(r"vt\s([0-9\.\-]+)\s([0-9\.\-]+)").unwrap();

    let mut textures_coordinates_mess: Vec<[f32; 2]> = Vec::new();

    for capture in re.captures_iter(str) {
        
        let t1: f32 = capture[1].to_string().parse().unwrap();
        let t2: f32 = capture[2].to_string().parse().unwrap();

        textures_coordinates_mess.push([t1, t2]);

    }
    

    // the materials and the mtl file
    let mut re_1 = Regex::new(r"mtllib\s(.*)").unwrap().find(str).unwrap();
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

    // triangles
    let mut normals: Vec<[f32; 3]> = Vec::new();
    for i in 0..normals_indices.len() {

        for j in 0..3 {
            
            let v = normals_indices[i][j] as usize;

            normals.push( normals_mess[v-1] );
        }

                
    }
    


    // textures coordinates
    let mut triangles_texture_coordinates: Vec<[f32; 2]> = Vec::new();

    for i in 0..textures_indices.len() {
        
        for j in 0..3 {

            let t = vertices_indices[i][j] as usize;

            triangles_texture_coordinates.push( textures_coordinates_mess[t] );
        }

    }







    // colors 

    // read the mtl file
    data = fs::read_to_string(format!("./assets/{}", mtl_filename)).unwrap();
    str = &data[..];



    // take the first material ambiant
    re = Regex::new(r"newmtl\s(\w+)\n.*\nKa\s([0-9\.]+)\s([0-9\.]+)\s([0-9\.]+)\s").unwrap();

    let mut ambiants: Vec<[f32; 3]> = Vec::new();

    for capture in re.captures_iter(str) {


        let r: f32 = capture[2].to_string().parse().unwrap();
        let g: f32 = capture[3].to_string().parse().unwrap();
        let b: f32 = capture[4].to_string().parse().unwrap();

        ambiants.push( [r, g, b ]);


    }

    // take the first material diffuse
    re = Regex::new(r"newmtl\s(\w+)\n.*\n.*\nKd\s([0-9\.]+)\s([0-9\.]+)\s([0-9\.]+)\s").unwrap();

    let mut diffuse: Vec<[f32; 3]> = Vec::new();

    for capture in re.captures_iter(str) {

        

        let r: f32 = capture[2].to_string().parse().unwrap();
        let g: f32 = capture[3].to_string().parse().unwrap();
        let b: f32 = capture[4].to_string().parse().unwrap();

        diffuse.push( [r, g, b] );


    }
    

    // the specular exponent and the mtl file
    re_1 = Regex::new(r"Ns\s([0-9\.]+)").unwrap().find(str).unwrap();
    let string_stuff = &data[re_1.start()+3..re_1.end()];
    let specular_exponent: f32 = string_stuff.to_string().parse().unwrap();
        

    // take the first material diffuse
    re = Regex::new(r"newmtl\s(\w+)\n.*\n.*\n.*\nKs\s([0-9\.]+)\s([0-9\.]+)\s([0-9\.]+)\s").unwrap();

    let mut specular: Vec<[f32; 3]> = Vec::new();

    for capture in re.captures_iter(str) {

        

        let r: f32 = capture[2].to_string().parse().unwrap();
        let g: f32 = capture[3].to_string().parse().unwrap();
        let b: f32 = capture[4].to_string().parse().unwrap();

        specular.push( [r, g, b] );


    }


    // the image texture in the mtl file
    /*
    re_1 = Regex::new(r"map_Kd\s(.*)\.\w+").unwrap().find(str).unwrap();
    let file_image_name = &data[re_1.start()+7..re_1.end()];
    let ifimage = image::io::Reader::open(format!("./assets/{}",file_image_name));

    let mut image_data: DynamicImage = DynamicImage::new_rgb8(0, 0);

    if !ifimage.is_err()
    {
        image_data = ifimage.unwrap().decode().unwrap();
    }
    else 
    {
        println!("can't find image texture");
    }
    */


    return (triangles, normals,/* (image_data, triangles_texture_coordinates),*/ (ambiants, diffuse, specular, specular_exponent));

}