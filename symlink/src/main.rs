use clap::{App, Arg};
use std::io::{Error, ErrorKind};
use std::os::unix::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let matches = App::new("symlink")
        .version("1.0")
        .author("rkd@rkd.me.uk")
        .about("'ln' replacement that's more intuitive")
        .arg(
            Arg::with_name("pointing-to")
                .short("p")
                .long("pointing-to")
                .value_name("FILE")
                .required(true)
                .help("Existing file for symlink to point to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("LINK")
                .help("Symlink file to create")
                .required(true)
                .index(1),
        )
        .get_matches();

    let symlink = Path::new(matches.value_of("LINK").unwrap());
    let pointing_to = Path::new(matches.value_of("pointing-to").unwrap());

    // Convert the source file from a relative to absolute path if necessary.
    let mut path = std::env::current_dir()?;
    let src_file = if pointing_to.is_absolute() {
        &pointing_to
    } else {
        path.push(pointing_to);
        path.as_path()
    };

    if symlink.exists() {
        Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("{} already exists", symlink.display()),
        ))
    } else if !src_file.exists() {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("{} does not exist", src_file.display()),
        ))
    } else {
        fs::symlink(src_file, symlink)
    }
}
