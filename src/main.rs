use strum::IntoEnumIterator;
use inheritance::pets::Pet;
use inheritance::pets::Noise;
use inheritance::pets::Gait;

fn main() {
    for critter in Pet::iter() {
        critter.noise();
        critter.gait();
        println!("{}", " ");
    }
}
