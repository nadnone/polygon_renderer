pub struct GLTFLoader;

impl GLTFLoader {


    pub fn load(filename: &str) -> (Vec<[f32; 6]>, Vec<[f32; 3]>, Vec<[f32; 3]>)
    {



        let (gltf, buffers, _) = gltf::import(filename).unwrap();


        let mut models_points = Vec::new();
        let mut models_normals = Vec::new();
        let mut models_colors = Vec::new();

           

        for mesh in gltf.meshes()
        {

            for primitive in mesh.primitives()
            {

                
                let material = primitive.material().pbr_metallic_roughness();
                let rgb_diffuse = material.base_color_factor();

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                if let Some(iter) = reader.read_indices() {
                    for vertex_indices in iter.into_u32() {

                        let position =  reader.read_positions().unwrap().into_iter().nth(vertex_indices as usize).unwrap();
                        let normals =  reader.read_normals().unwrap().into_iter().nth(vertex_indices as usize).unwrap();
                        
                        models_points.push([
                            position[0],
                            position[1],
                            position[2],

                            position[0],
                            position[1],
                            position[2]
                        ]);

                        models_normals.push(normals);

                        models_colors.push([rgb_diffuse[0], rgb_diffuse[1], rgb_diffuse[2]]);

                    }
                }
               


            }



        }

        if models_points.len() % 3 != 0
        {
            println!("[!] Objet mal chargé ou non triangulé");
        }


        return (models_points.clone(), models_normals.clone(), models_colors.clone());



    }

}