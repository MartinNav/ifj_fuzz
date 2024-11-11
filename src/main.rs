use std::char;

use rand::{rngs::ThreadRng, RngCore};

fn generate_random_str_of_rn_len(rng:&mut ThreadRng)->String{
    const MAX_LEN:usize=6000;
    let len:usize = (rng.next_u32()as usize)%MAX_LEN;
    let mut res = String::with_capacity(len);
    for _ in 0..len {
        res.push(char::from_u32(rng.next_u32()).unwrap_or('.'));
    }
    res
}
fn main() {
    let mut num_of_cysles=6669;
    let mut rng = rand::thread_rng();
    loop {
        let random_chars =generate_random_str_of_rn_len(&mut rng);
        
        if num_of_cysles==0 {
            break;
        }
        num_of_cysles-=1;
    }
}
