use crate::pixel::{self, blend, diff};

/// Applies the xBR filter.
pub fn apply(buf: &mut [u32], image: &[u32], width: u32, height: u32) {
    const SCALE: i32 = 2;

    let src_width = width as i32;
    let src_height = height as i32;
    let scaled_width = src_width * SCALE;

    let pixel_at = |x: i32, y: i32| {
        if x < 0 || x >= src_width || y < 0 || y >= src_height {
            0
        } else {
            image[(src_width * y + x) as usize]
        }
    };

    let matrix = &mut [0; 21];

    for y in 0..src_height {
        for x in 0..src_width {
            // Matrix: 10 is (0,0) i.e. current pixel.
            // 	-2 | -1|  0| +1| +2 	(x)
            // ______________________________
            // -2 |	    [ 0][ 1][ 2]
            // -1 |	[ 3][ 4][ 5][ 6][ 7]
            //  0 |	[ 8][ 9][10][11][12]
            // +1 |	[13][14][15][16][17]
            // +2 |	    [18][19][20]
            // (y)|

            matrix[0] = pixel_at(x - 1, y - 2);
            matrix[1] = pixel_at(x, y - 2);
            matrix[2] = pixel_at(x + 1, y - 2);
            matrix[3] = pixel_at(x - 2, y - 1);
            matrix[4] = pixel_at(x - 1, y - 1);
            matrix[5] = pixel_at(x, y - 1);
            matrix[6] = pixel_at(x + 1, y - 1);
            matrix[7] = pixel_at(x + 2, y - 1);
            matrix[8] = pixel_at(x - 2, y);
            matrix[9] = pixel_at(x - 1, y);
            matrix[10] = pixel_at(x, y);
            matrix[11] = pixel_at(x + 1, y);
            matrix[12] = pixel_at(x + 2, y);
            matrix[13] = pixel_at(x - 2, y + 1);
            matrix[14] = pixel_at(x - 1, y + 1);
            matrix[15] = pixel_at(x, y + 1);
            matrix[16] = pixel_at(x + 1, y + 1);
            matrix[17] = pixel_at(x + 2, y + 1);
            matrix[18] = pixel_at(x - 1, y + 2);
            matrix[19] = pixel_at(x, y + 2);
            matrix[20] = pixel_at(x + 1, y + 2);

            // Calculate color weights using 2 points in the matrix
            let d_10_9 = diff(matrix[10], matrix[9]);
            let d_10_5 = diff(matrix[10], matrix[5]);
            let d_10_11 = diff(matrix[10], matrix[11]);
            let d_10_15 = diff(matrix[10], matrix[15]);
            let d_10_14 = diff(matrix[10], matrix[14]);
            let d_10_6 = diff(matrix[10], matrix[6]);
            let d_4_8 = diff(matrix[4], matrix[8]);
            let d_4_1 = diff(matrix[4], matrix[1]);
            let d_9_5 = diff(matrix[9], matrix[5]);
            let d_9_15 = diff(matrix[9], matrix[15]);
            let d_9_3 = diff(matrix[9], matrix[3]);
            let d_5_11 = diff(matrix[5], matrix[11]);
            let d_5_0 = diff(matrix[5], matrix[0]);
            let d_10_4 = diff(matrix[10], matrix[4]);
            let d_10_16 = diff(matrix[10], matrix[16]);
            let d_6_12 = diff(matrix[6], matrix[12]);
            let d_6_1 = diff(matrix[6], matrix[1]);
            let d_11_15 = diff(matrix[11], matrix[15]);
            let d_11_7 = diff(matrix[11], matrix[7]);
            let d_5_2 = diff(matrix[5], matrix[2]);
            let d_14_8 = diff(matrix[14], matrix[8]);
            let d_14_19 = diff(matrix[14], matrix[19]);
            let d_15_18 = diff(matrix[15], matrix[18]);
            let d_9_13 = diff(matrix[9], matrix[13]);
            let d_16_12 = diff(matrix[16], matrix[12]);
            let d_16_19 = diff(matrix[16], matrix[19]);
            let d_15_20 = diff(matrix[15], matrix[20]);
            let d_15_17 = diff(matrix[15], matrix[17]);

            let a1 = d_10_14 + d_10_6 + d_4_8 + d_4_1 + 4.0 * d_9_5;
            let b1 = d_9_15 + d_9_3 + d_5_11 + d_5_0 + 4.0 * d_10_4;
            
            let idx = ((y * SCALE) * scaled_width) + (x * SCALE);
            buf[idx as usize] = matrix[10] & 0xFF0000FF;
            let idx = ((y * SCALE) * scaled_width) + (x * SCALE + 1);
            buf[idx as usize] = matrix[10] & 0xFF0000FF;
            let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE);
            buf[idx as usize] = matrix[10] & 0xFF0000FF;
            let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE + 1);
            buf[idx as usize] = matrix[10] & 0xFF0000FF;
            
            // Top Left Edge Detection Rule
            let idx = ((y * SCALE) * scaled_width) + (x * SCALE);
            buf[idx as usize] = if a1 < b1 {
                let new_pixel = if d_10_9 <= d_10_5 {
                    matrix[9]
                } else {
                    matrix[5]
                };
                let blended_pixel = blend(new_pixel, matrix[10], 0.5);
                blended_pixel
            } else {
                matrix[10]
            };
        

            // Top Right Edge Detection Rule
            let a2 = d_10_16 + d_10_4 + d_6_12 + d_6_1 + 4.0 * d_5_11;
            let b2 = d_11_15 + d_11_7 + d_9_5 + d_5_2 + 4.0 * d_10_6;
            let idx = ((y * SCALE) * scaled_width) + (x * SCALE + 1);
            buf[idx as usize] = if a2 < b2 {
                let new_pixel = if d_10_5 <= d_10_11 {
                    matrix[5]
                } else {
                    matrix[11]
                };
                let blended_pixel = blend(new_pixel, matrix[10], 0.5);
                blended_pixel
            } else {
                matrix[10]
            };

            // Bottom Left Edge Detection Rule
            let a3 = d_10_4 + d_10_16 + d_14_8 + d_14_19 + 4.0 * d_9_15;
            let b3 = d_9_5 + d_9_13 + d_11_15 + d_15_18 + 4.0 * d_10_14;
            let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE);
            buf[idx as usize] = if a3 < b3 {
                let new_pixel = if d_10_9 <= d_10_15 {
                    matrix[9]
                } else {
                    matrix[15]
                };
                let blended_pixel = blend(new_pixel, matrix[10], 0.5);
                blended_pixel
            } else {
                matrix[10]
            };

            // Bottom Right Edge Detection Rule
            let a4 = d_10_6 + d_10_14 + d_16_12 + d_16_19 + 4.0 * d_11_15;
            let b4 = d_9_15 + d_15_20 + d_15_17 + d_5_11 + 4.0 * d_10_16;
            let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE + 1);
            buf[idx as usize] = if a4 < b4 {
                let new_pixel = if d_10_11 <= d_10_15 {
                    matrix[11]
                } else {
                    matrix[15]
                };
                let blended_pixel = blend(new_pixel, matrix[10], 0.5);
                blended_pixel
            } else {
                matrix[10]
            };
        }
    }
}

// fn kernel2xv5(
//     pe: u32,
//     pi: u32,
//     ph: u32,
//     pf: u32,
//     pg: u32,
//     pc: u32,
//     pd: u32,
//     pb: u32,
//     f4: u32,
//     i4: u32,
//     h5: u32,
//     i5: u32,
//     n1: u32,
//     n2: u32,
//     n3: u32,
//     blend_colors: bool,
//     scale_alpha: bool,
// ) -> (u32, u32, u32) {
//     // | n0 | n1 |
//     // | n2 | n3 |

//     let mut result: (n1, n2, n3);

//     // Bottom Right Edge Detection Rule
//     let a4 = d_10_6 + d_10_14 + d_16_12 + d_16_19 + 4.0 * d_11_15;
//     let b4 = d_9_15 + d_15_20 + d_15_17 + d_5_11 + 4.0 * d_10_16;
//     let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE + 1);
//     buf[idx as usize] = if a4 < b4 {
//         let new_pixel = if d_10_11 <= d_10_15 {
//             matrix[11]
//         } else {
//             matrix[15]
//         };
//         let blended_pixel = blend(new_pixel, matrix[10], 0.5);
//         blended_pixel
//     } else {
//         pe
//     };
// }

fn alpha_blend_64w(dst: u32, src: u32) -> u32 {
    // return 0xFF00FF00;
    return pixel::blend_exp(dst, src, 3.0, 1.0);
}

// 64 + 64 = 128
fn alpha_blend_128w(dst: u32, src: u32) -> u32 {
    // return 0xFFFF0000;
    return pixel::blend_exp(dst, src, 1.0, 1.0);
}

// 128 + 64 = 192
fn alpha_blend_192w(dst: u32, src: u32) -> u32 {
    // return 0xFF0000FF;
    return pixel::blend_exp(dst, src, 1.0, 2.0);
}

fn left2_2x(n3: u32, n2: u32, pixel: u32) -> [u32; 2] {
    [alpha_blend_192w(n3, pixel), alpha_blend_64w(n2, pixel)]
}

fn up2_2x(n3: u32, n1: u32, pixel: u32) -> [u32; 2] {
    [alpha_blend_192w(n3, pixel), alpha_blend_64w(n1, pixel)]
}

fn dia_2x(n3: u32, pixel: u32) -> u32 {
    alpha_blend_128w(n3, pixel)
}
