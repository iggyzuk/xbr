# xBR x2 Filter in Rust

Pixel Art Upscaler

## Checklist

- Make into a library
- x3 and x4 versions
- Resizing
- Indexed PNG format

## Examples

![](/assets/in2.png)
![](/assets/out2.png)

![](/assets/in3.png)
![](/assets/out3.png)

![](/assets/in1.png)
![](/assets/out1.png)

## Run Command

```cargo run --release -- ./assets/in1.png ./assets/out1.png```

## Resources

### Projects

- https://github.com/mukaschultze/xBR-rs
- https://github.com/joseprio/xBRjs
- https://github.com/carlosascari/2xBR-Filter

### Resizing

- https://github.com/Cykooz/fast_image_resize
- https://github.com/irokaru/pixel-scaler/blob/master/src/lib/FileUtil.js#L100

```json
[dependencies]
image = "0.23.0"
``` 

```rust 
use image::{DynamicImage, GenericImageView, imageops};

fn resize_image(image: DynamicImage, new_width: u32, new_height: u32) -> DynamicImage {
    image.resize_exact(new_width, new_height, imageops::FilterType::Triangle)
}

fn main() {
    // Open the image using the `image` crate
    let image = image::open("path/to/your/image.jpg").expect("Failed to open image");

    // Set the desired new width and height
    let new_width = 800;
    let new_height = 600;

    // Resize the image using linear sampling
    let resized_image = resize_image(image, new_width, new_height);

    // Save the resized image to a file
    resized_image.save("path/to/save/resized_image.jpg").expect("Failed to save image");
}
```