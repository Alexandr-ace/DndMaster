use rand::Rng;
// let secret_number = rand::thread_rng().gen_range(1..101);
// rand::thread_rng(): создает генератор случайных чисел.
// gen_range(1..101): генерирует число в заданном диапазоне. Левая граница (1) — включена, правая (101) — не включена
use crate::battlefield::EntityId;
use crate::weapon::Weapon;

// Вывод персонажа в консоль — чтобы видеть результат генерации.

// Создание врага — с похожей случайной генерацией.

// Простой цикл боя — атака, расчёт урона через кубик, победа/поражение.

// Инвентарь и зелья — использовать Vec<Item>.
pub enum Run {
    Hold,
    Up,
    Down,
    Right,
    Left,
}

pub struct Character {
    pub id: EntityId,
    name: String,
    race: String,
    class: String,
    weapon: Weapon,
    armor: u32,
    level: u32,
    pub move_point: Run,
    iniciative: u32,
    expirence: u32,
}
impl Character {
    pub fn new_hero(id: EntityId) -> Self {
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
            id: id,
            name: Self::random_name(ELF_FEMALE_NAMES).to_string(),
            race: String::from("Elf"),
            class: String::from("Rogue"),
            weapon: weapon,
            armor: rand::thread_rng().gen_range(7..13) as u32,
            level: 1,
            move_point: Run::Up,
            iniciative: 0,
            expirence: 0,
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
