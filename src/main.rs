mod dna;
mod gene;

fn main() {
    let dna1 = dna::DNA::new(3, 2);
    let dna2 = dna::DNA::new(3, 2);
    println!("Parent 1: {}", dna1.to_string());
    println!("Parent 2: {}", dna2.to_string());
    let child = dna::DNA::merge(dna1, dna2);
    println!("Child   : {}", child.unwrap().to_string());
}
