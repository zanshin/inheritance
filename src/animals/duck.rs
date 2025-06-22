use crate::pets::Gait;
use crate::pets::Noise;
use crate::pets::Specimen;

/// Duck implements both the Noise and the Gait traits.

#[derive(Default)]
pub struct Duck;

impl Specimen for Duck {
    fn specimen(&self) {
        println!("Duck");
    }
}

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
