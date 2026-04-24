pub struct EntityId(u32);

impl EntityId {
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
            class: String::from("Rogue"),
            weapon: weapon,
            armor: rand::thread_rng().gen_range(7..13) as u32,
            level: 1,
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
