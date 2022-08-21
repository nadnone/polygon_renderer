use crate::constants::*;

pub fn projection(m: &mut Vec<[f32; 3]>)
{

    let f: f32 = (FOV/2.0).tan() / 2.0;


    for i in 0..m.len() {
        

        let x0 = m[i][0];
        let y0 = m[i][1];
        let z0 = m[i][2]; 


        let x = (HEIGHT/WIDTH) * f * x0;
        let y = f * y0;
        let z = LAMBDA * (z0 - 1.0); // 1.0 = Z_NEAR

        
        m[i][0] = x;
        m[i][1] = y;
        m[i][2] = z;

        
     
    }

}