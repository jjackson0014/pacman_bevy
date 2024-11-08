// Prelude + Other Crates
use crate::prelude::*;

// User input system
pub fn pacman_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Pacman>,
) {
    for mut pacman in query.iter_mut() {
        // Check for key presses and update Pac-Man's direction accordingly
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            //println!("UP PRESSED");
            pacman.direction = Vec2::new(0.0, 1.0); // Up
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            pacman.direction = Vec2::new(0.0, -1.0); // Down
           // println!("DOWN PRESSED");
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            pacman.direction = Vec2::new(-1.0, 0.0); // Left
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            pacman.direction = Vec2::new(1.0, 0.0); // Right
        } /*else {
            pacman.direction = Vec2::ZERO;
        }*/
    }
}

// Pac-Man Node Movement System

// Pac-Man Movement System
pub fn pacman_collision_based_movement_system(
    time: Res<Time>, 
    map: Res<Map>, // Access the map to check for walls
    mut query: Query<(&mut Transform, &mut Pacman)>
) {
    for (mut transform, mut pacman) in query.iter_mut() {


        // Calculate movement based on direction and speed
        let delta_movement = pacman.direction * pacman.speed * time.delta_seconds();
        let proposed_position = transform.translation + Vec3::new(delta_movement.x,delta_movement.y,0.0);
        // println!("proposed position = {}",proposed_position);

        // Convert the proposed position to grid coordinates
        // Trying to add room to keep Pac-Man from running THROUGH wall tiles
        let offset_x = if pacman.direction.x > 0.0 { pacman.radius } else if pacman.direction.x < 0.0 { -pacman.radius } else { 0.0 };
        let offset_y = if pacman.direction.y > 0.0 { pacman.radius } else if pacman.direction.y < 0.0 { -pacman.radius } else { 0.0 };
        
        let proposed_grid_x = ((proposed_position.x + offset_x + SCREEN_WIDTH / 2.0) / TILE_SIZE).floor() as i32;
        let proposed_grid_y = ((proposed_position.y + offset_y + SCREEN_HEIGHT / 2.0) / TILE_SIZE).floor() as i32;        

        
        // Check if the proposed tile is a wall
        if map.is_wall(proposed_grid_x,proposed_grid_y) {
            // Stop Pac-Man if the next tile is a wall
            println!("x:{},y{} is wall",proposed_grid_x,proposed_grid_y);
            pacman.direction = Vec2::ZERO;
            println!("Pac-Man blocked by a wall at ({}, {})", proposed_grid_x, proposed_grid_y);
        } else {
            // Move Pac-Man to the proposed position
            transform.translation = proposed_position;

            // Update grid_position as Pac-Man moves into a new grid cell
            if pacman.grid_position != (proposed_grid_x, proposed_grid_y) {
                // Update Pac-Man's grid position to the new cell
                pacman.grid_position = (proposed_grid_x, proposed_grid_y);
                println!("Pac-Man moved to a new grid position: {:?}", pacman.grid_position);
            }
        }
    }
}

