use crate::misc::*;


pub fn projection(m: &Vec<[f32; 3]>) -> Vec<[f32; 3]>
{

    let mut m_out = Vec::new();
    
    let f: f32 = (FOV/2.0).tan() / 2.0;

    for i in 0..m.len() {
        

        let x0 = m[i][0] as f32;
        let y0 = m[i][1] as f32;
        let z0 = m[i][2] as f32; 


        let mut x = (HEIGHT/WIDTH) * f * x0;
        let mut y = f * y0;
        let z = LAMBDA * z0 - 1.0;


        x += HALF_WIDTH as f32;
        y += HALF_HEIGHT as f32;


        m_out.push([x, y, z]);
    }

    return m_out;
}