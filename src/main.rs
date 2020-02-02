use std::fs::File;
use std::io::{self, prelude::*, BufWriter};
use std::path::Path;
use std::fmt;


#[derive(Debug)]
struct RGBTriplet
{
    red: f64,
    green: f64,
    blue: f64,
}

// impl RGBTriplet
// {
//     fn new(red: u32, green: u32, blue: u32) -> Self
//     {
//         RGBTriplet
//         {
//             red,
//             green, 
//             blue,
//         }
//     }
// }

impl fmt::Display for RGBTriplet
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{} {} {}", self.red.floor(), self.green.floor(), self.blue.floor())
    }
}


fn main() {
    let path = Path::new("pickmaker.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create: {}: {}", display, why),
        Ok(file) => BufWriter::new(file),
    };

    let len = 500;
    let width = 500;
    let depth = 255.0;

    write_ppm(&mut file, len, width, depth).expect("Error writing to file");

}

fn write_ppm(file: &mut BufWriter<File>, nrow: u32, ncol: u32, depth: f64) -> io::Result<()> {
    let header = "P3";

    writeln!(file, "{}", header)?;
    writeln!(file, "{} {} {}", nrow, ncol, depth)?;
    let (rinc, cinc) = (depth / nrow as f64, depth / ncol as f64);
    println!("rinc: {}, cinc: {}", rinc, cinc);

    let mut triplet = RGBTriplet
    {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
    };

    for r in 0..nrow
    {
        for c in 0..ncol
        {
            triplet.green = r as f64 * rinc;
            triplet.red = c as f64 * cinc;
            triplet.blue = r as f64 * rinc / 2.0 + c as f64 * cinc / 2.0;
            write!(file, "{} ", triplet)?;
        }
        writeln!(file)?;
    }
    file.flush()?;
    Ok(())
}
