use inheritance::pets::Gait;
use inheritance::pets::Noise;
use inheritance::pets::Pet;
use inheritance::pets::Specimen;
use strum::IntoEnumIterator;

fn main() {
    for critter in Pet::iter() {
        critter.specimen();
        critter.noise();
        critter.gait();
        println!("{}", " ");
    }
}
