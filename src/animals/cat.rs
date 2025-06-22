use crate::pets::Gait;
use crate::pets::Noise;
use crate::pets::Specimen;

/// Cat implements both the Noise and the Gait traits.

#[derive(Default)]
pub struct Cat;

impl Specimen for Cat {
    fn specimen(&self) {
        println!("Cat");
    }
}

impl Noise for Cat {
    fn noise(&self) {
        println!("meow");
    }
}

impl Gait for Cat {
    fn gait(&self) {
        println!("scamper");
    }
}
