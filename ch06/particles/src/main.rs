use graphics::math::{add, mul_scalar, Vec2d};
use piston_window::{clear, rectangle, PistonWindow, WindowSettings};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

type RGBA = [f32; 4];
const BACKGROUND_COLOR: RGBA = [0.0; 4];
const GRAY: RGBA = [0.7, 0.7, 0.7, 0.3];
const N_PARTICLES: usize = 500;

struct Shape {
    width: f64,
    height: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: RGBA,
}

impl Shape {
    fn new(x: f64, y: f64) -> Shape {
        let mut rng = thread_rng();
        // Create random number generator within -5 and 5
        let legal_range = Uniform::new(-5_f64, 5_f64);

        let x_speed = legal_range.sample(&mut rng);
        let y_speed = legal_range.sample(&mut rng);
        let x_accel = 0.1 * legal_range.sample(&mut rng);
        let y_accel = 0.1 * legal_range.sample(&mut rng);

        Shape {
            width: 10.0,
            height: 10.0,
            position: [x, y],
            velocity: [x_speed, y_speed],
            acceleration: [x_accel, y_accel],
            color: GRAY,
        }
    }

    fn update(&mut self) {
        let acceleration_decay = 0.7;
        let alpha_decay = 0.95;

        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);
        self.acceleration = mul_scalar(self.acceleration, acceleration_decay);
        self.color[3] *= alpha_decay;
    }
}

struct World {
    current_turn: usize,
    shapes: Vec<Box<Shape>>,
    width: u32,
    height: u32,
}

impl World {
    fn new(width: u32, height: u32) -> World {
        World {
            current_turn: 0,
            shapes: Vec::new(),
            width: width,
            height: height,
        }
    }

    fn add_shapes(&mut self, n: usize) {
        let x = (self.width / 2) as f64;
        let y = (self.height / 2) as f64;

        for _ in 0..n {
            self.shapes.push(Box::new(Shape::new(x, y)));
        }
    }

    fn remove_shapes(&mut self, n: usize) {
        let n_shapes = self.shapes.len();
        let to_remove = if n > n_shapes { n_shapes } else { n };

        for _ in 0..to_remove {
            // Slow operation because every item will shift to empty slot
            self.shapes.remove(0);
        }

        // force re-allocation
        self.shapes.shrink_to_fit();
    }

    fn calc_population_change(&self) -> isize {
        const N: f64 = N_PARTICLES as f64;
        const MAX: f64 = N * 0.3;
        const MIN: f64 = -N * 0.3;

        let x: f64 = self.current_turn as f64;
        // Create sine wave pulse
        let n = 0.4 * N * (0.1 * x).sin() + 0.1 * N * x.sin();
        n.max(MIN).min(MAX).round() as isize
    }

    fn update(&mut self) {
        let n = self.calc_population_change();

        if n > 0 {
            self.add_shapes(n as usize);
        } else {
            self.remove_shapes(n.abs() as usize);
        }

        self.current_turn += 1;
    }
}

fn main() {
    let (width, height) = (640, 480);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window");
    let mut world = World::new(width, height);
    world.add_shapes(N_PARTICLES);

    while let Some(event) = window.next() {
        for shape in &mut world.shapes {
            shape.update();
        }

        world.update();

        window.draw_2d(&event, |ctx, renderer, _| {
            clear(BACKGROUND_COLOR, renderer);

            for s in &world.shapes {
                let rect = [s.position[0], s.position[1], s.width, s.height];
                let transformation_matrix = ctx.transform;
                rectangle(s.color, rect, transformation_matrix, renderer);
            }
        });
    }
}
