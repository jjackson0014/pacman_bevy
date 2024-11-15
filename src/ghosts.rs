// Prelude + Other Crates
use crate::prelude::*;

#[derive(Component)]
pub struct Ghost{
    pub radius: f32,
    pub node_position: Vec2,
    pub speed: f32,
    pub node_direction: PacManDirection,  // Up Right Down Left for Node Movement
    pub queued_direction: Option<PacManDirection>,
    pub current_node: Entity,
    pub target_node: Option<Entity>,
    pub is_reversing: bool,
    pub is_visible: bool,
    pub goal: Vec2,
}

// Constants
const RADIUS: f32 = 5.0;

// Implementation
impl Ghost {
    pub fn new(
        node_position: Vec2,
        current_node: Entity, 
        queued_direction: Option<PacManDirection>,
    ) -> Self {
        Ghost {
            radius: RADIUS,
            node_position,
            speed: 100.0 * (TILE_SIZE / 16.0), //100
            node_direction: PacManDirection::Stop,
            queued_direction,
            current_node,
            target_node: None,
            is_reversing: false,
            is_visible: true,
            goal: Vec2::new(0.0, 0.0) ,
        }
    }

    // Find a node to spawn Ghost
    /*
    pub fn find_spawn_node(node_query: &Query<&MapNode>) -> Vec2{
        // Spawn on a particular Node
        if let Some(node) = node_query.iter().next() {
            node.position // Use the first node's position
        } else {
            Vec2::new(0.0, 0.0) // Default to (0,0) if no nodes are found
        }
    }
    */

    // Spawn a new Pac-Man
    pub fn spawn_ghost (
        mut commands: Commands,
        node_query: Query<(Entity, &MapNode)>
    ) {
        if let Some((node_entity, node)) = node_query.iter().next() {

            let spawn_node_position = node.position;
            println!("Ghost spawned on node: {:?}", spawn_node_position);
            commands.spawn((
                Ghost::new(spawn_node_position, node_entity, None),
                SpriteBundle {
                    sprite: Sprite {
                        color: BLUE,
                        custom_size: Some(Vec2::splat(TILE_SIZE * 0.8)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(spawn_node_position.x, spawn_node_position.y, 1.0)),
                    ..Default::default()
                },
            ));
        }
    }

    pub fn initialize_ghost_movement (
        mut commands: Commands,
        mut query: Query<(&mut Ghost, &mut Transform)>,
        node_query: Query<&MapNode>
    ) {
        for (mut ghost, mut transform) in query.iter_mut() {
            let valid_directions = ghost.valid_directions(&node_query);
            println!("Valid Spawn Directions: {:?}", valid_directions);
            if let Some(new_direction) = ghost.choose_random_direction(&valid_directions) {
                // Update current node to indicate ghost arrived
                // Update the last direction and target node for the ghost
                ghost.set_direction_and_target(new_direction, &node_query, false);
                println!("spawn direction: {:?} Target: {:?}",ghost.node_direction,ghost.target_node);
            }
        }
    }

     // Check if a given direction is valid (i.e., there's a neighbor in that direction)
     pub fn valid_directions(&self, node_query: &Query<&MapNode>) -> Vec<PacManDirection> {
        let mut valid_directions = Vec::new();
        let directions = [
            PacManDirection::Up,
            PacManDirection::Down,
            PacManDirection::Left,
            PacManDirection::Right,
        ];

        let last_direction = self.node_direction;
        // println!("Choosing Valid Direction");

        /*
        for &direction in directions.iter() {
            if let Ok(node) = node_query.get(self.current_node) {
                if node.neighbors.contains_key(&direction) &&
                direction.opposite() != last_direction {
                    valid_directions.push(direction);
                }
            }
        } */

        if let Ok(node) = node_query.get(self.current_node) {
            for &direction in directions.iter() {
                // Check if there is a valid neighbor (i.e., `Some(entity)`) in this direction
                if let Some(Some(_neighbor_entity)) = node.neighbors.get(&direction) {
                    // Ensure that the direction is not the one we just came from
                    if direction != last_direction.opposite() {
                        valid_directions.push(direction);
                    }
                }
            }
        }

        // println!("Valid Directions Available: {:?}",valid_directions);

        if valid_directions.is_empty() {
            //if let Some(last_dir) = last_direction {
                valid_directions.push(last_direction);
                // println!("No valid directions, using: {:?} ", last_direction);
            //}
        }

        valid_directions
    }

    pub fn choose_random_direction(&self, valid_directions: &[PacManDirection]) -> Option<PacManDirection> {
        // Check if the list of valid directions is empty
        if valid_directions.is_empty() {
            return None;
        }
    
        // Use `thread_rng` to get a random number generator and choose a random element
        let mut rng = thread_rng();
        valid_directions.choose(&mut rng).copied()
    }

    // Get the new target node in a specified direction, or return the current node if invalid
    pub fn get_new_target(&self, direction: PacManDirection, node_query: &Query<&MapNode>) -> Entity {
        if let Ok(node) = node_query.get(self.current_node) {
            //if self.valid_direction(direction, node_query) {
                if let Some(Some(target_node)) = node.neighbors.get(&direction) {
                    return *target_node;
                }
            //}
        }
        self.current_node // Return current node if no valid target
    }

    /// Sets Pac-Man's direction and updates the target node to the next node in that direction.
    pub fn set_direction_and_target(
        &mut self, 
        new_direction: PacManDirection, 
        node_query: &Query<&MapNode>,
        is_reverse: bool,
    ) {
        // Update Ghost's direction
        self.node_direction = new_direction;

        let origin_node = self.current_node;

        // Reversing
        if is_reverse {
            if let Some(target) = self.target_node{
                self.current_node = target;
            }
            self.target_node = Some(origin_node);
        }
        
        let next_node = self.get_new_target(new_direction, node_query);
        
        // If a valid target node exists, set it as the target node
        if next_node != self.current_node {
            self.target_node = Some(next_node);
        } else {
            // If no valid target, stop Ghost
            self.node_direction = PacManDirection::Stop;
            self.target_node = None;
        }
    }

    // Determine if Ghost is going to move past the target node
    pub fn overshot_target(&self, transform: &Transform, node_query: &Query<&MapNode>) -> bool {
        if let Ok(target_node) = node_query.get(self.get_new_target(self.node_direction, node_query)) {
            if let Ok(current_node) = node_query.get(self.current_node) {
                // Vector from current node to target node and from current node to Ghost
                let vec_to_target = target_node.position - current_node.position;
                let vec_to_ghost = Vec2::new(transform.translation.x, transform.translation.y) - current_node.position;
                
                // Compare distances squared to check if Ghost has reached or passed the target node
                vec_to_ghost.length_squared() >= vec_to_target.length_squared()
            } else {
                false
            }
        } else {
            false
        }
    }

}




