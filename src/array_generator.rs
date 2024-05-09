use rand::{distributions::Standard, prelude::*};

pub fn array_generator<T>() -> Vec<usize>
where
    Standard: Distribution<T>,
{
    let mut buffer = Vec::new();
    for _ in 0..10 {
        buffer.push(thread_rng().gen_range(0..=10));
    }
    buffer
}
