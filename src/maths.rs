pub fn soustraction_vectors(a: [f32; 3], b: [f32; 3]) -> [f32; 3]
{

    let mut res: [f32; 3] = [0.0, 0.0, 0.0];

    for i in 0..a.len() {
        
        res[i] = a[i] - b[i];
    }
    
    return res;
}

pub fn cross_product_vec3(a: [f32; 3], b: [f32; 3]) -> [f32; 3]
{
    let mut cross = [0.0, 0.0, 0.0];

    cross[0] = (a[1] * b[2]) - (a[2] * b[1]);
    cross[1] = (a[2] * b[0]) - (a[0] * b[2]);
    cross[2] = (a[0] * b[1]) - (a[1] * b[0]);

    return cross;
}

pub fn dot(a: [f32; 3], b :[f32; 3]) -> f32
{
     return (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2]);
}


pub fn scale_vec3(a: [f32; 3], factor: f32) -> [f32; 3]
{
    let mut res = [0.0, 0.0, 0.0];

    for i in 0..a.len() {
        res[i] = a[i] * factor;    
    }

    return res;
}

pub fn rotate(a: Vec<[f32; 3]>, angle: f32, axe: char) -> Vec<[f32; 3]>
{

    let cos = angle.cos();
    let sin = angle.sin();

    let matrix_rot;

    if axe == 'y'
    {
        matrix_rot = [
            [cos, -sin, 0.0],
            [sin, cos, 0.0],
            [0.0, 0.0, 1.0]
        ];
    }
    else if axe == 'x'
    {
        matrix_rot = [
            [1.0, 0.0, 0.0],
            [0.0, cos, -sin],
            [0.0, sin, cos]
        ];
    }
    else 
    {
        matrix_rot = [
            [cos, -sin, 0.0],
            [sin, cos, 0.0],
            [0.0, 0.0, 1.0]
        ];
    }

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

                    res[i][j] += kernel[i][k] * matrix_rot[k][j];

                }

            }

        }

        m_out.push(res[0]);
        m_out.push(res[1]);
        m_out.push(res[2]);
    }   

   

    return m_out;
}