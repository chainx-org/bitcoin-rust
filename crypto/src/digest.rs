// Copyright 2018 Chainpool

use std::iter::repeat;

pub trait Digest {
    fn input(&mut self, input: &[u8]);
    fn result(&mut self, out: &mut [u8]);
    fn reset(&mut self);
    fn output_bits(&self) -> usize;

    fn output_bytes(&self) -> usize {
        (self.output_bits() + 7) / 8
    }

    fn block_size(&self) -> usize;


    fn input_str(&mut self, input: &str) {
        self.input(input.as_bytes());
    }

    fn result_str(&mut self) -> String {
        use serialize::hex::ToHex;

        let mut buf: Vec<u8> = repeat(0).take((self.output_bits()+7)/8).collect();
        self.result(&mut buf);
        buf[..].to_hex()
    }

}
