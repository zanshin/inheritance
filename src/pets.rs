use strum::EnumIter;
use crate::animals::cat::Cat;
use crate::animals::dog::Dog;
use crate::animals::duck::Duck;

// Traits
pub trait Noise {
    fn noise(&self);
}

pub trait Gait {
    fn gait(&self);
}

// Enum of Pets
#[derive(EnumIter)]
pub enum Pet {
    Cat(Cat),
    Dog(Dog),
    Duck(Duck),
}

// Pet trait impls
impl Noise for Pet {
    fn noise(&self) {
        match self {
            Pet::Cat(cat) => cat.noise(),
            Pet::Dog(dog) => dog.noise(),
            Pet::Duck(duck) => duck.noise(),
        }
    }
}

impl Gait for Pet {
    fn gait(&self) {
        match self {
            Pet::Cat(cat) => cat.gait(),
            Pet::Dog(dog) => dog.gait(),
            Pet::Duck(duck) => duck.gait(),
        }
    }
}
