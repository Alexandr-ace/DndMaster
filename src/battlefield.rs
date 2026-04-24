#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityId(u32);
impl EntityId {
    pub fn new(id: u32) -> Self {
        EntityId(id)
    }
}
pub struct Position {
    id: EntityId,
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize, id: EntityId) -> Self {
        Self { x, y, id }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum CellContent {
    Empty,
    Hero(EntityId),
    Enemy(EntityId),
    Obstacle,
}
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    content: CellContent,
    // можно добавить эффекты: огонь, ядовитое облако
}

pub struct Battlefield {
    grid: [[Cell; 10]; 10],
    hero_position: Position,
    enemy_positions: Position,
}
impl Battlefield {
    pub fn new_battlefield(id_hero: EntityId, id_enemy: EntityId) -> Self {
        Self {
            grid: [[Cell {
                content: CellContent::Empty,
            }; 10]; 10],
            hero_position: Position::new(7, 8, id_hero),
            enemy_positions: Position::new(9, 8, id_enemy),
        }
    }
}
