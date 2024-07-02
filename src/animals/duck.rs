use crate::pets::Gait;
use crate::pets::Noise;

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
