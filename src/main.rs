use std::env;
use std::io::{BufRead, BufReader, BufWriter};
use std::fs::File;
use std::fs;

use epub_builder::EpubBuilder;
use epub_builder::Result;
use epub_builder::ZipLibrary;
use epub_builder::EpubContent;
use epub_builder::ReferenceType;
use epub_builder::TocElement;
use epub_builder::ZipCommand;

use std::io;
use std::io::Write;

// Try to print Zip file to stdout
fn run() -> Result<()> {

    let f = File::create("test.epub").unwrap();
    let mut writer = BufWriter::new(f);

    // Some dummy content to fill our books
    let dummy_content = fs::read_to_string("resources/c02.xhtml").unwrap();

    // Create a new EpubBuilder using the zip library
    EpubBuilder::new(ZipCommand::new()?)?

    // Set some metadata
        .metadata("author", "bionic writer")?
        .metadata("title", "bionic writing test")?

    // Add a chapter, mark it as beginning of the "real content"
        .add_content(EpubContent::new("chapter_1.xhtml", dummy_content.as_bytes())
                    .title("Chapter 1")
                    .reftype(ReferenceType::Text))?

    // Generate a toc inside of the document, that will be part of the linear structure.
        .inline_toc()

    // Finally, write the EPUB file using file writer
        .generate(&mut writer)?;
        
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {
            writeln!(&mut io::stderr(),
                     "Successfully wrote epub document to stdout!")
                .unwrap();
        }
        Err(err) => writeln!(&mut io::stderr(), "Error: {}", err).unwrap(),
    };

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("converting file {} to bionic writing", filename);

    let reader = BufReader::new(File::open(filename).expect("Cannot open file.txt"));

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            println!("word '{}'", word);
            // find size of word
            let size = word.len();
            println!("size of string: {}", size);
            let chars = word.chars();
            let bionic_stop_index = size / 2 + 1;

            let char_vec = chars.collect::<Vec<char>>();

            let (bionic_str, remainder_str) = char_vec.split_at(bionic_stop_index);
            let bionic_str: String = bionic_str.into_iter().collect();
            let remainder_str: String = remainder_str.into_iter().collect();

            let new_word = format!("**{}**{}", bionic_str, remainder_str);
            println!("{}", new_word);
        }
    }
}
