use rand::{RngExt};

pub struct Mutator {
    rng: rand::rngs::ThreadRng,
}

impl Mutator {
    pub fn new() -> Self {
        Mutator {
            rng: rand::rng(),
        }
    }

    pub fn mutate(&mut self,input : &[u8]) -> Vec<u8>{
        match self.rng.random_range(0..6){
            0 => self.bit_flip(input),
            1 => self.byte_flip(input),
            2 => self.append(input),
            3 => self.truncate(input),
            4 => self.repeat(input),
            _ => self.insert_known(input)
        }
    }

    fn bit_flip(&mut self, input: &[u8]) -> Vec<u8> {
        let mut out = input.to_vec();
        if out.is_empty() { return out; }
        let byte_idx = self.rng.random_range(0..out.len());
        let bit_idx = self.rng.random_range(0..8u8);
        out[byte_idx] ^= 1 << bit_idx;
        out
    }

    fn byte_flip(&mut self,input : &[u8]) -> Vec<u8>{
        let mut out = input.to_vec();
        if out.is_empty() {return out};
        let byte_idx = self.rng.random_range(0..out.len());
        out[byte_idx] ^= 0xFF;
        out
    }

    fn truncate(&mut self,input : &[u8]) -> Vec<u8>{
        if input.len() <=1 {return  input.to_vec()};
        let new_len = self.rng.random_range(1..input.len());
        input[..new_len].to_vec()
    }

    fn append(&mut self, input : &[u8]) -> Vec<u8>{
        let mut out = input.to_vec();
        let random_value_append = self.rng.random_range(1..64);
        for _ in 0..random_value_append{
            out.push(self.rng.random_range(0u8..=255))
        }
        out
    }

    fn repeat(&mut self,input : &[u8]) -> Vec<u8>{
        if input.is_empty() { return input.to_vec(); }
        let mut out = input.to_vec();
        let start = self.rng.random_range(0..input.len());
        let end = self.rng.random_range(start..input.len());
        let chunk = input[start..end].to_vec();
        let times = self.rng.random_range(2..10);
        for _ in 0..times {
            for byte in &chunk{
                out.push(*byte);
            }
        }
        out
    }

    fn insert_known(&mut self,input : &[u8]) -> Vec<u8>{
        let known: &[&[u8]] = &[
            b"/../../../etc/passwd",
            b";ls;",
            b"$(id)",
            b"%n%n%n%n",
            b"\x00\x00\x00\x00",
            b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            b"999999999999999999",
            b"\xff\xfe\xfd",
        ];
        let byte_to_append = self.rng.random_range(0..known.len());

        let pos = self.rng.random_range(0..=input.len());
        let bad = known[byte_to_append];

        let mut out = input[..pos].to_vec();
        out.extend_from_slice(&bad);
        out.extend_from_slice(&input[pos..]);

        out
    }
}