use std::{
    ffi::OsStr,
    fs::{self, File},
    io::stdout,
    path::{Path, PathBuf},
};

use ansi_term::Color;
use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};
use rand::prelude::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
struct Star(u8, u8, char);

#[derive(Deserialize)]
struct Constellation {
    title: String,
    graph: Vec<Star>,
    name: String,
    quadrant: String,
    right_ascension: String,
    declination: String,
    area: String,
    main_stars: String,
}

impl Constellation {
    fn render<T: std::io::Write>(&self, target: &mut T) -> Result<()> {
        writeln!(target, "┌{}┐", self.title)?;
        let mut lines = [[' '; 22]; 10];
        for star in &self.graph {
            lines[star.1 as usize][star.0 as usize] = star.2;
        }

        for (i, line) in lines.iter().enumerate() {
            write!(target, "│{}│", line.iter().collect::<String>())?;

            let mut write_info = |title: &str, info: &str| -> Result<()> {
                writeln!(
                    target,
                    "      {}: {}",
                    Color::White.bold().paint(title),
                    info
                )?;
                Ok(())
            };

            match i {
                1 => writeln!(target, "      {}", Color::White.bold().paint(&self.name))?,
                3 => write_info("Quadrant", &self.quadrant)?,
                4 => write_info("Right ascension", &self.right_ascension)?,
                5 => write_info("Declination", &self.declination)?,
                6 => write_info("Area", &self.area)?,
                7 => write_info("Main stars", &self.main_stars)?,
                _ => writeln!(target)?,
            }
        }

        writeln!(target, "└──────────────────────┘")?;
        Ok(())
    }
}

#[derive(Parser)]
#[clap(group(ArgGroup::new("action").required(true).args(&["name", "list", "random", "data-directory"])))]
struct Config {
    /// Set the path where constellations are loaded from
    #[clap(short, long)]
    asset_path: Option<PathBuf>,

    /// Show a random constellation
    #[clap(short, long)]
    random: bool,

    /// Name of the constellation to show
    name: Option<String>,

    /// List all constellations
    #[clap(short, long)]
    list: bool,

    /// Print the directory where constellations will be read from.
    #[clap(short, long)]
    data_directory: bool,
}

fn fetch_constellation(constellations_path: &Path, name: &str) -> Result<Constellation> {
    Ok(serde_json::from_reader(
        File::open(constellations_path.join(format!("{}.json", name)))
            .context("The provided constellation does not exist or is not readable")?,
    )
    .context("The provided constellation can not be parsed")?)
}

fn main() -> Result<()> {
    let config = Config::parse();

    let asset_path = config
        .asset_path
        .or_else(|| Some(dirs::data_dir()?.join("starfetch")))
        .context("No valid resource directory was found")?;

    let constellations_path = asset_path.join("constellations");

    if !constellations_path.is_dir() {
        fs::create_dir_all(&constellations_path)?;
    }

    if let Some(name) = config.name {
        fetch_constellation(&constellations_path, &name)?.render(&mut stdout())?;
    } else if config.random || config.list {
        let mut names = vec![];
        // Fetch the names of all constellations
        for p in constellations_path.read_dir()? {
            let p = p?;
            if p.file_type()?.is_file() && p.path().extension() == Some(OsStr::new("json")) {
                names.push(
                    p.path()
                        .file_stem()
                        .context("Yea this should never happen")?
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }

        if config.random {
            let name: &str = names
                .choose(&mut rand::thread_rng())
                .context("Constellations directory is empty")?
                .as_ref();
            fetch_constellation(&constellations_path, name)?.render(&mut stdout())?;
        } else if config.list {
            for c in &names {
                let constellation = fetch_constellation(&constellations_path, c)?;
                println!(
                    "{} - {} ({})",
                    Color::White.bold().paint(c),
                    constellation.name,
                    constellation.quadrant
                );
            }
        }
    } else if config.data_directory {
        println!(
            "The data directory is {}. Copy constellation files here.",
            Color::White
                .bold()
                .paint(constellations_path.to_string_lossy())
        )
    }

    Ok(())
}
