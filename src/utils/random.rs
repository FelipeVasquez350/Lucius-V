use rand::Rng;

pub fn get_random_number(lower_bound: u32, upper_bound: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower_bound..=upper_bound)
}