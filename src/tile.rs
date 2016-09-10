use graphics::types::Color;

use tile_type::TileType;

#[derive(Copy, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub hidden: bool,
    pub adjacent_mines: usize
}

impl Tile {
    pub fn color(&self) -> Color {
        const GREY: Color = [0.66, 0.66, 0.66, 1.0];
        if self.hidden { GREY } else { self.tile_type.color() }
    }

    pub fn nearby_mines(&self) -> String {
        format!("{}", self.adjacent_mines)
    }
}

