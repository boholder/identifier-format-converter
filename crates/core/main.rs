use std::process;
mod app;

fn main() {
    let matches = app::app().get_matches();
    let files = matches.values_of_lossy("files");
    // let raw_words = match files {
    //     None => { extractor::read_from_std_in() }
    //     Some(files) => { extractor::read_from_files(files) }
    // };
    // println!("{}", raw_words.unwrap());
    process::exit(0);
}