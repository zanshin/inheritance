use crate::animals::cat::Cat;
use crate::animals::dog::Dog;
use crate::animals::duck::Duck;
pub(crate) use strum::EnumIter;

// Traits
pub trait Noise {
    fn noise(&self);
}

pub trait Gait {
    fn gait(&self);
}

pub trait Specimen {
    fn specimen(&self);
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

impl Specimen for Pet {
    fn specimen(&self) {
        match self {
            Pet::Cat(cat) => cat.specimen(),
            Pet::Dog(dog) => dog.specimen(),
            Pet::Duck(duck) => duck.specimen(),
        }
    }
}
