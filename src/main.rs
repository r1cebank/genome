mod gene;
mod dna;

fn main() {
    let dna = dna::DNA::new(2, 2);
    let dna2 = dna::DNA::from(dna.to_string());
    println!("Original Value: {:?}", dna.to_string());
    println!("Restored Value: {:?}", dna2.to_string());
}
