use crate::pixel::{blend_64w, dia, diff, is_equal, left2, up2};

//    A1 B1 C1
// A0 A  B  C  C4
// D0 D  E  F  F4
// G0 G  H  I  I4
//    G5 H5 I5

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
pub fn apply_x2(dst_buf: &mut [u32], image: &[u32], width: u32, height: u32) {
  const SCALE: u32 = 2;

  let dst_w = (width * SCALE) as usize;

  for y in 0..height {
    for x in 0..width {
      let kernel = Kernel::new(image, width, height, x, y);

      // All 4 pixels will start with the default pixel value (e).
      // | n0 | n1 |
      // | n2 | n3 |

      let mut e0 = kernel.e;
      let mut e1 = kernel.e;
      let mut e2 = kernel.e;
      let mut e3 = kernel.e;

      sample_x2(kernel.down_right(), &mut e1, &mut e2, &mut e3);
      sample_x2(kernel.up_right(), &mut e0, &mut e3, &mut e1);
      sample_x2(kernel.up_left(), &mut e2, &mut e1, &mut e0);
      sample_x2(kernel.down_left(), &mut e3, &mut e0, &mut e2);

      // // Apply new pixel colors to destination.
      let dst_x = (x * SCALE) as usize;
      let dst_y = (y * SCALE) as usize;

      dst_buf[dst_x + dst_y * dst_w] = e0;
      dst_buf[dst_x + 1 + dst_y * dst_w] = e1;
      dst_buf[dst_x + (dst_y + 1) * dst_w] = e2;
      dst_buf[dst_x + 1 + (dst_y + 1) * dst_w] = e3;
    }
  }
}

fn sample_x2(s: KernelSection, n1: &mut u32, n2: &mut u32, n3: &mut u32) {
  // https://forums.libretro.com/t/xbr-algorithm-tutorial/123

  // Edge Detection Rule (EDR)

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

  // If the weighted distance (wd) among pixels in those red directions are smaller than those in blue,
  // then we can assume there’s a predominant edge in the H-F direction.
  // Here’s the EDR:
  // `wd(red) = d(E,C) + d(E,G) + d(I,F4) + d(I,H5) + 4*d(H,F)`
  // `wd(blue)= d(H,D) + d(H,I5) + d(F,I4) + d(F,B) + 4*d(E,I)`
  // `EDR = (wd(red) < wd(blue))` // bool value

  // Red
  let e =
    diff(s.e, s.c) + diff(s.e, s.g) + diff(s.i, s.h5) + diff(s.i, s.f4) + diff(s.h, s.f) * 4.0;
  // Blue
  let i =
    diff(s.h, s.d) + diff(s.h, s.i5) + diff(s.f, s.i4) + diff(s.f, s.b) + diff(s.e, s.i) * 4.0;

  let px = if diff(s.e, s.f) <= diff(s.e, s.h) {
    s.f
  } else {
    s.h
  };

  // Edge cases
  //
  //    A1 B1 C1
  // A0 A  B\/C  C4
  // D0 D\/E\\F\ F4
  // G0 G/\H\\I \I4
  //    G5 H5\I5
  //
  let edge_cases = !is_equal(s.f, s.b) && !is_equal(s.h, s.d)
    || is_equal(s.e, s.i) && (!is_equal(s.f, s.i4) && !is_equal(s.h, s.i5))
    || is_equal(s.e, s.g)
    || is_equal(s.e, s.c);

  if e < i && edge_cases {
    //    _  _  _
    // _  _  _  Cb _
    // _  _  _  Fa _
    // _  Ga Hb  _  _
    //    _  _  _
    let ke = diff(s.f, s.g);
    let ki = diff(s.h, s.c);
    let ex2 = s.e != s.c && s.b != s.c;
    let ex3 = s.e != s.g && s.d != s.g;

    if (ke * 2.0) <= ki && ex3 || ke >= (ki * 2.0) && ex2 {
      if (ke * 2.0) <= ki && ex3 {
        let left_out = left2(*n3, *n2, px);
        *n3 = left_out[0];
        *n2 = left_out[1];
      }
      if ke >= (ki * 2.0) && ex2 {
        let up_out = up2(*n3, *n1, px);
        *n3 = up_out[0];
        *n1 = up_out[1];
      }
    } else {
      *n3 = dia(*n3, px); // 45°
    }
  } else if e <= i {
    *n3 = blend_64w(*n3, px);
  }
}
