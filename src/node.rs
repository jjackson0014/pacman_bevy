use crate::prelude::*;

// Define direction constants
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


// Create individual Node Component
#[derive(Component)]
pub struct Node {
    position: Vec2,
    neighbors: HashMap<Direction, Option<Entity>>, // Neighbor nodes
}

impl Node {
    pub fn new(x: f32, y: f32) -> Self {
        Node {
            position: Vec2::new(x, y),
            neighbors: HashMap::from([
                (Direction::Up, None),
                (Direction::Down, None),
                (Direction::Left, None),
                (Direction::Right, None),
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
        let node_a = commands.spawn(Node::new(-80.0, -80.0)).id();
        let node_b = commands.spawn(Node::new(0.0, -80.0)).id();
        let node_c = commands.spawn(Node::new(-80.0, 0.0)).id();
        let node_d = commands.spawn(Node::new(0.0, 0.0)).id();
        let node_e = commands.spawn(Node::new(80.0, 0.0)).id();
        let node_f = commands.spawn(Node::new(-80.0, 160.0)).id();
        let node_g = commands.spawn(Node::new(80.0, 160.0)).id();

        // Store node entities in the node list
        self.node_list = vec![node_a, node_b, node_c, node_d, node_e, node_f, node_g];

        // Define neighbor relationships
        commands.entity(node_a).insert(Node {
            neighbors: HashMap::from([
                (Direction::Right, Some(node_b)),
                (Direction::Down, Some(node_c)),
                (Direction::Up, None),
                (Direction::Left, None),
            ]),
            ..Node::new(-80.0, -80.0)
        });
        
        commands.entity(node_b).insert(Node {
            neighbors: HashMap::from([
                (Direction::Left, Some(node_a)),
                (Direction::Down, Some(node_d)),
                (Direction::Up, None),
                (Direction::Right, None),
            ]),
            ..Node::new(0.0, -80.0)
        });
        
        commands.entity(node_c).insert(Node {
            neighbors: HashMap::from([
                (Direction::Up, Some(node_a)),
                (Direction::Right, Some(node_d)),
                (Direction::Down, Some(node_f)),
                (Direction::Left, None),
            ]),
            ..Node::new(-80.0, 0.0)
        });
        
        commands.entity(node_d).insert(Node {
            neighbors: HashMap::from([
                (Direction::Up, Some(node_b)),
                (Direction::Left, Some(node_c)),
                (Direction::Right, Some(node_e)),
                (Direction::Down, None),
            ]),
            ..Node::new(0.0, 0.0)
        });
        
        commands.entity(node_e).insert(Node {
            neighbors: HashMap::from([
                (Direction::Left, Some(node_d)),
                (Direction::Down, Some(node_g)),
                (Direction::Up, None),
                (Direction::Right, None),
            ]),
            ..Node::new(80.0, 0.0)
        });
        
        commands.entity(node_f).insert(Node {
            neighbors: HashMap::from([
                (Direction::Up, Some(node_c)),
                (Direction::Right, Some(node_g)),
                (Direction::Down, None),
                (Direction::Left, None),
            ]),
            ..Node::new(-80.0, 160.0)
        });
        
        commands.entity(node_g).insert(Node {
            neighbors: HashMap::from([
                (Direction::Up, Some(node_e)),
                (Direction::Left, Some(node_f)),
                (Direction::Right, None),
                (Direction::Down, None),
            ]),
            ..Node::new(80.0, 160.0)
        });
    }
}

// Systems to Render Nodes on Screen
pub fn setup_node_group(mut commands: Commands, mut node_group: ResMut<NodeGroup>) {
    node_group.setup_test_nodes(&mut commands);
}

pub fn render_nodes_as_quads(
    mut commands: Commands,
    query: Query<(Entity, &Node)>,
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
                let direction = end - start;
                let length = direction.length();
                let angle = direction.y.atan2(direction.x);

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