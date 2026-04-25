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
impl CellContent {
    pub fn render_str(&self) -> &'static str {
        match self {
            CellContent::Empty => ".",
            CellContent::Hero(_) => "Г",
            CellContent::Enemy(_) => "В",
            CellContent::Obstacle => "#",
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub content: CellContent,
    pub render: &'static str,
    // можно добавить эффекты: огонь, ядовитое облако
}

pub struct Battlefield {
    grid: [[Cell; 10]; 10],
    hero_position: Position,
    enemy_positions: Position,
}
impl Battlefield {
    pub fn new_battlefield(id_hero: EntityId, id_enemy: EntityId) -> Self {
        let mut field = Self {
            grid: [[Cell {
                content: CellContent::Empty,
                render: ".",
            }; 10]; 10],
            hero_position: Position::new(7, 8, id_hero),
            enemy_positions: Position::new(9, 8, id_enemy),
        };
        // Ставим метки
        field.grid[8][7].content = CellContent::Hero(id_hero);
        field.grid[8][9].content = CellContent::Enemy(id_enemy);
        field
    }
    pub fn prerender(&mut self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                self.grid[row][col].render = self.grid[row][col].content.render_str();
            }
        }
    }
    pub fn render(&mut self) {
        self.prerender();
        for row in 0..10 {
            for col in 0..10 {
                print!("{} ", self.grid[row][col].render);
            }
            println!(); // перенос строки после каждой строки поля
        }
    }
}
