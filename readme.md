# xBR x2 Filter in Rust

Pixel Art Upscaler

## Checklist

- Add docs
- Add tests
- Add x3 and x4 versions
- Allow resizing: e.g. x2.7 uses x3 and scales down a little

## Examples

`cargo run --example process_image --features="image" --release`

![](/examples/assets/input1.png)
![](/examples/assets/output1.png)

![](/examples/assets/input2.png)
![](/examples/assets/output2.png)

![](/examples/assets/input3.png)
![](/examples/assets/output3.png)

Can be used in realtime to help with anti-aliasing.

![](/examples/assets/realtime.gif)

## Resources

### Projects

- <https://github.com/mukaschultze/xBR-rs>
- <https://github.com/joseprio/xBRjs>
- <https://github.com/carlosascari/2xBR-Filter>

### Resizing

- <https://github.com/Cykooz/fast_image_resize>
- <https://github.com/irokaru/pixel-scaler/blob/master/src/lib/FileUtil.js#L100>
