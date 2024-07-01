use strum::*;

mod animals;
use animals::cat::Cat;
use animals::dog::Dog;
use animals::duck::Duck;

// Traits
trait Noise {
    fn noise(&self);
}

trait Gait {
    fn gait(&self);
}

// Enum of Pets
#[derive(EnumIter)]
enum Pet {
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

fn main() {
    for critter in Pet::iter() {
        critter.noise();
        critter.gait();
        println!("{}", " ");
    }
}
