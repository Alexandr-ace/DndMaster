use crate::character::Run;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellContent {
    Empty,
    Hero(EntityId),
    Enemy(EntityId),
    Obstacle,
}
impl CellContent {
    pub fn render(&self) -> char {
        match self {
            CellContent::Empty => '.',
            CellContent::Hero(_) => 'Г',
            CellContent::Enemy(_) => 'В',
            CellContent::Obstacle => '#',
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub content: CellContent,
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
            hero_position: Position::new(2, 0, id_hero),
            enemy_positions: Position::new(1, 0, id_enemy),
        }
    }

    pub fn render(&mut self) {
        self.setup_place();
        for row in 0..10 {
            for col in 0..10 {
                print!("{} ", self.grid[row][col].content.render());
            }
            println!(); // перенос строки после каждой строки поля
        }
    }
    pub fn setup_place(&mut self) {
        self.grid[self.hero_position.x][self.hero_position.y].content =
            CellContent::Hero(self.hero_position.id);
        self.grid[self.enemy_positions.x][self.enemy_positions.y].content =
            CellContent::Enemy(self.enemy_positions.id);
    }
    pub fn setup_place_empty(&mut self) {
        self.grid[self.hero_position.x][self.hero_position.y].content = CellContent::Empty;
        self.grid[self.enemy_positions.x][self.enemy_positions.y].content = CellContent::Empty;
    }
    pub fn hero_run(&mut self, point: &Run) {
        match point {
            Run::Hold => return, // Ничего не делаем
            Run::Up => {
                if self.hero_position.x == 0 {
                    return;
                } else {
                    match self.grid[self.hero_position.x - 1][self.hero_position.y].content {
                        CellContent::Empty => self.hero_position.x = self.hero_position.x - 1,
                        CellContent::Hero(_) => println!("Впереди дружественный герой"),
                        CellContent::Enemy(_) => println!("Впереди враг"),
                        CellContent::Obstacle => println!("Впереди препятствие"),
                    }
                }
            }
            Run::Down => {
                if self.hero_position.x >= 9 {
                    return;
                };
                self.hero_position.x = self.hero_position.x + 1;
            }
            Run::Left => {
                if self.hero_position.y == 0 {
                    return;
                };
                self.hero_position.y = self.hero_position.y - 1;
            }
            Run::Right => {
                if self.hero_position.y >= 9 {
                    return;
                };
                self.hero_position.y = self.hero_position.y + 1;
            }
        };
        self.setup_place_empty();
    }
}
