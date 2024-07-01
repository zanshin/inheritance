use crate::Gait;
use crate::Noise;

/// Cat implements both the Noise and the Gait traits.

#[derive(Default)]
pub struct Cat;

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
