// Prelude + Other Crates
use crate::prelude::*;

// User input system
pub fn pacman_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pacman_query: Query<&mut Pacman>,
    node_query: Query<&MapNode>,
) {
    for mut pacman in pacman_query.iter_mut() {
        let new_direction = if keyboard_input.pressed(KeyCode::ArrowUp) {
            Some(PacManDirection::Up)
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Some(PacManDirection::Down)
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Some(PacManDirection::Left)
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Some(PacManDirection::Right)
        } else {
            None
        };
        

        if let Some(direction) = new_direction {
            // If Pac-Man is currently stopped, start moving in the new direction
            if pacman.node_direction == PacManDirection::Stop {
                pacman.set_direction_and_target(direction, &node_query, false);
                pacman.queued_direction = None; // Clear the queue since we're starting immediately
                println!("New Direction: {:?}", pacman.node_direction);
            // Reverse?
            } else if pacman.node_direction.opposite() == direction {
                pacman.set_direction_and_target(direction, &node_query, true);
                pacman.queued_direction = None;
                println!("Reverse Input Detected{:?}", pacman.node_direction);
            } else {
                // Otherwise, queue the new direction to apply at the next node
                pacman.queued_direction = Some(direction);
            }
        }
    }
}

// Pac-Man Node Movement System
pub fn pacman_node_based_movement_system(
    time: Res<Time>, 
    mut pacman_query: Query<(&mut Pacman, &mut Transform)>,
    node_query: Query<&MapNode>,
) {
    for (mut pacman, mut transform) in pacman_query.iter_mut() {
        // Calculate incremental movement based on direction and speed
        let movement_distance = pacman.speed * time.delta_seconds();
        let delta_movement = match pacman.node_direction {
            PacManDirection::Up => Vec2::new(0.0, movement_distance),
            PacManDirection::Down => Vec2::new(0.0, -movement_distance),
            PacManDirection::Left => Vec2::new(-movement_distance, 0.0),
            PacManDirection::Right => Vec2::new(movement_distance, 0.0),
            PacManDirection::Stop => Vec2::ZERO,
        };

        // Apply incremental movement based on `delta_movement`
        let new_position = Vec2::new(transform.translation.x, transform.translation.y) + delta_movement;
        transform.translation = Vec3::new(new_position.x, new_position.y, transform.translation.z);
        
        // Check if Pac-Man has overshot or reached the target node
        if pacman.overshot_target(&transform, &node_query) {
            if let Some(target_node) = pacman.target_node {
                // Align Pac-Man exactly to the node's position
                if let Ok(node) = node_query.get(target_node) {
                    transform.translation = Vec3::new(node.position.x, node.position.y, transform.translation.z);
                    pacman.current_node = target_node;
                    pacman.node_position = node.position;

                    // Apply queued direction if valid; otherwise, maintain the current direction or stop
                    if let Some(queued_direction) = pacman.queued_direction {
                        if pacman.valid_direction(queued_direction, &node_query) {
                            pacman.set_direction_and_target(queued_direction, &node_query, false);
                            pacman.queued_direction = None;
                        }
                    }

                    // Set the next target based on current direction, or stop if there is none
                    /*
                    let new_target = pacman.get_new_target(pacman.node_direction, &node_query);
                    if new_target != pacman.current_node {
                        // pacman.current_node = new_target;
                        pacman.target_node = new_target;
                    } else {
                        pacman.node_direction = PacManDirection::Stop;
                    }
                    
                    if !pacman.valid_direction(pacman.node_direction, &node_query) {
                        pacman.node_direction = PacManDirection::Stop;
                    }
                    */
                }
            }
        }/* else {
            println!("Pacman has not overshot target");
        }*/
    }
    
}