#[derive(Debug, Clone)]
pub struct PreviewConfig {
    show_board: bool,
    show_board_size: bool,
    show_tiles: bool,
    show_tile_count: bool,
}

impl PreviewConfig {
    pub fn new(
        show_board: bool,
        show_board_size: bool,
        show_tiles: bool,
        show_tile_count: bool,
    ) -> PreviewConfig {
        PreviewConfig {
            show_board,
            show_board_size,
            show_tiles,
            show_tile_count,
        }
    }

    pub fn show_board(&self) -> bool {
        self.show_board
    }

    pub fn show_board_size(&self) -> bool {
        self.show_board_size
    }

    pub fn show_tiles(&self) -> bool {
        self.show_tiles
    }

    pub fn show_tile_count(&self) -> bool {
        self.show_tile_count
    }
}

impl Default for PreviewConfig {
    fn default() -> Self {
        PreviewConfig {
            show_board: true,
            show_board_size: true,
            show_tiles: true,
            show_tile_count: true,
        }
    }
}
