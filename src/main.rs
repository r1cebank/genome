mod dna;
mod gene;
mod utils;

fn main() {
    let dna = dna::DNA::new(1, 1);
    let dna_str = String::from(dna);
    println!("{}", dna_str);
    let dna_copy = dna::DNA::from(dna_str);
    println!("{}", dna_copy.to_string());
}
