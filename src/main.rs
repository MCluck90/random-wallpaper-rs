extern crate rand;
extern crate wallpaper;

use rand::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
  if let Err(e) = run() {
    println!("{}", e);
    std::process::exit(1);
  }
}

fn run() -> Result<(), String> {
  // Known image extensions
  let image_extensions = vec!["bmp", "gif", "jpg", "jpeg", "png"];

  // Get the current wallpaper
  let current_wallpaper = wallpaper::get().map_err(|err| err.to_string())?;
  let current_wallpaper_path = Path::new(&current_wallpaper);
  let current_wallpaper = current_wallpaper_path
    .to_str()
    .ok_or("Path to current wallpaper is not valid Unicode")?;

  // Find the rest of the wallpapers in the same directory
  let wallpaper_dir_path = current_wallpaper_path
    .parent()
    .ok_or("Failed to get image directory. Is your wallpaper the actual root of your system?")?;

  let wallpapers = fs::read_dir(wallpaper_dir_path)
    .map_err(|err| err.to_string())?
    .map(|w| w.unwrap().path())
    // Filter out folders
    .filter(|w| w.is_file())
    // Don't pick the existing wallpaper
    .filter(|w| w.to_str().unwrap() != current_wallpaper)
    // Only pick files with known image extensions
    .filter(|w| {
      if let Some(extension) = w.extension() {
        image_extensions.contains(&extension.to_str().unwrap())
      } else { false }
    })
    .collect::<Vec<PathBuf>>();

  // Make sure at least one file could be found
  if wallpapers.len() == 0 {
    return Err(
      "It appears as though there's only one image in the same folder as \
       your current wallpaper. Could not find a new wallpaper to switch to."
        .to_string(),
    );
  }

  // Pick a random wallpaper
  let mut rng = thread_rng();
  let index = rng.gen_range(0, wallpapers.len());
  let new_wallpaper = &wallpapers[index];

  // Display which one was picked
  println!("{}", new_wallpaper.display());

  // Assign it as the new wallpaper
  wallpaper::set_from_file(new_wallpaper.to_str().unwrap()).map_err(|err| err.to_string())?;

  Ok(())
}
