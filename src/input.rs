use std::error::Error;
use std::fs::{self, File};

fn path_for_day(day: u8) -> Result<fs::DirEntry, Box<dyn Error>> {
    for path in fs::read_dir("input")? {
        let path = path?;
        let fnum = path
            .file_name()
            .to_str()
            .unwrap()
            .split("_")
            .next()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        if fnum == day {
            return Ok(path);
        }
    }
    Err("Couldn't match day")?
}

pub fn open_real_data(day: u8) -> Result<File, Box<dyn Error>> {
    println!("looking for {day}");
    let day_path = path_for_day(day)?;
    let f = File::open(day_path.path())?;
    Ok(f)
}
