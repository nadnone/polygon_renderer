use crate::shader;

pub struct Rasterizer;


impl Rasterizer {

    pub fn draw(m: &Vec<[f32; 3]>, normals: &Vec<[f32; 3]>, phong_data: &(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>, f32)) -> Vec<(f32, f32, f32, [u8; 3])>
    {

        let mut m_out: Vec<(f32, f32, f32, [u8; 3])> = Vec::new();

        for i in (0..m.len()).step_by(3) {
            


            let v0 = m[i + 0];
            let v1 = m[i + 1];
            let v2 = m[i + 2];

            
            // to check less pixels at time
            let (min_x, max_x, min_y, max_y) = Self::_check_min_max(v0, v1, v2);



            for px in min_x..max_x+1 {
                
                for py in min_y..max_y+1 {


                    // pixel coordinate
                    let p = [px as f32, py as f32, 0.0];


                    // check if pixel is inside triangle
                    if Self::_is_in_triangle(p, v0, v1, v2) 
                    {

                        let rgb_phong = shader::shader_phong(normals, p, phong_data, i);

                        m_out.push((px as f32, py as f32, v0[2], rgb_phong));
                    }
      
                }

            }


        }

        return m_out;

    }

    fn _is_in_triangle(p: [f32; 3], a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> bool
    {
        // positifs
        let mut check_pos = true;
        check_pos &= Self::_edge_check(p, c, a) > 0.0;
        check_pos &= Self::_edge_check(p, a, b) > 0.0;
        check_pos &= Self::_edge_check(p, b, c) > 0.0;

        // negatifs
        let mut check_neg = true;
        check_neg &= Self::_edge_check(p, c, a) < 0.0;
        check_neg &= Self::_edge_check(p, a, b) < 0.0;
        check_neg &= Self::_edge_check(p, a, c) < 0.0;

        
        return check_pos | check_neg; 

    }

    fn _edge_check(p: [f32; 3], a: [f32; 3], b: [f32; 3]) -> f32
    {
        // calcul du determinant
        return (a[0] - p[0]) * (b[1] - p[1]) - (a[1] - p[1]) * (b[0] - p[0]);
    }

    fn _check_min_max(v0: [f32; 3], v1: [f32; 3], v2: [f32; 3]) -> (i32, i32, i32, i32)
    {
        let max_x = *vec![v0[0] as i32, v1[0] as i32, v2[0] as i32].iter().max().unwrap();
        let min_x = *vec![v0[0] as i32, v1[0] as i32, v2[0] as i32].iter().min().unwrap();

        let max_y = *vec![v0[1] as i32, v1[1] as i32, v2[1] as i32].iter().max().unwrap();
        let min_y = *vec![v0[1] as i32, v1[1] as i32, v2[1] as i32].iter().min().unwrap();

        return (min_x as i32, max_x as i32, min_y as i32, max_y as i32)
    }

}
