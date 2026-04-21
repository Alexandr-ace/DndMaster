use rand::Rng;
// let secret_number = rand::thread_rng().gen_range(1..101);
// rand::thread_rng(): создает генератор случайных чисел.
// gen_range(1..101): генерирует число в заданном диапазоне. Левая граница (1) — включена, правая (101) — не включена
use crate::dice::DiceType;
pub struct Weapon {
    name: String,
    dise_damage: DiceType,
    difficult: u32,
    modificate: u32,
}

impl Weapon {
    pub fn new_weapon() -> Self {
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
