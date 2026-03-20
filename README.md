# mPNGp

Build with `cargo build --release` ([cargo](https://www.rust-lang.org/fr/learn/get-started) needs to be installed)

```
Usage: target/release/mPNGp FILE [options]

Options:
    -h, --help          print this help menu
    -s, --size SIZE     display the image at a larger size than 1 pixel per pixel
```

Black and white pictures will be displayed in the terminal, a graphical window will be opened for color ones. You may want to use the `-s 20` option (with a similar number) to scale up the pictures in the window, as the examples are very low-resolution.

Beware, the terminal output may display Wayland debug information when closing the window. This is because the crate I use for graphics has not completely impemented Wayland, but it works fine.