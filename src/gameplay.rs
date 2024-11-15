use std::thread::current;

// Prelude + Other Crates
use crate::prelude::*;

#[derive(Default, Resource)]
pub struct Score(pub u32);

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
            // Reverse
            } else if pacman.node_direction.opposite() == direction {
                pacman.set_direction_and_target(direction, &node_query, true);
                pacman.queued_direction = None;
                println!("Reverse Input Detected{:?}", pacman.node_direction);
            } else {
                // Otherwise, queue the new direction to apply at the next node
                pacman.queued_direction = Some(direction);
            }
        } else {
            let old_direction = pacman.node_direction;
            pacman.set_direction_and_target(old_direction, &node_query, false);
        }
    }
}

// Pac-Man Node Movement System
pub fn pacman_node_based_movement_system(
    time: Res<Time>, 
    mut pacman_query: Query<(&mut Pacman, &mut Transform)>,
    node_query: Query<&MapNode>,
    node_group: Res<NodeGroup>, // Adding NodeGroup here to access `node_list`
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

                    // Check if the node is a portal
                    if node.is_portal {
                        
                        println!("Arrived at Portal");
                        let grid_x = ((node.position.x + X_OFFSET) / TILE_SIZE) as usize;
                        let grid_y = ((node.position.y - Y_OFFSET) / TILE_SIZE) as usize;

                        println!("Current Node: {:?}", pacman.node_position);
                        
                        if let Some(opposite_position) = find_opposite_portal(&node) {

                            println!("Ran exit finding function");
                            
                            let opp_grid_x = ((opposite_position.x - X_OFFSET) / TILE_SIZE) as usize;
                            let opp_grid_y = ((opposite_position.y - Y_OFFSET) / -TILE_SIZE) as usize;
                            println!("Came up with Opposite Node x, y: {},{}",opp_grid_x,opp_grid_y);
                            println!("Opposite Node Found: {:?}", node_group.node_list.get(&(opp_grid_x as usize, opp_grid_y as usize)));
                            
                            if let Some(&opposite_entity) = node_group.node_list.get(&(opp_grid_x as usize, opp_grid_y as usize)) {
                                println!("Found opposite portal");
                                // Transport Pac-Man to the opposite portal node
                                if let Ok(opposite_node) = node_query.get(opposite_entity) {
                                    transform.translation = Vec3::new(opposite_node.position.x, opposite_node.position.y, transform.translation.z);
                                    pacman.current_node = opposite_entity; // Set to the opposite node's entity
                                    pacman.node_position = opposite_node.position;
                                }
                            }
                        }
                    }

                    // Apply queued direction if valid; otherwise, maintain the current direction or stop
                    if let Some(queued_direction) = pacman.queued_direction {
                        if pacman.valid_direction(queued_direction, &node_query) {
                            pacman.set_direction_and_target(queued_direction, &node_query, false);
                            pacman.queued_direction = None;
                        }
                    }
                }
            }
        }/* else {
            println!("Pacman has not overshot target");
        }*/
    }
}

pub fn pellet_collision_system(
    mut commands: Commands,
    pacman_query: Query<&Transform, With<Pacman>>,
    mut pellet_query: Query<(Entity, &Transform, &Pellet)>,
    mut score: ResMut<Score>,
) {
    if let Ok(pacman_transform) = pacman_query.get_single() {
        for (pellet_entity, pellet_transform, pellet) in pellet_query.iter_mut() {
            // Check if Pac-Man is at the same position as a pellet (simple collision detection)
            let distance = pacman_transform
                .translation
                .distance(pellet_transform.translation);
            if distance < pellet.collide_radius {
                // Collision detected, remove pellet
                commands.entity(pellet_entity).despawn();

                // Update score based on pellet type
                score.0 += pellet.base_points;
                println!("Pellet collected! New score: {}", score.0);
            }
        }
    }
}

// Ghost Node Movement System
pub fn ghost_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Ghost, &mut Transform)>,
    node_query: Query<&MapNode>,
    nodes: Res<NodeGroup>,
) {
    for (mut ghost, mut transform) in query.iter_mut() {
        
        // We know that we start with an initial direction and target
        if ghost.overshot_target(&transform, &node_query) {
            if let Some(target_node) = ghost.target_node {
                if let Ok(node) = node_query.get(target_node) {
                    transform.translation = Vec3::new(node.position.x, node.position.y, transform.translation.z);
                    ghost.current_node = target_node;
                    ghost.node_position = node.position;

                    // Check for portal disable [TODO]
                    if node.is_portal {
                                    
                        println!("Arrived at Portal");
                        let grid_x = ((node.position.x + X_OFFSET) / TILE_SIZE) as usize;
                        let grid_y = ((node.position.y - Y_OFFSET) / TILE_SIZE) as usize;

                        println!("Current Node: {:?}", ghost.node_position);
                        
                        if let Some(opposite_position) = find_opposite_portal(&node) {

                            println!("Ran exit finding function");
                            
                            let opp_grid_x = ((opposite_position.x - X_OFFSET) / TILE_SIZE) as usize;
                            let opp_grid_y = ((opposite_position.y - Y_OFFSET) / -TILE_SIZE) as usize;
                            println!("Came up with Opposite Node x, y: {},{}",opp_grid_x,opp_grid_y);
                            println!("Opposite Node Found: {:?}", nodes.node_list.get(&(opp_grid_x as usize, opp_grid_y as usize)));
                            
                            if let Some(&opposite_entity) = nodes.node_list.get(&(opp_grid_x as usize, opp_grid_y as usize)) {
                                println!("Found opposite portal");
                                // Transport Pac-Man to the opposite portal node
                                if let Ok(opposite_node) = node_query.get(opposite_entity) {
                                    transform.translation = Vec3::new(opposite_node.position.x, opposite_node.position.y, transform.translation.z);
                                    ghost.current_node = opposite_entity; // Set to the opposite node's entity
                                    ghost.node_position = opposite_node.position;
                                }
                            }
                        }
                    }
                    
                    // Arrived, find next direction
                    let potential_directions = ghost.valid_directions(&node_query);
                    if let Some(chosen_direction) = ghost.choose_random_direction(&potential_directions) {
                        // Move in the new direction
                        ghost.set_direction_and_target(chosen_direction, &node_query, false);
                    }
                }
            }
        } else {
            // Calculate incremental movement based on direction and speed
            let movement_distance = ghost.speed * time.delta_seconds();
            let delta_movement = match ghost.node_direction {
                PacManDirection::Up => Vec2::new(0.0, movement_distance),
                PacManDirection::Down => Vec2::new(0.0, -movement_distance),
                PacManDirection::Left => Vec2::new(-movement_distance, 0.0),
                PacManDirection::Right => Vec2::new(movement_distance, 0.0),
                PacManDirection::Stop => Vec2::ZERO,
            };

            // Apply incremental movement based on `delta_movement`
            let new_position = Vec2::new(transform.translation.x, transform.translation.y) + delta_movement;
            transform.translation = Vec3::new(new_position.x, new_position.y, transform.translation.z);
            
            }
    }
}