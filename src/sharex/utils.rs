use rand::{distributions::Alphanumeric, distributions::DistString};

pub fn random_string(size: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), size)
}
