use graphics::types::Color;

use tile_type::TileType;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub hidden: bool,
    pub marked: bool,
    pub adjacent_mines: usize
}

impl Tile {
    pub fn color(&self) -> Color {
        const GREY: Color = [0.66, 0.66, 0.66, 1.0];
        const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
        if self.marked {
            YELLOW
        } else if self.hidden {
            GREY
        } else {
            self.tile_type.color()
        }
    }

    pub fn nearby_mines(&self) -> String {
        format!("{}", self.adjacent_mines)
    }

    pub fn click(&mut self) {
        if self.hidden {
            self.hidden = false;
            self.marked = false;
        }
    }

    pub fn mark(&mut self) {
        if self.hidden {
            self.marked = !self.marked;
        }
    }
}

