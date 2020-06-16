use crate::gene::Gene;
use crate::utils;
use arrayvec::ArrayVec;
use rand::prelude::*;

pub struct DNA {
    pub pool_size: u16,
    pub gene_size: u16,
    pub genes: Vec<Gene>,
}

impl DNA {
    pub fn new(pool_size: u16, gene_size: u16) -> DNA {
        DNA {
            pool_size: pool_size,
            gene_size: gene_size,
            genes: (0..pool_size).map(|_| Gene::new(gene_size)).collect(),
        }
    }
    pub fn merge(left_dna: DNA, right_dna: DNA, mutate: bool) -> Option<DNA> {
        let mut ration_rng = thread_rng();
        let mut mutate_rng = thread_rng();
        match (left_dna.pool_size == right_dna.pool_size)
            && (left_dna.gene_size == right_dna.gene_size)
        {
            true => Some(DNA {
                pool_size: left_dna.pool_size,
                gene_size: left_dna.gene_size,
                genes: (0..left_dna.pool_size)
                    .map(|i| {
                        if ration_rng.gen::<f32>() >= 0.5 {
                            left_dna.genes[i as usize].to_string()
                        } else {
                            right_dna.genes[i as usize].to_string()
                        }
                    })
                    .map(|g| {
                        let mut gene = Gene::from(g);
                        if mutate && mutate_rng.gen::<f32>() >= 0.9 {
                            gene.mutate();
                        }
                        gene
                    })
                    .collect(),
            }),
            false => None,
        }
    }
    pub fn compare(left_dna: DNA, right_dna: DNA) -> f64 {
        let mut same_markers = 0;
        if left_dna.pool_size != right_dna.pool_size {
            return 0 as f64;
        }
        (0..left_dna.pool_size).for_each(|i| {
            if left_dna.genes[i as usize].to_string() == right_dna.genes[i as usize].to_string() {
                same_markers = same_markers + 1;
            }
        });
        same_markers as f64 / left_dna.pool_size as f64
    }
    pub fn to_latent_vec(&self) -> Vec<f32> {
        self.genes
            .iter()
            .map(|g| g.markers.iter().map(|m| m.value).collect::<Vec<f32>>())
            .collect::<Vec<Vec<f32>>>()
            .concat()
    }
    pub fn to_string(&self) -> String {
        let pool_size_hex: String = utils::u16_to_string(self.pool_size);
        let gene_size_hex: String = utils::u16_to_string(self.gene_size);

        format!(
            "{}{}{}",
            pool_size_hex,
            gene_size_hex,
            self.genes.iter().map(|m| m.to_string()).collect::<String>()
        )
    }
}

impl std::convert::From<DNA> for String {
    fn from(dna: DNA) -> String {
        let pool_size_hex: String = utils::u16_to_string(dna.pool_size);
        let gene_size_hex: String = utils::u16_to_string(dna.gene_size);

        format!(
            "{}{}{}",
            pool_size_hex,
            gene_size_hex,
            dna.genes.iter().map(|m| m.to_string()).collect::<String>()
        )
    }
}

impl std::convert::From<String> for DNA {
    fn from(dna: String) -> DNA {
        let pool_size_hex = &dna[0..4];
        let gene_size_hex = &dna[4..8];
        let genes_hex = &dna[8..];

        let decoded_pool_size: ArrayVec<_> = utils::partition_str(pool_size_hex, 2)
            .iter()
            .map(|c| u8::from_str_radix(c, 16).unwrap())
            .collect::<ArrayVec<_>>();
        let decoded_pool_size_array: [u8; 2] = decoded_pool_size.into_inner().unwrap();
        let pool_size = u16::from_be_bytes(decoded_pool_size_array);

        let gene_size_size: ArrayVec<_> = utils::partition_str(gene_size_hex, 2)
            .iter()
            .map(|c| u8::from_str_radix(c, 16).unwrap())
            .collect::<ArrayVec<_>>();
        let gene_size_size_array: [u8; 2] = gene_size_size.into_inner().unwrap();
        let gene_size = u16::from_be_bytes(gene_size_size_array);
        let genes = utils::partition_str(genes_hex, 8 * (gene_size + 1) as usize)
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();

        DNA {
            pool_size: pool_size,
            gene_size: gene_size,
            genes: genes.iter().map(|g| Gene::from(g.clone())).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_be_converted_and_back() {
        let dna = DNA::new(4, 2);
        let dna2 = DNA::from(dna.to_string());
        assert_eq!(dna.gene_size, dna2.gene_size);
        assert_eq!(dna.pool_size, dna2.pool_size);
        assert_eq!(dna.genes.len(), dna2.genes.len());
        assert_eq!(dna.to_string(), dna2.to_string());
    }
    #[test]
    fn can_be_merged() {
        let dna1 = DNA::new(2, 2);
        let dna2 = DNA::new(2, 2);
        match DNA::merge(dna1, dna2, false) {
            Some(_) => assert!(true),
            None => assert!(false),
        };
    }
    #[test]
    fn cannot_be_merged() {
        let dna1 = DNA::new(2, 2);
        let dna2 = DNA::new(3, 2);
        match DNA::merge(dna1, dna2, false) {
            Some(_) => assert!(false),
            None => assert!(true),
        };
    }
    #[test]
    fn check_merged_gene_ratio() {
        let dna1 = DNA::new(512, 4);
        let dna2 = DNA::new(512, 4);
        let dna1str = dna1.to_string();
        let dna2str = dna2.to_string();
        let child = DNA::merge(dna1, dna2, false).unwrap();
        let child_str = child.to_string();
        let parent1_ratio = DNA::compare(DNA::from(child_str.clone()), DNA::from(dna1str));
        let parent2_ratio = DNA::compare(DNA::from(child_str), DNA::from(dna2str));
        assert!(parent1_ratio != 0 as f64);
        assert!(parent2_ratio != 0 as f64);
        assert!(parent1_ratio + parent2_ratio == 1 as f64);
    }
}
