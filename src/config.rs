use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::cli::{Color, Opts};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub moments: Vec<Moment>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Moment {
    pub start_date: String,
    pub format: String,
}

/// Reads the configuration from a file and returns a `Config` struct.
pub fn load(opts: &Opts) -> Result<Config, crate::Error> {
    set_color(opts);

    let default_path = &default_path()?;
    let config_path = match &opts.config {
        Some(path) => path,
        None => default_path,
    };

    let file_content =
        fs::read_to_string(config_path).map_err(|_| crate::Error::ConfigNotFound {
            path: config_path.to_path_buf(),
        })?;
    let config = ron::from_str::<Config>(&file_content);
    match config {
        Ok(conf) => Ok(conf),
        Err(e) => Err(crate::Error::InvalidConfig {
            message: e.to_string(),
        }),
    }
}

/// Sets the color mode for the application
fn set_color(opts: &Opts) {
    match opts.color {
        Color::Always => {
            owo_colors::set_override(true);
        }
        Color::Never => {
            owo_colors::set_override(false);
        }
        Color::Auto => {
            owo_colors::unset_override();
        }
    };
}

/// Return configuration path
fn default_path() -> Result<PathBuf, crate::Error> {
    let path = if cfg!(windows) {
        Path::new(&std::env::var("APPDATA")?)
            .join("Nai")
            .join("config.ron")
    } else {
        Path::new(&std::env::var("HOME")?)
            .join(".config")
            .join("nai")
            .join("config.ron")
    };
    Ok(path)
}
