use clap::{Arg, App, ArgSettings, crate_name, crate_authors, crate_description, crate_version};

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(Arg::new("roots")
            .long("roots")
            .about("set media file's roots")
            .settings(&[ArgSettings::MultipleOccurrences, ArgSettings::TakesValue])
            .required(true))
        .get_matches();
    let roots: Vec<&str> = matches.values_of("roots").unwrap().collect();
    for root in roots {
        println!("root: {}", root)
    }
}
