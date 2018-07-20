extern crate wallpaper;

use std::fs;
use std::path::Path;

fn main() {
  let current_wallpaper = wallpaper::get().unwrap();
  let current_wallpaper_path = Path::new(&current_wallpaper);
  let wallpaper_dir_path = current_wallpaper_path.parent().unwrap();
  let wallpapers = fs::read_dir(wallpaper_dir_path)
    .unwrap()
    .map(|w| w.unwrap().path())
    .filter(|w| w.is_file());

  for w in wallpapers {
    println!("{}", w.display());
  }
}
