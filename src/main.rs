extern crate rand;
extern crate wallpaper;

use rand::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
  // Get the current wallpaper
  let current_wallpaper = wallpaper::get().unwrap();
  let current_wallpaper_path = Path::new(&current_wallpaper);
  let current_wallpaper = current_wallpaper_path.to_str().unwrap();

  // Find the rest of the wallpapers in the same directory
  let wallpaper_dir_path = current_wallpaper_path.parent().unwrap();
  let wallpapers = fs::read_dir(wallpaper_dir_path)
    .unwrap()
    .map(|w| w.unwrap().path())
    // Filter out folders and the current wallpaper
    .filter(|w| w.is_file() && w.to_str().unwrap() != current_wallpaper)
    .collect::<Vec<PathBuf>>();

  // Pick a random wallpaper
  let mut rng = thread_rng();
  let index = rng.gen_range(0, wallpapers.len());
  let new_wallpaper = &wallpapers[index];
  // Display which one was picked
  println!("{}", new_wallpaper.display());

  // Assign it as the new wallpaper
  wallpaper::set_from_file(new_wallpaper.to_str().unwrap()).unwrap();
}
