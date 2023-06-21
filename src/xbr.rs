use crate::pixel::{self, diff, is_equal};

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

//    _  _  _
// _  _  B  _  _
// _  D  E  F F4
// _  G  H  I I4
//    _  H5 I5
struct KernelSection {
    e: u32,
    i: u32,
    h: u32,
    f: u32,
    g: u32,
    c: u32,
    d: u32,
    b: u32,
    f4: u32,
    i4: u32,
    h5: u32,
    i5: u32,
}

impl Kernel {
    fn new(image: &[u32], width: u32, height: u32, x: u32, y: u32) -> Self {
        let width = width as i32;
        let height = height as i32;
        let x = x as i32;
        let y = y as i32;

        let pixel_at = |x: i32, y: i32| {
            if x < 0 || x >= width || y < 0 || y >= height {
                0
            } else {
                image[(width * y + x) as usize]
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

    //    _  _  _
    // _  _  B  _  _
    // _  D  E  F F4
    // _  G  H  I I4
    //    _  H5 I5
    fn down_right(&self) -> KernelSection {
        KernelSection {
            e: self.e,
            i: self.i,
            h: self.h,
            f: self.f,
            g: self.g,
            c: self.c,
            d: self.d,
            b: self.b,
            f4: self.f4,
            i4: self.i4,
            h5: self.h5,
            i5: self.i5,
        }
    }

    //    _  B1 C1
    // _  PA PB PC C4
    // _  PD PE PF F4
    // _  _  PH PI _
    //    _  _ _
    fn up_right(&self) -> KernelSection {
        KernelSection {
            e: self.e,
            i: self.c,
            h: self.f,
            f: self.b,
            g: self.i,
            c: self.a,
            d: self.h,
            b: self.d,
            f4: self.b1,
            i4: self.c1,
            h5: self.f4,
            i5: self.c4,
        }
    }

    //    A1  B1 _
    // A0 PA PB PC  _
    // D0 PD PE PF _
    // _  PG PH _  _
    //    _  _ _
    fn up_left(&self) -> KernelSection {
        KernelSection {
            e: self.e,
            i: self.a,
            h: self.b,
            f: self.d,
            g: self.c,
            c: self.g,
            d: self.f,
            b: self.h,
            f4: self.d0,
            i4: self.a0,
            h5: self.b1,
            i5: self.a1,
        }
    }

    //    _  _  _
    // _  PA PB _  _
    // D0 PD PE PF _
    // G0 PG PH _  _
    //    G5 H5 _
    fn down_left(&self) -> KernelSection {
        KernelSection {
            e: self.e,
            i: self.g,
            h: self.d,
            f: self.h,
            g: self.a,
            c: self.i,
            d: self.b,
            b: self.f,
            f4: self.h5,
            i4: self.g5,
            h5: self.d0,
            i5: self.g0,
        }
    }
}

/// Applies the xBR filter.
pub fn apply(dst_buf: &mut [u32], image: &[u32], width: u32, height: u32) {
    const SCALE: u32 = 2;

    let dst_w = (width * SCALE) as usize;

    for y in 0..height {
        for x in 0..width {
            let k = Kernel::new(image, width, height, x, y);

            // Weights are useless
            // let w = Weights::new(&kernel);

            // let idx = ((y * SCALE) * scaled_width) + (x * SCALE);
            // buf[idx as usize] = kernel.e & 0xFF0000FF;
            // let idx = ((y * SCALE) * scaled_width) + (x * SCALE + 1);
            // buf[idx as usize] = kernel.e & 0xFF0000FF;
            // let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE);
            // buf[idx as usize] = kernel.e & 0xFF0000FF;
            // let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE + 1);
            // buf[idx as usize] = kernel.e & 0xFF0000FF;

            // // Top Left Edge Detection Rule
            // let idx = ((y * SCALE) * scaled_width) + (x * SCALE);
            // buf[idx as usize] = if a1 < b1 {
            //     let new_pixel = if w.ed <= w.eb { kernel.d } else { kernel.b };
            //     let blended_pixel = blend(new_pixel, kernel.e, 0.5);
            //     blended_pixel
            // } else {
            //     kernel.e
            // };

            // // Top Right Edge Detection Rule
            // let a2 = w.ei + w.ea + w.cf4 + w.cb1 + 4.0 * w.bf;
            // let b2 = w.fh + w.fc4 + w.db + w.bc1 + 4.0 * w.ec;
            // let idx = ((y * SCALE) * scaled_width) + (x * SCALE + 1);
            // buf[idx as usize] = if a2 < b2 {
            //     let new_pixel = if w.eb <= w.ef { kernel.b } else { kernel.f };
            //     let blended_pixel = blend(new_pixel, kernel.e, 0.5);
            //     blended_pixel
            // } else {
            //     kernel.e
            // };

            // // Bottom Left Edge Detection Rule
            // let a3 = w.ea + w.ei + w.gd0 + w.gh5 + 4.0 * w.dh;
            // let b3 = w.db + w.dg0 + w.fh + w.hg5 + 4.0 * w.eg;
            // let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE);
            // buf[idx as usize] = if a3 < b3 {
            //     let new_pixel = if w.ed <= w.eh { kernel.d } else { kernel.h };
            //     let blended_pixel = blend(new_pixel, kernel.e, 0.5);
            //     blended_pixel
            // } else {
            //     kernel.e
            // };

            // // Bottom Right Edge Detection Rule
            // let a4 = w.ec + w.eg + w.if4 + w.ih5 + 4.0 * w.fh;
            // let b4 = w.dh + w.hi5 + w.hi4 + w.bf + 4.0 * w.ei;
            // let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE + 1);
            // buf[idx as usize] = if a4 < b4 {
            //     let new_pixel = if w.ef <= w.eh { kernel.f } else { kernel.h };
            //     let blended_pixel = blend(new_pixel, kernel.e, 0.5);
            //     blended_pixel
            // } else {
            //     kernel.e
            // };

            // let mut sp = SuperPixel::new(kernel.e);

            // let r = sample(&kernel, &w, sp.e1, sp.e2, sp.e3);
            // sp.e1 = r[0];

            // // Top Left Edge Detection Rule
            // let a1 = w.eg + w.ec + w.ad0 + w.aa1 + 4.0 * w.db;
            // let b1 = w.dh + w.da0 + w.bf + w.ba1 + 4.0 * w.ea;
            // sp.e1 = if a1 < b1 {
            //     let new_pixel = if w.ed <= w.eb { kernel.d } else { kernel.b };
            //     let blended_pixel = blend(new_pixel, kernel.e, 0.5);
            //     blended_pixel
            // } else {
            //     kernel.e
            // };

            // // Top Right Edge Detection Rule
            // let a2 = w.ei + w.ea + w.cf4 + w.cb1 + 4.0 * w.bf;
            // let b2 = w.fh + w.fc4 + w.db + w.bc1 + 4.0 * w.ec;
            // sp.e2 = if a2 < b2 {
            //     let new_pixel = if w.eb <= w.ef { kernel.b } else { kernel.f };
            //     let blended_pixel = blend(new_pixel, kernel.e, 0.5);
            //     blended_pixel
            // } else {
            //     kernel.e
            // };

            // // Bottom Left Edge Detection Rule
            // let a3 = w.ea + w.ei + w.gd0 + w.gh5 + 4.0 * w.dh;
            // let b3 = w.db + w.dg0 + w.fh + w.hg5 + 4.0 * w.eg;
            // sp.e3 = if a3 < b3 {
            //     let new_pixel = if w.ed <= w.eh { kernel.d } else { kernel.h };
            //     let blended_pixel = blend(new_pixel, kernel.e, 0.5);
            //     blended_pixel
            // } else {
            //     kernel.e
            // };

            // // Bottom Right Edge Detection Rule
            // let a4 = w.ec + w.eg + w.if4 + w.ih5 + 4.0 * w.fh;
            // let b4 = w.dh + w.hi5 + w.hi4 + w.bf + 4.0 * w.ei;
            // sp.e4 = if a4 < b4 {
            //     let new_pixel = if w.ef <= w.eh { kernel.f } else { kernel.h };
            //     let blended_pixel = blend(new_pixel, kernel.e, 0.5);
            //     blended_pixel
            // } else {
            //     kernel.e
            // };

            // let idx = ((y * SCALE) * scaled_width) + (x * SCALE);
            // buf[idx as usize] = sp.e1;
            // let idx = ((y * SCALE) * scaled_width) + (x * SCALE + 1);
            // buf[idx as usize] = sp.e2;
            // let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE);
            // buf[idx as usize] = sp.e3;
            // let idx = ((y * SCALE + 1) * scaled_width) + (x * SCALE + 1);
            // buf[idx as usize] = sp.e4;

            // All 4 pixels will start the default pixel (e).
            let mut e0 = k.e;
            let mut e1 = k.e;
            let mut e2 = k.e;
            let mut e3 = k.e;

            kernel2xv5(k.down_right(), &mut e1, &mut e2, &mut e3);
            kernel2xv5(k.up_right(), &mut e0, &mut e3, &mut e1);
            kernel2xv5(k.up_left(), &mut e2, &mut e1, &mut e0);
            kernel2xv5(k.down_left(), &mut e3, &mut e0, &mut e2);

            // // Apply new pixel colors to destination.
            let dst_x = (x * 2) as usize;
            let dst_y = (y * 2) as usize;

            dst_buf[dst_x + dst_y * dst_w] = e0;
            dst_buf[dst_x + 1 + dst_y * dst_w] = e1;
            dst_buf[dst_x + (dst_y + 1) * dst_w] = e2;
            dst_buf[dst_x + 1 + (dst_y + 1) * dst_w] = e3;
        }
    }
}

fn kernel2xv5(s: KernelSection, n1: &mut u32, n2: &mut u32, n3: &mut u32) {
    // | n0 | n1 |
    // | n2 | n3 |

    // 1) Edge Detection Rule (EDR)

    // There's 4 diagonal rotations, we'll focus on down-right edge.
    // Consider a pixel E and its neighbors like the configuration below:

    //    A1 B1 C1
    // A0 A  B  C  C4
    // D0 D  E  F  F4
    // G0 G  H  I  I4
    //    G5 H5 I5

    // We have to know if there’s an edge along pixels H and F.
    // If yes, then pixel E must be interpolated.
    // If no, then the predominant edge is along pixels E and I,
    // so that E doesn’t need to be interpolated.

    let ex = s.e == s.h || s.e == s.f;
    if ex {
        return;
    }

    // result[0] = 0xFF0000FF;
    // result[1] = 0x00FF00FF;
    // result[2] = 0x0000FFFF;

    // return result;

    // If the weighted distance (wd) among pixels in those red directions are smaller than those in blue,
    // then we can assume there’s a predominant edge in the H-F direction.
    // Here’s the EDR:
    // wd(red) = d(E,C) + d(E,G) + d(I,F4) + d(I,H5) + 4*d(H,F)
    // wd(blue)= d(H,D) + d(H,I5) + d(F,I4) + d(F,B) + 4*d(E,I)
    // EDR = (wd(red) < wd(blue))
    // EDR is a bool variable.

    let e =
        diff(s.e, s.c) + diff(s.e, s.g) + diff(s.i, s.h5) + diff(s.i, s.f4) + diff(s.h, s.f) * 4.0; // Red
    let i =
        diff(s.h, s.d) + diff(s.h, s.i5) + diff(s.f, s.i4) + diff(s.f, s.b) + diff(s.e, s.i) * 4.0; // Blue

    let px = if diff(s.e, s.f) <= diff(s.e, s.h) {
        s.f
    } else {
        s.h
    };

    if e < i
        && (!is_equal(s.f, s.b) && !is_equal(s.h, s.d)
            || is_equal(s.e, s.i) && (!is_equal(s.f, s.i4) && !is_equal(s.h, s.i5))
            || is_equal(s.e, s.g)
            || is_equal(s.e, s.c))
    {
        let ke = diff(s.f, s.g);
        let ki = diff(s.h, s.c);
        let ex2 = s.e != s.c && s.b != s.c;
        let ex3 = s.e != s.g && s.d != s.g;

        if (ke * 2.0) <= ki && ex3 || ke >= (ki * 2.0) && ex2 {
            if (ke * 2.0) <= ki && ex3 {
                let left_out = left2_2x(*n3, *n2, px);
                *n3 = left_out[0];
                *n2 = left_out[1];
                // result[2] = 0x00FF0000; // red
                // result[1] = 0x00FFAA00; // red-ish
            }
            if ke >= (ki * 2.0) && ex2 {
                let up_out = up2_2x(*n3, *n1, px);
                *n3 = up_out[0];
                *n1 = up_out[1];
                // result[2] = 0x000000FF;
                // result[0] = 0x00FF00FF;
            }
        } else {
            // Looks like: 45°
            *n3 = dia_2x(*n3, px);
            // result[2] = 0x0000FFFF;
        }
    } else if e <= i {
        *n3 = alpha_blend_64w(*n3, px);
        // result[2] = 0x00FFFF00;
    }
}

/// 64 = 64
fn alpha_blend_64w(dst: u32, src: u32) -> u32 {
    // return 0x00FF00FF;
    return pixel::blend(dst, src, 3.0, 1.0);
}

/// 64 + 64 = 128
fn alpha_blend_128w(dst: u32, src: u32) -> u32 {
    // return 0xFF0000FF;
    return pixel::blend(dst, src, 1.0, 1.0);
}

/// 128 + 64 = 192
fn alpha_blend_192w(dst: u32, src: u32) -> u32 {
    // return 0x0000FFFF;
    return pixel::blend(dst, src, 1.0, 2.0);
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
