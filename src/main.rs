use std::{fs::File, io};
use io::Write;

mod cb;
mod offset;
mod deflate;
mod ni;

fn main() -> io::Result<()> {
    const FILE: &'static [u8] = include_bytes!("../examples/TESTINSTRUMENTBEST.nki");

    match ni::read(FILE) {
        Ok(f) => println!("done\n{:?}", f.0.len()),
        Err(e) => println!("error: {:?}", e)
    }

    // match deflate::deflate(FILE, 1113) {
    //     Ok((_, content)) => {
    //         let mut buffer = File::create("booga2.deflate")?;
    //         buffer.write_all(&content)?;
    //         println!("done!");
    //     },
    //     Err(e) => println!("error: {:?}", e)
    // }
    Ok(())
}
