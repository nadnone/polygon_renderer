use std::vec;

pub fn soustraction_vectors(a: [f32; 3], b: [f32; 3]) -> [f32; 3]
{

    let mut res: [f32; 3] = [0.0, 0.0, 0.0];

    for i in 0..a.len() {
        
        res[i] = a[i] - b[i];
    }
    
    return res;
}

pub fn cross_product_2x2(a: [f32; 3], b: [f32; 3]) -> f32
{

    return (a[0] * b[1]) - (a[1] * b[0]);

}

pub fn scale_vec3(a: [f32; 3], factor: f32) -> [f32; 3]
{
    let mut res = [0.0, 0.0, 0.0];

    for i in 0..a.len() {
        res[i] = a[i] * factor;    
    }

    return res;
}

pub fn rotate_x(a: Vec<[f32; 3]>, angle: f32) -> Vec<[f32; 3]>
{

    let cos = angle.cos();
    let sin = angle.sin();

    let matrix_rot_x = [
        [1.0, 0.0, 0.0],
        [0.0, cos, -sin],
        [0.0, sin, cos]
    ];

    let mut m_out = Vec::new();

    for p in (0..a.len()).step_by(3) {



        let kernel = [a[p], a[p+1], a[p+2]];

        
        let mut res: [[f32; 3]; 3] = [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0]
        ];

        
        for i in 0..3 {

            for j in 0..3 {
                
                for k in 0..3 {

                    res[i][j] += kernel[i][k] * matrix_rot_x[k][j];

                }

            }

        }

        m_out.push(res[0]);
        m_out.push(res[1]);
        m_out.push(res[2]);
    }   

   

    return m_out;
}