use std::str;
use crate::gene::Gene;
use arrayvec::ArrayVec;

pub struct DNA {
    pub pool_size: u16,
    pub gene_size: u16,
    pub genes: Vec<Gene>
}

impl DNA {
    pub fn new(pool_size: u16, gene_size: u16) -> DNA {
        DNA {
            pool_size: pool_size,
            gene_size: gene_size,
            genes: (0..pool_size).map(|_| Gene::new(gene_size)).collect()
        }
    }
    pub fn to_string(&self) -> String {
        let pool_size_hex: String = self.pool_size
            .to_be_bytes()
            .iter()
            .map(|val| format!("{:0>2x}", val))
            .collect();
        let gene_size_hex: String = self.gene_size
            .to_be_bytes()
            .iter()
            .map(|val| format!("{:0>2x}", val))
            .collect();

        format!("{}{}{}", pool_size_hex, gene_size_hex,
            self.genes.iter().map(|m| m.to_string()).collect::<String>())
    }
}

impl std::convert::From<DNA> for String {
    fn from(dna: DNA) -> String {
        let pool_size_hex: String = dna.pool_size
            .to_be_bytes()
            .iter()
            .map(|val| format!("{:0>2x}", val))
            .collect();
        let gene_size_hex: String = dna.gene_size
            .to_be_bytes()
            .iter()
            .map(|val| format!("{:0>2x}", val))
            .collect();

        format!("{}{}{}", pool_size_hex, gene_size_hex,
            dna.genes.iter().map(|m| m.to_string()).collect::<String>())
    }
}

impl std::convert::From<String> for DNA {
    fn from(dna: String) -> DNA {
        let pool_size_hex = &dna[0..4];
        let gene_size_hex = &dna[4..8];
        let genes_hex = &dna[8..];

        let decoded_pool_size: ArrayVec<_> = pool_size_hex.as_bytes()
            .chunks(2)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .iter()
            .map(|c| u8::from_str_radix(c, 16).unwrap())
            .collect::<ArrayVec<_>>();
        let decoded_pool_size_array: [u8; 2] = decoded_pool_size.into_inner().unwrap();
        let pool_size = u16::from_be_bytes(decoded_pool_size_array);

        let gene_size_size: ArrayVec<_> = gene_size_hex.as_bytes()
            .chunks(2)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .iter()
            .map(|c| u8::from_str_radix(c, 16).unwrap())
            .collect::<ArrayVec<_>>();
        let gene_size_size_array: [u8; 2] = gene_size_size.into_inner().unwrap();
        let gene_size = u16::from_be_bytes(gene_size_size_array);
        let genes = genes_hex.as_bytes()
            .chunks(8 * gene_size as usize)
            .map(|x| str::from_utf8(x).unwrap())
            .map(String::from)
            .collect::<Vec<String>>();

        DNA {
            pool_size: pool_size,
            gene_size: gene_size,
            genes: genes.iter().map(|g| Gene::from(g.clone())).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_be_converted_and_back() {
        let dna = DNA::new(2, 2);
        let dna2 = DNA::from(dna.to_string());
        assert_eq!(dna.to_string(), dna2.to_string());
    }
}
