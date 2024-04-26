use codeowners_rs::parser::parse_file;
use std::io;
use std::path::PathBuf;

fn usage() -> ! {
    let name = std::env::args().next().unwrap_or("gh-codeowner".into());
    eprintln!("{} <CODEOWNERS> <file>", name);
    std::process::exit(1);
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let Some((codeowners, file)) = args.next().zip(args.next()) else {
        usage();
    };

    let path = PathBuf::from(codeowners);
    let ruleset = parse_file(path.as_path())?.into_ruleset();
    if let Some(owners) = ruleset.owners(file) {
        for owner in owners {
            println!("{}", owner.value);
        }
    };

    Ok(())
}
