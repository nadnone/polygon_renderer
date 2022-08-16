use crate::maths_vectors_helper::*;

pub fn shader_phong(normals: &Vec<[f32; 3]>, v0: [f32; 3], phong_data: &(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>), i: usize) -> [u8; 3]
{
    // diffuse
    let norm = normals[i];
    let light = [10.0, 1.8, 1.5];
    let light_dir = normaliser(soustraction_vectors([0.0, 600.0, 0.0], v0));

    let diff = produit_vectoriel(norm, light_dir); // L . N
    let diffuse = produit_vectoriel(light, diff); // I v K

    // ambient                        
    let ambient = produit_vectoriel(light, phong_data.0[0]);


    // specular
    let cam_pos = normaliser(normaliser([0.0, 0.0, 100.0]));
    let r = scalair(reflect(light_dir, cam_pos), -1.0);
    


    let spec_diff = produit_scalair(light, phong_data.2[0]); // V.R
    let spec_rv = produit_vectoriel(r,  norm); // R v N
    let specular = scalair(spec_rv, spec_diff);
    

    let mut phong = addition_vectors(diffuse, ambient);
        phong = addition_vectors(phong, specular);    
        phong = produit_vectoriel(phong, phong_data.1[0]);

    let r = phong[0] * 255.0;
    let g = phong[1] * 255.0;
    let b = phong[2] * 255.0;

    return [r as u8, g as u8, b as u8];
}

fn reflect(light_dir: [f32; 3], normal: [f32; 3]) -> [f32; 3]
{
    let a = produit_vectoriel(normal, scalair(light_dir, -1.0));

    let b = produit_vectoriel(normal, a);
    
    let total = produit_vectoriel(normal, b);

    return total;
}