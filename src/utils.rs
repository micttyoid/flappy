// utils.rs
use rand::Rng;

pub fn random_pipe_position() -> (f32, f32) {
    let mut rng = rand::rng();
    //let lower = -rng.gen_range(70.0..280.0); // Lower pipe position (negative)
    let lower = -rng.random_range(70.0..280.0);
    (lower, lower + 450.0)
}
