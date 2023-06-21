// use crate::pixel::diff;

// //    A1 B1 C1
// // A0 A  B  C  C4
// // D0 D  E  F  F4
// // G0 G  H  I  I4
// //    G5 H5 I5

// // Matrix: 10 is (0,0) i.e. current pixel.
// // 	-2 | -1|  0| +1| +2 	(x)
// // ______________________________
// // -2 |	    [ 0][ 1][ 2]
// // -1 |	[ 3][ 4][ 5][ 6][ 7]
// //  0 |	[ 8][ 9][10][11][12]
// // +1 |	[13][14][15][16][17]
// // +2 |	    [18][19][20]
// // (y)|
// pub struct Kernel {
//     pub a1: u32, // 0
//     pub b1: u32, // 1
//     pub c1: u32, // 2
//     pub a0: u32, // 3
//     pub a: u32,  // 4
//     pub b: u32,  // 5
//     pub c: u32,  // 6
//     pub c4: u32, // 7
//     pub d0: u32, // 8
//     pub d: u32,  // 9
//     pub e: u32,  // 10
//     pub f: u32,  // 11
//     pub f4: u32, // 12
//     pub g0: u32, // 13
//     pub g: u32,  // 14
//     pub h: u32,  // 15
//     pub i: u32,  // 16
//     pub i4: u32, // 17
//     pub g5: u32, // 18
//     pub h5: u32, // 19
//     pub i5: u32, // 20
// }

// impl Kernel {
//     pub fn new(image: &[u32], width: u32, height: u32, x: u32, y: u32) -> Self {
//         let width = width as i32;
//         let height = height as i32;
//         let x = x as i32;
//         let y = y as i32;

//         let pixel_at = |x: i32, y: i32| {
//             if x < 0 || x >= width || y < 0 || y >= height {
//                 0
//             } else {
//                 image[(width * y + x) as usize]
//             }
//         };

//         Kernel {
//             a1: pixel_at(x - 1, y - 2),
//             b1: pixel_at(x, y - 2),
//             c1: pixel_at(x + 1, y - 2),
//             a0: pixel_at(x - 2, y - 1),
//             a: pixel_at(x - 1, y - 1),
//             b: pixel_at(x, y - 1),
//             c: pixel_at(x + 1, y - 1),
//             c4: pixel_at(x + 2, y - 1),
//             d0: pixel_at(x - 2, y),
//             d: pixel_at(x - 1, y),
//             e: pixel_at(x, y),
//             f: pixel_at(x + 1, y),
//             f4: pixel_at(x + 2, y),
//             g0: pixel_at(x - 2, y + 1),
//             g: pixel_at(x - 1, y + 1),
//             h: pixel_at(x, y + 1),
//             i: pixel_at(x + 1, y + 1),
//             i4: pixel_at(x + 2, y + 1),
//             g5: pixel_at(x - 1, y + 2),
//             h5: pixel_at(x, y + 2),
//             i5: pixel_at(x + 1, y + 2),
//         }
//     }
// }

// pub struct Weights {
//     pub ed: f32,  // 10_9
//     pub eb: f32,  // 10_5
//     pub ef: f32,  // 10_11
//     pub eh: f32,  // 10_15
//     pub eg: f32,  // 10_14
//     pub ec: f32,  // 10_6
//     pub ad0: f32, // 4_8
//     pub aa1: f32, // 4_1
//     pub db: f32,  // 9_5
//     pub dh: f32,  // 9_15
//     pub da0: f32, // 9_3
//     pub bf: f32,  // 5_11
//     pub ba1: f32, // 5_0
//     pub ea: f32,  // 10_4
//     pub ei: f32,  // 10_16
//     pub cf4: f32, // 6_12
//     pub cb1: f32, // 6_1
//     pub fh: f32,  // 11_15
//     pub fc4: f32, // 11_7
//     pub bc1: f32, // 5_2
//     pub gd0: f32, // 14_8
//     pub gh5: f32, // 14_19
//     pub hg5: f32, // 15_18
//     pub dg0: f32, // 9_13
//     pub if4: f32, // 16_12
//     pub ih5: f32, // 16_19
//     pub hi5: f32, // 15_20
//     pub hi4: f32, // 15_17
// }

// impl Weights {
//     pub fn new(kernel: &Kernel) -> Self {
//         Weights {
//             ed: diff(kernel.e, kernel.d),
//             eb: diff(kernel.e, kernel.b),
//             ef: diff(kernel.e, kernel.f),
//             eh: diff(kernel.e, kernel.h),
//             eg: diff(kernel.e, kernel.g),
//             ec: diff(kernel.e, kernel.c),
//             ad0: diff(kernel.a, kernel.d0),
//             aa1: diff(kernel.a, kernel.a1),
//             db: diff(kernel.d, kernel.b),
//             dh: diff(kernel.d, kernel.h),
//             da0: diff(kernel.d, kernel.a0),
//             bf: diff(kernel.b, kernel.f),
//             ba1: diff(kernel.b, kernel.a1),
//             ea: diff(kernel.e, kernel.a),
//             ei: diff(kernel.e, kernel.i),
//             cf4: diff(kernel.c, kernel.f4),
//             cb1: diff(kernel.c, kernel.b1),
//             fh: diff(kernel.f, kernel.h),
//             fc4: diff(kernel.f, kernel.c4),
//             bc1: diff(kernel.b, kernel.c1),
//             gd0: diff(kernel.g, kernel.d0),
//             gh5: diff(kernel.g, kernel.h5),
//             hg5: diff(kernel.h, kernel.g5),
//             dg0: diff(kernel.d, kernel.g0),
//             if4: diff(kernel.i, kernel.f4),
//             ih5: diff(kernel.i, kernel.h5),
//             hi5: diff(kernel.h, kernel.i5),
//             hi4: diff(kernel.h, kernel.i4),
//         }
//     }
// }
