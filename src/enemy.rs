use crate::battlefield::EntityId;
use crate::weapon::Weapon;
use rand::Rng;

pub struct Enemy {
    id: EntityId,
    race: String,
    class: String,
    weapon: Weapon,
    armor: u32,
    level: u32,
    iniciative: u32,
    expirence: u32,
}
impl Enemy {
    pub fn new_enemy(id: EntityId) -> Self {
        let weapon = Weapon::new_weapon();

        const ENEMY_TYPES: [&str; 10] = [
            "Гоблин",
            "Орк",
            "Скелет",
            "Паук",
            "Разбойник",
            "Волк",
            "Зомби",
            "Гнолл",
            "Культист",
            "Тролль",
        ];

        Self {
            id: id,
            race: Self::random_race(ENEMY_TYPES).to_string(),
            class: String::from("Warrior"),
            weapon: weapon,
            armor: rand::thread_rng().gen_range(7..13) as u32,
            level: 1,
            iniciative: 0,
            expirence: 0,
        }
    }
    pub fn random_race(hero_names: [&str; 10]) -> &str {
        let randomize: usize = rand::thread_rng().gen_range(0..hero_names.len());
        hero_names[randomize]
    }

    pub fn race(&self) -> &str {
        &self.race
    }
}
