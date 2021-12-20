use winit::{
    event_loop::{EventLoop},
    window::{WindowBuilder},
};

mod state;
mod events;
mod vertex;
mod utils;

use state::State;
use vertex::Vertex;
use events::listen;
use utils::random_color;

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
    /* padding */ 0,
];

fn main() {
    let vertices: &[Vertex] = &[
        Vertex { position: [-0.0868241, 0.49240386, 0.0], color: random_color() }, // A
        Vertex { position: [-0.49513406, 0.06958647, 0.0], color: random_color() }, // B
        Vertex { position: [-0.21918549, -0.44939706, 0.0], color: random_color() }, // C
        Vertex { position: [0.35966998, -0.3473291, 0.0], color: random_color() }, // D
        Vertex { position: [0.44147372, 0.2347359, 0.0], color: random_color() }, // E
    ];
    
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let state = pollster::block_on(State::new(&window, vertices, INDICES));

    listen(event_loop, window, state);
}