use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("/Users/paulgrant/Dev/rosalind_rust/transcribing_RNA_DNA/rosalind_rna.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{}", contents);

    let output = contents.replace("T", "U");

    println!("{}", output);

    Ok(())
}