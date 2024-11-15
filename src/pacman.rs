// Prelude + Other Crates
use crate::prelude::*;

#[derive(Component)]
pub struct Pacman{
    pub radius: f32,
    // pub grid_position: (i32,i32),
    pub node_position: Vec2,
    pub speed: f32,
    // pub vec_direction: Vec2, // Vec with Direction
    pub node_direction: PacManDirection,  // Up Right Down Left for Node Movement
    pub queued_direction: Option<PacManDirection>,
    pub current_node: Entity,
    pub target_node: Option<Entity>,
    pub is_reversing: bool,
}

// Constants
const RADIUS: f32 = 8.0;

// Implementation
impl Pacman {
    pub fn new(
        node_position: Vec2, 
        current_node: Entity, 
        queued_direction: Option<PacManDirection>,
    ) -> Self {
        Pacman {
            radius: RADIUS,
            node_position,
            speed: 100.0 * (TILE_SIZE / 16.0),
            node_direction: PacManDirection::Stop,
            // vec_direction: Vec2::ZERO,
            queued_direction,
            current_node,
            target_node: None,
            is_reversing: false,
        }
    }

    // Find a node to spawn pacman
    pub fn find_spawn_node(node_query: &Query<&MapNode>) -> Vec2{
        // Spawn on a particular Node
        if let Some(node) = node_query.iter().next() {
            node.position // Use the first node's position
        } else {
            Vec2::new(0.0, 0.0) // Default to (0,0) if no nodes are found
        }
    }


    // Spawn a new Pac-Man
    pub fn spawn_pacman (
        mut commands: Commands,
        node_query: Query<(Entity, &MapNode)>
    ) {
        if let Some((node_entity, node)) = node_query.iter().next() {
            let spawn_node_position = node.position;
            println!("Pacman spawned on node: {:?}", spawn_node_position);
            commands.spawn((
                Pacman::new(spawn_node_position, node_entity, None),
                SpriteBundle {
                    sprite: Sprite {
                        color: YELLOW,
                        custom_size: Some(Vec2::splat(TILE_SIZE * 0.8)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(spawn_node_position.x, spawn_node_position.y, 1.0)),
                    ..Default::default()
                },
            ));
        }
    }

     // Check if a given direction is valid (i.e., there's a neighbor in that direction)
     pub fn valid_direction(&self, direction: PacManDirection, node_query: &Query<&MapNode>) -> bool {
        if let Ok(node) = node_query.get(self.current_node) {
            return node.neighbors.contains_key(&direction) && direction != PacManDirection::Stop;
        }
        false
    }

    // Get the new target node in a specified direction, or return the current node if invalid
    pub fn get_new_target(&self, direction: PacManDirection, node_query: &Query<&MapNode>) -> Entity {
        if let Ok(node) = node_query.get(self.current_node) {
            if self.valid_direction(direction, node_query) {
                if let Some(Some(target_node)) = node.neighbors.get(&direction) {
                    return *target_node;
                }
            }
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
        // Update Pac-Man's direction
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
            // If no valid target, stop Pac-Man
            self.node_direction = PacManDirection::Stop;
            self.target_node = None;
        }
    }

    // Determine if Pac-Man is going to move past the target node
    pub fn overshot_target(&self, transform: &Transform, node_query: &Query<&MapNode>) -> bool {
        if let Ok(target_node) = node_query.get(self.get_new_target(self.node_direction, node_query)) {
            if let Ok(current_node) = node_query.get(self.current_node) {
                // Vector from current node to target node and from current node to Pac-Man
                let vec_to_target = target_node.position - current_node.position;
                let vec_to_pacman = Vec2::new(transform.translation.x, transform.translation.y) - current_node.position;
                
                // Compare distances squared to check if Pac-Man has reached or passed the target node
                vec_to_pacman.length_squared() >= vec_to_target.length_squared() // || vec_to_pacman.dot(vec_to_target) > 0.0
            } else {
                false
            }
        } else {
            false
        }
    }



}




