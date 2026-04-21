#![allow(dead_code, unused_variables)]
mod character; // говорит: "ищи character.rs"
mod dice; // ищи dice.rs
mod enemy; // потом будет
mod inventory;
mod weapon; // ищи weapon.rs // потом будет

use character::Character;
// use dice::DiceType;

fn main() {
    let hero: Character = Character::new_hero();
    println!("Имя персонажа: {}", hero.name());
}
