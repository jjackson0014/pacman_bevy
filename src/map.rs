// Prelude + Other Crates
use crate::prelude::*;

// Constants
const MAP_WIDTH: usize = (SCREEN_WIDTH / TILE_SIZE) as usize; // Update to divide screen
const MAP_HEIGHT: usize = (SCREEN_HEIGHT / TILE_SIZE) as usize;

#[derive(Component)]
struct Tile;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Path,
    Pellet,
}

#[derive(Resource)]
pub struct Map {
    grid: Vec<Vec<TileType>>,
    width: usize,
    height: usize,
}

impl Map {
    // Make a new map resource
    pub fn new() -> Self {
        let grid = Self::generate_debug_grid();
        let width = grid[0].len();
        let height = grid.len();
        Self {grid, width, height}
    }

    // Create a map of the appropriate size
    pub fn generate_grid() -> Vec<Vec<TileType>> {
        let mut grid = vec![vec![TileType::Path; MAP_WIDTH]; MAP_HEIGHT];
    
        // Set up the Borders
        for x in 0..MAP_WIDTH {
            grid[0][x] = TileType::Wall; // Top Row
            grid[MAP_HEIGHT - 1][x] = TileType::Wall; // Bottom Row
        }
    
        for y in 0..MAP_HEIGHT {
            grid[y][0] = TileType::Wall; // Left Column
            grid[y][MAP_WIDTH - 1] = TileType::Wall; // Right Column
        }
    
        grid
    }

    pub fn generate_debug_grid() -> Vec<Vec<TileType>> {
        let mut grid = vec![vec![TileType::Path; MAP_WIDTH]; MAP_HEIGHT];
    
        // Set up the Borders
        for x in 0..MAP_WIDTH {
            grid[0][x] = TileType::Wall; // Top Row
            grid[MAP_HEIGHT - 1][x] = TileType::Wall; // Bottom Row
        }
    
        for y in 0..MAP_HEIGHT {
            grid[y][0] = TileType::Wall; // Left Column
            grid[y][MAP_WIDTH - 1] = TileType::Wall; // Right Column
        }
    
        grid
    }

    pub fn setup_debug_map(&self, mut commands: Commands, asset_server: Res<AssetServer>) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let x_position = x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0 + TILE_SIZE / 2.0;
                let y_position = -(y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0 + TILE_SIZE / 2.0);
                // println!("x = {}, y = {}, xval = {}, yval = {}", x, y, x_position, y_position);
    
                let color = match tile {
                    TileType::Wall => Color::srgb(0.0, 0.0, 1.0),  // Blue for walls
                    TileType::Path => Color::srgb(0.0, 0.0, 0.0),  // Black for paths
                    TileType::Pellet => Color::srgb(255.0, 255.0, 255.0), // WHITE for pellets
                };
    
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(TILE_SIZE - 1.0)), // Adding Room for Border
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(x_position, y_position, 0.0),
                    ..Default::default()
                });
                
                // Add a border for path tiles
                if let TileType::Path = tile {
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1.0, 1.0, 0.0), // Yellow border
                            custom_size: Some(Vec2::splat(TILE_SIZE)), // Full tile size for border
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(x_position, y_position, -0.1), // Slightly behind the tile
                        ..Default::default()
                    });
                }
                // Coordinates
                /*
                commands.spawn(Text2dBundle {
                    text: Text::from_section(
                        x.to_string() + &y.to_string(), 
                        TextStyle {
                            font_size: TILE_SIZE / 2.0,
                            color: Color::WHITE,
                            ..default()
                        }
                    ),
                    transform: Transform::from_xyz(x_position, y_position, 0.0),
                    ..Default::default()
                });
                */
            }
            let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf"); // Make sure the font path is correct
            for x in 0..MAP_WIDTH {
                let x_position = x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0; // + TILE_SIZE / 2.0;
                let label_text = Text::from_section(
                    x.to_string(),
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: TILE_SIZE / 2.0,
                        color: Color::rgb(0.5, 1.0, 0.5),
                    },
                );
        
                // Column labels
                commands.spawn(Text2dBundle {
                    text: label_text.clone(),
                    transform: Transform::from_xyz(x_position, SCREEN_HEIGHT / 2.0, 1.0),
                    ..Default::default()
                });
            }
        
            for y in 0..MAP_HEIGHT {
                let y_position = -(y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0 ); //+ TILE_SIZE / 2.0
                let label_text = Text::from_section(
                    y.to_string(),
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: TILE_SIZE / 2.0,
                        color: Color::rgb(0.5, 1.0, 0.5),
                    },
                );
        
                // Row labels
                commands.spawn(Text2dBundle {
                    text: label_text,
                    transform: Transform::from_xyz(-SCREEN_WIDTH / 2.0, y_position, 1.0),
                    ..Default::default()
                });
            }

        }
    }

    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.grid[y as usize][x as usize] == TileType::Wall
        } else {
            false // Out of bounds; treat it as a wall
        }
    }

}

// System
pub fn setup_map_system(commands: Commands, map: Res<Map>, asset_server: Res<AssetServer>) {
    map.setup_debug_map(commands, asset_server);
}

// Camera setup system
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}