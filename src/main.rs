mod dna;
mod gene;
mod utils;
fn main() {
    let dna1 = dna::DNA::new(512, 4);
    let dna2 = dna::DNA::new(512, 4);
    let dna1str = dna1.to_string();
    // println!("Parent 1: {}", dna1.to_string());
    // println!(
    //     "Parent  : {} {} {}",
    //     dna1.pool_size,
    //     dna1.gene_size,
    //     dna1.genes.len()
    // );
    // println!("Parent 2: {}", dna2.to_string());
    let child = dna::DNA::merge(dna1, dna2, false).unwrap();
    // println!("Child   : {}", child.to_string());
    // let parent = dna::DNA::from(dna1str.clone());
    // println!(
    //     "Parent  : {} {} {}",
    //     parent.pool_size,
    //     parent.gene_size,
    //     parent.genes.len()
    // );
    println!(
        "Child/Parent1: {}",
        dna::DNA::compare(child, dna::DNA::from(dna1str))
    );
}
