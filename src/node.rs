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

// Create individual Node Component
#[derive(Component)]
pub struct MapNode {
    pub position: Vec2,
    pub neighbors: HashMap<PacManDirection, Option<Entity>>, // Neighbor nodes
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
        }
    }
}

// Group Nodes together
#[derive(Resource)]
pub struct NodeGroup {
    node_list: Vec<Entity>,
}

impl NodeGroup {
    pub fn new() -> Self {
        NodeGroup {
            node_list: Vec::new(),
        }
    }
    pub fn setup_test_nodes(&mut self, commands: &mut Commands) {
        // Create nodes with specified positions
        let node_a = commands.spawn(MapNode::new(-80.0, -80.0)).id();
        let node_b = commands.spawn(MapNode::new(0.0, -80.0)).id();
        let node_c = commands.spawn(MapNode::new(-80.0, 0.0)).id();
        let node_d = commands.spawn(MapNode::new(0.0, 0.0)).id();
        let node_e = commands.spawn(MapNode::new(80.0, 0.0)).id();
        let node_f = commands.spawn(MapNode::new(-80.0, 160.0)).id();
        let node_g = commands.spawn(MapNode::new(80.0, 160.0)).id();

        // Store node entities in the node list
        self.node_list = vec![node_a, node_b, node_c, node_d, node_e, node_f, node_g];

        // Define neighbor relationships
        commands.entity(node_a).insert(MapNode {
            neighbors: HashMap::from([
                (PacManDirection::Right, Some(node_b)),
                (PacManDirection::Down, None),
                (PacManDirection::Up, Some(node_c)),
                (PacManDirection::Left, None),
            ]),
            ..MapNode::new(-80.0, -80.0)
        });
        
        commands.entity(node_b).insert(MapNode {
            neighbors: HashMap::from([
                (PacManDirection::Left, Some(node_a)),
                (PacManDirection::Down, None),
                (PacManDirection::Up, Some(node_d)),
                (PacManDirection::Right, None),
            ]),
            ..MapNode::new(0.0, -80.0)
        });
        
        commands.entity(node_c).insert(MapNode {
            neighbors: HashMap::from([
                (PacManDirection::Up, Some(node_f)),
                (PacManDirection::Right, Some(node_d)),
                (PacManDirection::Down, Some(node_a)),
                (PacManDirection::Left, None),
            ]),
            ..MapNode::new(-80.0, 0.0)
        });
        
        commands.entity(node_d).insert(MapNode {
            neighbors: HashMap::from([
                (PacManDirection::Up, None),
                (PacManDirection::Left, Some(node_c)),
                (PacManDirection::Right, Some(node_e)),
                (PacManDirection::Down, Some(node_b)),
            ]),
            ..MapNode::new(0.0, 0.0)
        });
        
        commands.entity(node_e).insert(MapNode {
            neighbors: HashMap::from([
                (PacManDirection::Left, Some(node_d)),
                (PacManDirection::Down, None),
                (PacManDirection::Up, Some(node_g)),
                (PacManDirection::Right, None),
            ]),
            ..MapNode::new(80.0, 0.0)
        });
        
        commands.entity(node_f).insert(MapNode {
            neighbors: HashMap::from([
                (PacManDirection::Up, None),
                (PacManDirection::Right, Some(node_g)),
                (PacManDirection::Down, Some(node_c)),
                (PacManDirection::Left, None),
            ]),
            ..MapNode::new(-80.0, 160.0)
        });
        
        commands.entity(node_g).insert(MapNode {
            neighbors: HashMap::from([
                (PacManDirection::Up, None),
                (PacManDirection::Left, Some(node_f)),
                (PacManDirection::Right, None),
                (PacManDirection::Down, Some(node_e)),
            ]),
            ..MapNode::new(80.0, 160.0)
        });
    }
}

// Systems to Render Nodes on Screen
pub fn setup_node_group(mut commands: Commands, mut node_group: ResMut<NodeGroup>) {
    node_group.setup_test_nodes(&mut commands);
}

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
                color: RED,
                custom_size: Some(Vec2::splat(16.0)), // Adjust size as needed
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(node.position.x, node.position.y, 0.5)),
            ..Default::default()
        });

        // Draw lines to each neighbor using a quad
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
                // Debugging output for line rendering between nodes
                println!(
                    "Drawing line from ({}, {}) to ({}, {})",
                    start.x, start.y, end.x, end.y
                );

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(length, 2.0)), // Thin line, adjust thickness if needed
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(midpoint.x, midpoint.y, 0.5),
                        rotation: Quat::from_rotation_z(angle),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }
    }
}