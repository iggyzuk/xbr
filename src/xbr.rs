use crate::pixel::{self, blend, diff};

//    A1 B1 C1
// A0 A  B  C  C4
// D0 D  E  F  F4
// G0 G  H  I  I4
//    G5 H5 I5

// Matrix: 10 is (0,0) i.e. current pixel.
// 	-2 | -1|  0| +1| +2 	(x)
// ______________________________
// -2 |	    [ 0][ 1][ 2]
// -1 |	[ 3][ 4][ 5][ 6][ 7]
//  0 |	[ 8][ 9][10][11][12]
// +1 |	[13][14][15][16][17]
// +2 |	    [18][19][20]
// (y)|
struct Kernel {
    a1: u32, // 0
    b1: u32, // 1
    c1: u32, // 2
    a0: u32, // 3
    a: u32,  // 4
    b: u32,  // 5
    c: u32,  // 6
    c4: u32, // 7
    d0: u32, // 8
    d: u32,  // 9
    e: u32,  // 10
    f: u32,  // 11
    f4: u32, // 12
    g0: u32, // 13
    g: u32,  // 14
    h: u32,  // 15
    i: u32,  // 16
    i4: u32, // 17
    g5: u32, // 18
    h5: u32, // 19
    i5: u32, // 20
}

impl Kernel {
    fn new(image: &[u32], width: u32, height: u32, x: u32, y: u32) -> Self {
        let src_width = width as i32;
        let src_height = height as i32;

        let x = x as i32;
        let y = y as i32;

        let pixel_at = |x: i32, y: i32| {
            if x < 0 || x >= src_width || y < 0 || y >= src_height {
                0
            } else {
                image[(src_width * y + x) as usize]
            }
        };

        Kernel {
            a1: pixel_at(x - 1, y - 2),
            b1: pixel_at(x, y - 2),
            c1: pixel_at(x + 1, y - 2),
            a0: pixel_at(x - 2, y - 1),
            a: pixel_at(x - 1, y - 1),
            b: pixel_at(x, y - 1),
            c: pixel_at(x + 1, y - 1),
            c4: pixel_at(x + 2, y - 1),
            d0: pixel_at(x - 2, y),
            d: pixel_at(x - 1, y),
            e: pixel_at(x, y),
            f: pixel_at(x + 1, y),
            f4: pixel_at(x + 2, y),
            g0: pixel_at(x - 2, y + 1),
            g: pixel_at(x - 1, y + 1),
            h: pixel_at(x, y + 1),
            i: pixel_at(x + 1, y + 1),
            i4: pixel_at(x + 2, y + 1),
            g5: pixel_at(x - 1, y + 2),
            h5: pixel_at(x, y + 2),
            i5: pixel_at(x + 1, y + 2),
        }
    }
}

struct Weights {
    ed: f32,  // 10_9
    eb: f32,  // 10_5
    ef: f32,  // 10_11
    eh: f32,  // 10_15
    eg: f32,  // 10_14
    ec: f32,  // 10_6
    ad0: f32, // 4_8
    aa1: f32, // 4_1
    db: f32,  // 9_5
    dh: f32,  // 9_15
    da0: f32, // 9_3
    bf: f32,  // 5_11
    ba1: f32, // 5_0
    ea: f32,  // 10_4
    ei: f32,  // 10_16
    cf4: f32, // 6_12
    cb1: f32, // 6_1
    fh: f32,  // 11_15
    fc4: f32, // 11_7
    bc1: f32, // 5_2
    gd0: f32, // 14_8
    gh5: f32, // 14_19
    hg5: f32, // 15_18
    dg0: f32, // 9_13
    if4: f32, // 16_12
    ih5: f32, // 16_19
    hi5: f32, // 15_20
    hi4: f32, // 15_17
}

impl Weights {
    fn new(kernel: &Kernel) -> Self {
        Weights {
            ed: diff(kernel.e, kernel.d),
            eb: diff(kernel.e, kernel.b),
            ef: diff(kernel.e, kernel.f),
            eh: diff(kernel.e, kernel.h),
            eg: diff(kernel.e, kernel.g),
            ec: diff(kernel.e, kernel.c),
            ad0: diff(kernel.a, kernel.d0),
            aa1: diff(kernel.a, kernel.a1),
            db: diff(kernel.d, kernel.b),
            dh: diff(kernel.d, kernel.h),
            da0: diff(kernel.d, kernel.a0),
            bf: diff(kernel.b, kernel.f),
            ba1: diff(kernel.b, kernel.a1),
            ea: diff(kernel.e, kernel.a),
            ei: diff(kernel.e, kernel.i),
            cf4: diff(kernel.c, kernel.f4),
            cb1: diff(kernel.c, kernel.b1),
            fh: diff(kernel.f, kernel.h),
            fc4: diff(kernel.f, kernel.c4),
            bc1: diff(kernel.b, kernel.c1),
            gd0: diff(kernel.g, kernel.d0),
            gh5: diff(kernel.g, kernel.h5),
            hg5: diff(kernel.h, kernel.g5),
            dg0: diff(kernel.d, kernel.g0),
            if4: diff(kernel.i, kernel.f4),
            ih5: diff(kernel.i, kernel.h5),
            hi5: diff(kernel.h, kernel.i5),
            hi4: diff(kernel.h, kernel.i4),
        }
    }
}

/// Applies the xBR filter.
pub fn apply(buf: &mut [u32], image: &[u32], width: u32, height: u32) {
    const SCALE: u32 = 2;

    let scaled_width = width * SCALE;

    for y in 0..height {
        for x in 0..width {
            let kernel = Kernel::new(image, width, height, x, y);
            let w = Weights::new(&kernel);

            let a1 = w.eg + w.ec + w.ad0 + w.aa1 + 4.0 * w.db;
            let b1 = w.dh + w.da0 + w.bf + w.ba1 + 4.0 * w.ea;

            // let idx = ((y * SCALE) * scaled_width) + (x * SCALE);
            // buf[idx as usize] = kernel.e & 0xFF0000FF;
            // let idx = ((y * SCALE) * scaled_width) + (x * SCALE + 1);
            // buf[idx as usize] = kernel.e & 0xFF0000FF;
            // let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE);
            // buf[idx as usize] = kernel.e & 0xFF0000FF;
            // let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE + 1);
            // buf[idx as usize] = kernel.e & 0xFF0000FF;

            // Top Left Edge Detection Rule
            let idx = ((y * SCALE) * scaled_width) + (x * SCALE);
            buf[idx as usize] = if a1 < b1 {
                let new_pixel = if w.ed <= w.eb { kernel.d } else { kernel.b };
                let blended_pixel = blend(new_pixel, kernel.e, 0.5);
                blended_pixel
            } else {
                kernel.e
            };

            // Top Right Edge Detection Rule
            let a2 = w.ei + w.ea + w.cf4 + w.cb1 + 4.0 * w.bf;
            let b2 = w.fh + w.fc4 + w.db + w.bc1 + 4.0 * w.ec;
            let idx = ((y * SCALE) * scaled_width) + (x * SCALE + 1);
            buf[idx as usize] = if a2 < b2 {
                let new_pixel = if w.eb <= w.ef { kernel.b } else { kernel.f };
                let blended_pixel = blend(new_pixel, kernel.e, 0.5);
                blended_pixel
            } else {
                kernel.e
            };

            // Bottom Left Edge Detection Rule
            let a3 = w.ea + w.ei + w.gd0 + w.gh5 + 4.0 * w.dh;
            let b3 = w.db + w.dg0 + w.fh + w.hg5 + 4.0 * w.eg;
            let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE);
            buf[idx as usize] = if a3 < b3 {
                let new_pixel = if w.ed <= w.eh { kernel.d } else { kernel.h };
                let blended_pixel = blend(new_pixel, kernel.e, 0.5);
                blended_pixel
            } else {
                kernel.e
            };

            // Bottom Right Edge Detection Rule
            let a4 = w.ec + w.eg + w.if4 + w.ih5 + 4.0 * w.fh;
            let b4 = w.dh + w.hi5 + w.hi4 + w.bf + 4.0 * w.ei;
            let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE + 1);
            buf[idx as usize] = if a4 < b4 {
                let new_pixel = if w.ef <= w.eh { kernel.f } else { kernel.h };
                let blended_pixel = blend(new_pixel, kernel.e, 0.5);
                blended_pixel
            } else {
                kernel.e
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
