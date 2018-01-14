use std::env;
use std::fs;
use std::path;

extern crate clap;
use clap::{Arg, App};

fn update() -> Result<(), Box<::std::error::Error>> {
    let target = self_update::get_target()?;
    self_update::backends::github::Update::configure()?
        .repo_owner("SirVer")
        .repo_name("relto")
        .target(&target)
        .bin_name("relto")
        .show_download_progress(true)
        .show_output(false)
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    Ok(())
}

fn make_absolute(path: &path::Path, current_dir: &path::Path) -> path::PathBuf {
    let absolute_path = if path.is_absolute() {
        path.to_owned()
    } else {
        current_dir.join(path)
    };
    fs::canonicalize(&absolute_path).unwrap_or(absolute_path)
}

fn main() {
    let matches = App::new("relto")
        .version("1.0")
        .author("Holger Rapp <HolgerRapp@gmx.net>")
        .about("Prints input paths relative to a directory.")
        .arg(Arg::with_name("DIR")
            .short("d")
            .long("dir")
            .help("Directory to find relative path to. [.]")
            .takes_value(true))
        .arg(Arg::with_name("paths")
            .multiple(true)
            .help("The input paths."))
        .arg(Arg::with_name("update")
             .long("update")
             .help("Update binary in-place from latest release")
        .get_matches();

    if matches.is_present("update") {
        update.unwrap();
        return;
    }

    let paths = matches.values_of("paths");
    if paths.is_none() {
        return;
    }

    let current_dir = env::current_dir().unwrap();
    let dir = make_absolute(&matches.value_of("DIR")
                                .map(|s| path::PathBuf::from(s))
                                .unwrap_or(current_dir.clone()),
                            &current_dir);

    for path in paths.unwrap() {
        let path = path::Path::new(path);
        let absolute_path = make_absolute(&path, &current_dir);
        let print_path: &path::Path = match absolute_path.strip_prefix(&dir) {
            Ok(path) => path,
            Err(_) => &absolute_path,
        };
        println!("{}", print_path.to_string_lossy());
    }
}
