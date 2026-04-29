#![allow(dead_code, unused_variables)]
mod battlefield;
mod character;
mod dice;
mod enemy;
mod inventory;
mod weapon;
use crate::enemy::Enemy;
use battlefield::Battlefield;
use battlefield::EntityId;
use character::Character;
// impl From<u32> for EntityId {
//     fn from(n: u32) -> Self {
//         EntityId(n)
//     }
// }
// git add .
// git commit -m "Описание изменений"
// git push origin master

fn main() {
    let num = 25;
    if num == 25 {
        println!("Handsome!")
    } else {
        println!("Handsome, too!")
    }
    let mut field: Environment = Environment::new_game();
    field.battlifield.render();
    println!("Герой двигается");
    field.run_hero();
}

struct Environment {
    id_count: EntityId,
    hero: Character,
    enemy: Enemy,
    battlifield: Battlefield,
}
impl Environment {
    pub fn new_game() -> Self {
        let mut id_count: u32 = 1;
        let hero: EntityId = EntityId::new(id_count);
        // let mut id_listed: Vec<EntityId> = Vec::new();
        // id_listed.push(hero.clone());
        id_count += 1;
        let enemy: EntityId = EntityId::new(id_count);
        // id_listed.push(enemy.clone());
        id_count += 1;
        let id_cont_exp: EntityId = EntityId::new(id_count);

        Self {
            id_count: id_cont_exp,
            hero: Character::new_hero(hero),
            enemy: Enemy::new_enemy(enemy),
            battlifield: Battlefield::new_battlefield(hero, enemy),
        }
    }
    pub fn run_hero(&mut self) {
        self.battlifield.hero_run(&self.hero.move_point);
        self.battlifield.render();
    }
}
