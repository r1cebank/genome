mod gene;
mod dna;

fn main() {
    let mut gene = gene::Gene::new(2);
    println!("Before: {}", gene.to_string());
    gene.mutate();
    println!("After:  {}", gene.to_string());
}
