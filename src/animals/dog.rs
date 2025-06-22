use crate::pets::Gait;
use crate::pets::Noise;
use crate::pets::Specimen;

/// Dog implements both the Noise and the Gait traits.

#[derive(Default)]
pub struct Dog;

impl Specimen for Dog {
    fn specimen(&self) {
        println!("Dog");
    }
}

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
