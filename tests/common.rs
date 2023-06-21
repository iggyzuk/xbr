#[path = "../src/xbr.rs"]
mod xbr;
#[path = "../src/pixel.rs"]
mod pixel;

#[test]
fn test() {
    // xbr::apply(buf, image, width, height)
    let d = pixel::diff(0, 2);
    println!("{d}");
    assert_eq!(d, 10.0);
    // println!("Diff is: {d}");
}
