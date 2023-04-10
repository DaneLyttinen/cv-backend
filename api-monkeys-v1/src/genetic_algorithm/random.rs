use rand::{Rng};
use rand::distributions::Alphanumeric;
pub(crate) fn gen_random_float() -> f64{
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..1.0);
}

pub(crate) fn gen_random_num_in_range(a: u32, b : u32) -> u32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(a..b);
}

pub(crate) fn gen_random_char() -> String{
    let rng = rand::thread_rng();
    let s = unsafe {
        String::from_utf8_unchecked(
            rng
                .sample_iter(&Alphanumeric)
                .take(1)
                .collect::<Vec<_>>(),
        )};
    return s;
}

pub(crate) fn proportional_random(weights: &[u32], sum: u32) -> u32{
    let mut val = gen_random_float() * sum as f64;
    for weight in weights {
        let dereferenced_int = *weight;
        if val < dereferenced_int as f64 {
            return dereferenced_int;
        }
        val -= dereferenced_int as f64;
    }
    return 0;
}