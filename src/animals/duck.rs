use crate::Gait;
use crate::Noise;

/// Duck implements both the Noise and the Gait traits.

#[derive(Default)]
pub struct Duck;

impl Noise for Duck {
    fn noise(&self) {
        println!("quack");
    }
}

impl Gait for Duck {
    fn gait(&self) {
        println!("waddle");
    }
}
