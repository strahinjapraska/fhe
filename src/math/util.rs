use rand::Rng;
use rug::Integer;
use crate::math::{finite_field::modulo};
use rug::Complete;


pub fn random_binary_vector(n: usize) -> Vec<Integer>{

    let mut rng = rand::thread_rng(); 

    (0..n)
    .map(|_| {
            let num = rng.gen_range(0..=1);
            Integer::from(num)
        })
    .collect()

}

pub fn scale(c: &Vec<Integer>, p: &Integer, t: &Integer) -> Vec<Integer>{
    let half_p = (p / &Integer::from(2)).complete();
    c.iter().map(|a| {
        let tx = (t * a).complete();

        let y = if tx >= 0 {
            (tx + &half_p) / p
        } else {
            (tx - &half_p) / p
        };

        modulo(&y, p)
    }).collect()
}
