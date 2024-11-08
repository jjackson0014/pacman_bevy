// Prelude + Other Crates
use crate::prelude::*;

#[derive(Component)]
pub struct Pacman{
    pub radius: f32,
    pub grid_position: (i32,i32),
    pub speed: f32,
    pub direction: Vec2, // Vec with Direction
}

// Constants
const RADIUS: f32 = 8.0;

// Spawn a new Pac-Man
pub fn spawn_pacman (mut commands: Commands) {
    let color = YELLOW;
    let x = 14 as usize;
    let y = 16 as usize;
    let x_position = x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0 + TILE_SIZE / 2.0;
    let y_position = -(y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0 + TILE_SIZE / 2.0);
    println!("x = {}, y = {}, xval = {}, yval = {}", x, y, x_position, y_position);
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(TILE_SIZE,TILE_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x_position, y_position, 1.0),
            ..Default::default()
        },
        Pacman{
            // Radius
            radius: RADIUS,
            // Position
            grid_position: (x as i32,y as i32),
            // Speed
            speed: 100.0 * (TILE_SIZE/16.0),
            // Direction
            direction: Vec2::ZERO,
        }

    ));
}