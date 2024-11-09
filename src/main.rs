// Modules
mod map;
use map::Map;
mod node;
use node::NodeGroup;
mod pacman;
use pacman::Pacman;
mod gameplay;



// Prelude
mod prelude {
    // Crates
    pub use bevy::{prelude::*, sprite::MaterialMesh2dBundle}; // 0.14
    pub use std::collections::HashMap;
    // Tile-Based Grid Constants:
    // Our tiles are going to be 16x16 pixels and the
    // screen will be 448x512, so 28 columns and 32 rows
    pub const TILE_SIZE: f32 = 16.0;
    pub const SCREEN_WIDTH: f32 = 448.0;
    pub const SCREEN_HEIGHT: f32 = 512.0;

    // Colors
    pub const YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);
    pub const WHITE: Color = Color::srgb(255.0, 255.0, 255.0);
    pub const RED: Color = Color::srgb(255.0, 0.0, 0.0);

    pub use crate::map::*;
    pub use crate::pacman::*;
    pub use crate::node::*;
    pub use crate::gameplay::*;
}

use prelude::*;

// Main
pub fn main() {
    // Create the Bevy App/Game
    App::new()
        .add_plugins(
            DefaultPlugins
        )
        .insert_resource(Map::new())
        .insert_resource(NodeGroup::new())
        .add_systems(
            Startup, 
            (
                map::setup_map_system,
                node::setup_node_group,
                node::render_nodes_as_quads,
                spawn_camera,
                pacman::Pacman::spawn_pacman
            ).chain()
        )
        .add_systems(Update, (
            gameplay::pacman_input_system,
            //gameplay::pacman_collision_based_movement_system
            gameplay::pacman_node_based_movement_system,
        )
        //.chain()
        )
        .run();
}