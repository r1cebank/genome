mod gene;

use rand::prelude::*;
use rand_distr::StandardNormal;

fn main() {
    let mut rng = rand::thread_rng();
    let val: f64 = rng.sample(StandardNormal);
    let gene = gene::Gene::new();
    println!("Original Value: {:?}", gene.markers.value);
}
