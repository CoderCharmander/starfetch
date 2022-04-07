use std::{fmt::Write, io::stdout};

use ansi_term::{Style, Color};
use serde::Deserialize;
use anyhow::Result;

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
                writeln!(target, "      {}: {}", Color::White.bold().paint(title), info)?;
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

fn main() {
    Constellation {
        title: "───── ａｒｉｅｓ ─────".into(),
        graph: vec![
            Star(4, 2, '✦'),
            Star(14, 4, '✦'),
            Star(16, 6, '✦'),
            Star(15, 7, '✦'),
        ],
        name: "Aries".into(),
        quadrant: "NQ1".into(),
        right_ascension: "01h 46m 37.3761s –03h 29m 42.4003s".into(),
        declination: "+31.2213154° to –10.3632069°".into(),
        area: "441 sq.deg. (39th)".into(),
        main_stars: "4, 9".into(),
    }.render(&mut stdout()).unwrap();
}
