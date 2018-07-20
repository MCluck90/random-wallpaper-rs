# Random Wallpaper

A little utility I wrote for myself to pick a random wallpaper.

## How does it work?

First, it finds where your current wallpaper exists. Then it looks for other images<sup>1</sup> in that same directory and picks one at random. That randomly selected image is now your wallpaper.

## How to use it

You'll need to have [Rust installed.](https://www.rust-lang.org/en-US/install.html)

```bash
$ git clone https://github.com/MCluck90/random-wallpaper-rs.git
$ cd random-wallpaper-rs
$ cargo run --release
```

---

1: Images are defined as files which end with one of the following extensions: .bmp, .gif, .jpg, .jpeg, .png. It does not check if these are valid files.