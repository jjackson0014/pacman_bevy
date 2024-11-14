use crate::prelude::*;

// Define direction constants
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum PacManDirection {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

impl PacManDirection {
    pub fn opposite(&self) -> PacManDirection {
        match *self {
            PacManDirection::Up => PacManDirection::Down,
            PacManDirection::Down => PacManDirection::Up,
            PacManDirection::Left => PacManDirection::Right,
            PacManDirection::Right => PacManDirection::Left,
            PacManDirection::Stop => PacManDirection::Stop,
        }
    }
}

// Pellets
#[derive(Bundle)]
pub struct PelletBundle {
    pellet: Pellet,
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Pellet {
    pub is_power: bool,
    pub color: Color,
    pub pellet_radius: f32,
    pub collide_radius: f32,
    pub base_points: i32,
    pub is_eaten: bool,
    pub flashing_off: bool,
    pub flash_time: f32,
    pub self_timer: f32,
}

impl Pellet {
    pub fn new(is_power: bool) -> Self {

        let size = if is_power {
            8.0 * (TILE_SIZE / 16.0)
        } else {
            4.0 * (TILE_SIZE / 16.0)
        };

        let base_points = if is_power {50} else {10};

        let flash_time = if is_power {0.2} else {0.0};

        Pellet { 
            is_power,
            color: WHITE,
            pellet_radius: size,
            collide_radius: size,
            base_points: base_points,
            is_eaten: false,
            flashing_off: false,
            flash_time: flash_time,
            self_timer: 0.0,
        }
        
    }

    // pub fn flash_pellets(&self, tick) {}

}

#[derive(Component)]
pub struct PowerPellet;

// Create a maze resource to be used in node building
// Define Cell Types
#[derive(Debug, Clone, Copy, PartialEq)]
enum MazeCell {
    Empty,
    Node,
    Path {has_pellet: bool},
    PowerPellet
}

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct Maze {
    grid: HashMap<(usize, usize), MazeCell>,
}

impl Maze {
    
    pub fn new() -> io::Result<Self> {
        Self::read_map("assets/mazes/maze1.txt")
    }

    pub fn read_map(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);

        let mut grid = HashMap::new();

        for (y, line) in reader.lines().enumerate() {
            let line = line?.trim().to_string();  
            for (x, c) in line.split_whitespace().enumerate() {
                let cell = match c {
                    "X" => MazeCell::Empty,
                    "+" => MazeCell::Node,
                    "n" => MazeCell::Node,
                    "|" => MazeCell::Path { has_pellet: false },
                    "-" => MazeCell::Path { has_pellet: false },
                    "." => MazeCell::Path { has_pellet: true },
                    "P" => MazeCell::PowerPellet,
                    _ => MazeCell::Empty, 
                };
                grid.insert((x, y), cell);
            }
        }
        
        Ok(Maze { grid })
    }

    //
}

// Create individual Node Component
#[derive(Component)]
pub struct MapNode {
    pub position: Vec2,
    pub neighbors: HashMap<PacManDirection, Option<Entity>>, // Neighbor nodes
    pub is_portal: bool,
}

impl MapNode {
    pub fn new(x: f32, y: f32) -> Self {
        MapNode {
            position: Vec2::new(x, y),
            neighbors: HashMap::from([
                (PacManDirection::Up, None),
                (PacManDirection::Down, None),
                (PacManDirection::Left, None),
                (PacManDirection::Right, None),
            ]),
            is_portal: false,
        }
    }
}

// Group Nodes together
#[derive(Resource)]
pub struct NodeGroup {
    pub node_list: HashMap<(usize, usize), Entity>,
}

impl NodeGroup {
    pub fn new() -> Self {
        NodeGroup {
            node_list: HashMap::new(),
        }
    }

    pub fn setup_nodes(
        &mut self, 
        mut commands: Commands, 
        maze: Res<Maze>
    ) {

        // let mut nodes = HashMap::new(); // Store nodes by (x, y) positions

        // Create nodes for each `MazeCell::Node` or `MazeCell::PowerPellet`
        for (&(x, y), &cell) in maze.grid.iter() {
            if matches!(cell, MazeCell::Node | MazeCell::PowerPellet) {
                let x_position = x as f32 * TILE_SIZE + X_OFFSET;
                let y_position = -(y as f32 * TILE_SIZE) + Y_OFFSET;
                
                let node_entity = commands.spawn(MapNode::new(x_position, y_position)).id();
                self.node_list.insert((x, y), node_entity);
            }
        }
        
    }
}

// Systems to Render Nodes on Screen
pub fn load_maze(mut commands: Commands) {
    match Maze::new() {
        Ok(maze) => commands.insert_resource(maze),
        Err(e) => eprintln!("Failed to load maze: {}", e),
    }
}

//
pub fn maze_to_nodes(
    mut commands: Commands,
    mut node_group: ResMut<NodeGroup>,
    maze: Res<Maze>,
) {
    node_group.setup_nodes(commands, maze);
}

// Define a helper function to find the next node in a direction
pub fn find_next_node(
    start_x: usize,
    start_y: usize,
    dx: isize,
    dy: isize,
    maze: &Maze,
    nodes: &HashMap<(usize, usize), Entity>
) -> Option<Entity> {
    let (mut x, mut y) = (start_x as isize, start_y as isize);

    loop {
        x += dx;
        y += dy;

        // Convert (x, y) back to usize for grid lookup
        if x < 0 || y < 0 {
            return None;
        }
        let (ux, uy) = (x as usize, y as usize);

        // Check if the cell exists in the grid
        match maze.grid.get(&(ux, uy)) {
            Some(MazeCell::Node) | Some(MazeCell::Path { has_pellet: _ }) | 
            Some(MazeCell::PowerPellet)=> {
                // Return the next node if it's a valid path or node
                if let Some(entity) = nodes.get(&(ux, uy)) {
                    return Some(*entity);
                }
            }
            Some(MazeCell::Empty) | None => {
                // Stop if we encounter an empty cell or go out of bounds
                return None;
            }
            _ => {} // Continue if the cell is not a node or path
        }

        // Boundary check in case we move out of bounds
        let max_x = maze.grid.keys().map(|(x, _)| *x).max().unwrap_or(0);
        let max_y = maze.grid.keys().map(|(_, y)| *y).max().unwrap_or(0);

        if ux >= max_x || uy >= max_y {
            return None;
        }
    }
}

// Define neighbors by checking adjacent cells
pub fn assign_neighbors(
    maze: Res<Maze>,
    map_nodes: Res<NodeGroup>,
    mut query: Query<&mut MapNode>,
) {
    for (&(x, y), &node_entity) in map_nodes.node_list.iter() {
        if let Ok(mut node) = query.get_mut(node_entity) {
            // Check each direction and assign neighbors if found
            if let Some(up_neighbor) = find_next_node(x, y, 0, -1, &maze, &map_nodes.node_list) {
                node.neighbors.insert(PacManDirection::Up, Some(up_neighbor));
            }
            if let Some(down_neighbor) = find_next_node(x, y, 0, 1, &maze, &map_nodes.node_list) {
                node.neighbors.insert(PacManDirection::Down, Some(down_neighbor));
            }
            if let Some(left_neighbor) = find_next_node(x, y, -1, 0, &maze, &map_nodes.node_list) {
                node.neighbors.insert(PacManDirection::Left, Some(left_neighbor));
            }
            if let Some(right_neighbor) = find_next_node(x, y, 1, 0, &maze, &map_nodes.node_list) {
                node.neighbors.insert(PacManDirection::Right, Some(right_neighbor));
            }
        }
    }
}

// Portals
pub fn identify_portal_nodes (
    map_nodes: Res<NodeGroup>,
    mut query: Query<&mut MapNode>,
) {
    // Maps will always be desined so that valid portals are nodes that are stationed along the wall
    // This is controlled/checked by the map i.e. no portals on corners, etc

    for (&(_x, _y), &node_entity) in map_nodes.node_list.iter() {
        if let Ok(mut node) = query.get_mut(node_entity) {
            if node.position.x == LEFT_BOUND {
                node.is_portal = true;
                println!("Identified Left-Side Portal");
            }

            if node.position.x == RIGHT_BOUND {
                node.is_portal = true;
                println!("Identified Right-Side Portal");
            }

            if node.position.y == TOP_BOUND {
                node.is_portal = true;
                println!("Identified Top-Side Portal");
            }

            if node.position.y == BOTTOM_BOUND {
                node.is_portal = true;
                println!("Identified Bottom-Side Portal");
            }

        }
    }
}

// Lifetime - suggested by rustanalyzer - honestly not 100%
// sure on this but <'a> is like a lifetime "tag" to say every input has the same
// Lifetime
pub fn find_opposite_portal(
    current_node: &MapNode,
) -> Option<Vec2> {
    let x = current_node.position.x;
    let y = current_node.position.y;

    println!("Current Node X,Y {},{}",x,y);

    let opposite_position = if x == LEFT_BOUND {
        Vec2::new(RIGHT_BOUND, y)
    } else if x == RIGHT_BOUND {
        Vec2::new(LEFT_BOUND, y)
    } else if y == TOP_BOUND {
        Vec2::new(x, BOTTOM_BOUND)
    } else if y == BOTTOM_BOUND {
        Vec2::new(x, TOP_BOUND)
    } else {
        return None; // Not a portal node
    };

    Some(opposite_position)
}

// pub fn portal transport

pub fn render_nodes_as_quads(
    mut commands: Commands,
    query: Query<(Entity, &MapNode)>,
) {
    for (_, node) in query.iter() {
        // Draw each node as a circle
        // Debugging output for node positions
        println!("Rendering Node at Position X: {} Y: {}", node.position.x, node.position.y);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: WHITE,
                custom_size: Some(Vec2::splat(16.0)), // Adjust size as needed
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(node.position.x, node.position.y, 0.5)),
            ..default()
        });

        // Draw lines to each neighbor using a quad
        /*
        for neighbor in node.neighbors.values().flatten() {
            if let Ok((_, neighbor_node)) = query.get(*neighbor) {
                let start = Vec2::new(node.position.x, node.position.y);
                let end = Vec2::new(neighbor_node.position.x, neighbor_node.position.y);

                // Calculate the midpoint and rotation for the line
                let midpoint = (start + end) / 2.0;
                let line_direction = end - start;
                let length = line_direction.length();
                let angle = line_direction.y.atan2(line_direction.x);

                // Spawn a quad as a line
                // Debugging output for line rendering between 
                /*
                println!(
                    "Drawing line from ({}, {}) to ({}, {})",
                    start.x, start.y, end.x, end.y
                );
                */

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(length, 2.0)), // Thin line, adjust thickness if needed
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(midpoint.x, midpoint.y, 0.5),
                        rotation: Quat::from_rotation_z(angle),
                        ..default()
                    },
                    ..default()
                });
            }
        }*/
    }
    
}

// Pellets
pub fn render_pellets(
    mut commands: Commands,
    maze: Res<Maze>,
) {
    for (&(x, y), &cell) in maze.grid.iter() {
        if let Some(pellet) = match cell {
            MazeCell::Path { has_pellet: true } => Some(Pellet::new(false)),
            MazeCell::PowerPellet => Some(Pellet::new(true)),
            _ => None,
        } {

            let splat_size = pellet.pellet_radius;
            println!("Pellet at X, Y (grid) {}, {}",x,y);
            println!(
                "Pellet at X, Y (trans) {}, {}",
                x as f32 * TILE_SIZE + X_OFFSET,
                -(y as f32 * TILE_SIZE) + Y_OFFSET
            );
            println!("Splat Size {}",splat_size);

            commands.spawn(
                PelletBundle{
                    pellet: pellet,
                    sprite: SpriteBundle {
                        sprite: Sprite {
                            color: WHITE,
                            custom_size: Some(Vec2::splat(splat_size)), // Adjust size as needed
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            x as f32 * TILE_SIZE + X_OFFSET,
                            -(y as f32 * TILE_SIZE) + Y_OFFSET,
                            2.0,
                        )),
                        ..default()
                    }
                }
            );
        }
    }
}

pub fn power_pellet_flash_system(
    time: Res<Time>,
    mut query: Query<(&mut Pellet, &mut Sprite), With<Pellet>>,
) {
    for (mut pellet, mut sprite) in query.iter_mut() {
        // Only apply flashing to power pellets
        if pellet.is_power && !pellet.is_eaten {
            // Increment the timer by the elapsed time since the last frame
            pellet.self_timer += time.delta_seconds();

            // Check if the timer has reached or exceeded the flash time
            if pellet.self_timer >= pellet.flash_time {
                // Toggle visibility
                pellet.flashing_off = !pellet.flashing_off;
                
                // Update the sprite's color to match visibility
                sprite.color = if !pellet.flashing_off {
                    pellet.color // Original color when visible
                } else {
                    Color::NONE // Transparent when invisible
                };

                // Reset the self_timer to start counting for the next flash
                pellet.self_timer = 0.0;
            }
        }
    }
}