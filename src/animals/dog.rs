use crate::pets::Gait;
use crate::pets::Noise;

/// Dog implements both the Noise and the Gait traits.

#[derive(Default)]
pub struct Dog;

impl Noise for Dog {
    fn noise(&self) {
        println!("woof");
    }
}

impl Gait for Dog {
    fn gait(&self) {
        println!("run");
    }
}
