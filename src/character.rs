use rand::Rng;
// let secret_number = rand::thread_rng().gen_range(1..101);
// rand::thread_rng(): создает генератор случайных чисел.
// gen_range(1..101): генерирует число в заданном диапазоне. Левая граница (1) — включена, правая (101) — не включена
use crate::weapon::Weapon;

pub struct Character {
    name: String,
    race: String,
    class: String,
    weapon: Weapon,
    armor: u32,
    level: u32,
}
impl Character {
    pub fn new_hero() -> Self {
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
    pub fn random_name(hero_names: [&str; 10]) -> &str {
        let randomize: usize = rand::thread_rng().gen_range(0..hero_names.len());
        hero_names[randomize]
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
