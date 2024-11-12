// Modules
mod map;
use map::Map;
mod node;
use node::NodeGroup;
use node::Maze;
mod pacman;
use pacman::Pacman;
mod gameplay;



// Prelude
mod prelude {
    // Crates
    pub use bevy::{prelude::*, sprite::MaterialMesh2dBundle}; // 0.14
    pub use std::collections::HashMap;
    pub use std::fs::File;
    pub use std::io::{self, BufRead};
    pub use std::path::Path;
    // Tile-Based Grid Constants:
    // Our tiles are going to be 16x16 pixels and the
    // screen will be 448x512, so 28 columns and 32 rows
    pub const TILE_SIZE: f32 = 16.0;
    pub const SCREEN_WIDTH: f32 = 464.0;
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
        // .insert_resource(Maze::new("/assets/mazes/maze_test.txt"))
        .add_systems(
            Startup, 
            (
                map::setup_map_system,
                node::load_maze,
                // node::setup_node_group,
                node::maze_to_nodes,
                node::assign_neighbors,
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