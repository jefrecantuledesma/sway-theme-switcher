use clap::{App, Arg};
use dirs;
//use image::io::Reader as ImageReader;
use std::fs;
use std::io::{self, Write};

fn main() {
    let matches = App::new("Sway Config Switcher")
        .version("1.0")
        .author("Bai Senyou")
        .about("A CLI application made to help you switch between config files and wallpapers.")
        .arg(
            Arg::with_name("list-configs")
                .long("list-configs")
                .help("List available config files"),
        )
        .arg(
            Arg::with_name("select-config")
                .long("select-config")
                .takes_value(true)
                .help("Select a config file by number"),
        )
        .arg(
            Arg::with_name("setup")
                .short("setup")
                .long("setup")
                .help("Setup initial configuration"),
        )
        .arg(
            Arg::with_name("list-wallpapers")
                .long("list-wallpapers")
                .help("List available wallpapers"),
        )
        .arg(
            Arg::with_name("select-wallpaper")
                .long("select-wallpaper")
                .takes_value(true)
                .help("Select a wallpaper by number"),
        )
        .get_matches();

    if matches.is_present("list-configs") {
        list_configs();
    } else if let Some(selected) = matches.value_of("select-config") {
        select_config(selected.parse().unwrap());
    } else if matches.is_present("setup") {
        setup_config();
    } else if matches.is_present("list-wallpapers") {
        list_wallpapers();
    } else if let Some(selected) = matches.value_of("select-wallpaper") {
        select_wallpaper(selected.parse().unwrap());
    }
}

fn list_wallpapers() {
    let wallpapers_path = dirs::home_dir()
        .expect("Failed to determine home directory")
        .join(".config/sway/wallpapers");

    if let Ok(entries) = fs::read_dir(&wallpapers_path) {
        for (index, entry) in entries.enumerate() {
            if let Ok(entry) = entry {
                println!("{}. {}", index + 1, entry.file_name().to_string_lossy());
            }
        }
    }
}

fn select_wallpaper(selected: usize) {
    let wallpapers_path = dirs::home_dir()
        .expect("Failed to determine home directory")
        .join(".config/sway/wallpapers");

    if let Ok(entries) = fs::read_dir(&wallpapers_path) {
        let selected_entry = entries
            .enumerate()
            .find(|(index, _)| *index == selected - 1)
            .map(|(_, entry)| entry)
            .and_then(|entry| entry.ok()); // Extract the Ok value from the Result

        let target_path = dirs::home_dir()
            .expect("Failed to determine home directory")
            .join(".config/sway");

        let target_wallpaper_path = target_path.join("wallpaper.png");

        // Delete the old wallpaper if it exists
        let old_wallpaper_path = target_path.join("wallpaper.png");
        if old_wallpaper_path.exists() {
            if let Err(e) = fs::remove_file(&old_wallpaper_path) {
                eprintln!("Error deleting old wallpaper: {}", e);
            }
        }

        if let Some(entry) = selected_entry {
            if let Err(e) = fs::copy(entry.path(), &target_wallpaper_path) {
                eprintln!("Error copying wallpaper: {}", e);
                return;
            }
        }
    }
}

fn list_configs() {
    let config_files_path = dirs::home_dir()
        .expect("Failed to determine home directory")
        .join(".config/sway/config_files");

    if let Ok(entries) = fs::read_dir(&config_files_path) {
        for (index, entry) in entries.enumerate() {
            if let Ok(entry) = entry {
                println!("{}. {}", index + 1, entry.file_name().to_string_lossy());
            }
        }
    }
}

fn select_config(selected: usize) {
    let config_files_path = dirs::home_dir()
        .expect("Failed to determine home directory")
        .join(".config/sway/config_files");

    if let Ok(entries) = fs::read_dir(&config_files_path) {
        let selected_entry = entries
            .enumerate()
            .find(|(index, _)| *index == selected - 1)
            .map(|(_, entry)| entry)
            .and_then(|entry| entry.ok()); // Extract the Ok value from the Result

        if let Some(entry) = selected_entry {
            let target_path = dirs::home_dir()
                .expect("Failed to determine home directory")
                .join(".config/sway/config");

            if let Err(e) = fs::copy(entry.path(), &target_path) {
                eprintln!("Error copying config file: {}", e);
            }
        }
    }
}

fn setup_config() {
    let config_files_path = dirs::home_dir()
        .expect("Failed to determine home directory")
        .join(".config/sway/config_files");

    if !config_files_path.exists() {
        if let Err(e) = fs::create_dir_all(&config_files_path) {
            eprintln!("Error creating config files directory: {}", e);
            return;
        }
    }

    let current_config_path = dirs::home_dir()
        .expect("Failed to determine home directory")
        .join(".config/sway/config");

    let new_config_name = {
        let mut input = String::new();
        print!("Enter a name for the current config: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    };

    let new_config_path = config_files_path.join(&new_config_name);

    if let Err(e) = fs::copy(&current_config_path, &new_config_path) {
        eprintln!("Error copying config file: {}", e);
        return;
    }
}
