#![allow(dead_code, unused_variables)]
use rand::Rng;
// let secret_number = rand::thread_rng().gen_range(1..101);
// rand::thread_rng(): создает генератор случайных чисел.
// gen_range(1..101): генерирует число в заданном диапазоне. Левая граница (1) — включена, правая (101) — не включена
fn main() {
    let hero: Character = Character::new_hero();
    let name: &String = &hero.name;
    println!("Имя персонажа: {name}");
}
struct Character {
    name: String,
    race: String,
    class: String,
    weapon: Weapon,
    armor: u32,
    level: u32,
}
impl Character {
    fn new_hero() -> Self {
        let weapon = Weapon::new_weapon();

        const ELF_FEMALE_NAMES: [&str; 10] = [
            "Aelindra",
            "Laurelin",
            "Nimue",
            "Galadriel",
            "Ithilwen",
            "Luthien",
            "Finduilas",
            "Miriel",
            "Silmeria",
            "Caladhiel",
        ];

        Self {
            name: Self::random_name(ELF_FEMALE_NAMES).to_string(),
            race: String::from("Elf"),
            class: String::from("Archer"),
            weapon: weapon,
            armor: rand::thread_rng().gen_range(7..13) as u32,
            level: 1,
        }
    }
    fn random_name(hero_names: [&str; 10]) -> &str {
        let randomize: usize = rand::thread_rng().gen_range(0..hero_names.len());
        hero_names[randomize]
    }
}

struct Weapon {
    name: String,
    dise_damage: DiceType,
    difficult: u32,
    modificate: u32,
}
impl Weapon {
    fn new_weapon() -> Self {
        const WEAPON_NAMES: [&str; 10] = [
            "Стальной клинок",
            "Кованый топор",
            "Длинный меч",
            "Боевой молот",
            "Охотничий нож",
            "Шипастая булава",
            "Гномий кинжал",
            "Двуручный меч",
            "Короткое копьё",
            "Тяжёлый цеп",
        ];

        Self {
            name: Self::random_name_weapon(WEAPON_NAMES).to_string(),
            dise_damage: DiceType::D8,
            difficult: rand::thread_rng().gen_range(5..10) as u32,
            modificate: rand::thread_rng().gen_range(5..7) as u32,
        }
    }
    fn random_name_weapon(weapon_names: [&str; 10]) -> &str {
        let randomize: usize = rand::thread_rng().gen_range(0..weapon_names.len());
        weapon_names[randomize]
    }
}

enum DiceType {
    D4,
    D6,
    D8,
    D20,
}
