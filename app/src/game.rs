use rand::Rng;

pub struct Game {
    object_size: f64,
    seconds_passed: f64,
    old_time_stamp: f64,
    width: f64,
    height: f64,
    objects: Vec<Square>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            object_size: 20.0,
            seconds_passed: 0.0,
            old_time_stamp: 0.0,
            width: 0.0,
            height: 0.0,
            objects: Vec::new(),
        }
    }

    pub fn init(&mut self, width: f64, height: f64) -> &Self {
        // log!("width: {:?}", width);
        self.seconds_passed = 0.0;
        self.old_time_stamp = 0.0;
        self.width = width;
        self.height = height;
        self.init_objects();
        self
    }

    pub fn update(&mut self, time_stamp: f64) {
        self.seconds_passed = (time_stamp - self.old_time_stamp) / 1000.0;
        self.old_time_stamp = time_stamp;

        for object in self.objects.iter_mut() {
            object.update(self.seconds_passed)
        }
    }

    pub fn render(&self, render_object: fn(&Square)) {
        for object in self.objects.iter() {
            render_object(object);
        }
    }

    fn init_objects(&mut self) {
        self.objects.clear();
        for row in 1..(self.width / self.object_size / 4.0) as i64 {
            for column in 1..(self.height / self.object_size / 4.0) as i64 {
                self.objects
                    .push(Square::new(row as f64 * self.object_size * 4.0,
                                      column as f64 * self.object_size * 4.0,
                                      self.object_size));
            }
        }
        // log!("len: {:?}", self.objects.len());
    }
}

#[derive(PartialEq)]
pub struct Square {
    pub(crate) x: f64,
    pub(crate) y: f64,
    vx: f64,
    vy: f64,
    pub(crate) size: f64,
    pub(crate) color: String,
}

impl Square {
    fn new(x: f64, y: f64, size: f64) -> Square {
        let mut rng = rand::thread_rng();

        Square {
            x,
            y,
            vx: 250.0 * rng.gen::<f64>() - 125.0,
            vy: 250.0 * rng.gen::<f64>() - 125.0,
            size,
            color: get_random_rgb(),
        }
    }

    fn collides_with(&self, other: &Square) -> bool {
        self.x < other.x + other.size &&
            self.x + self.size > other.x &&
            self.y < other.y + other.size &&
            self.size + self.y > other.y
    }

    fn invert_vx(&mut self) {
        self.vx = -self.vx;
    }

    fn invert_vy(&mut self) {
        self.vy = -self.vy;
    }

    fn update(&mut self, seconds_passed:f64) {
        self.x += self.vx * seconds_passed;
        self.y += self.vy * seconds_passed;
    }
}

//Particle
// #[derive(PartialEq)]
// struct Particle {
//     x: f64,
//     y: f64,
//     vx: f64,
//     vy: f64,
//     size: i32,
//     color: String,
// }
//
// impl Particle {
//     fn new(x: f64, y: f64, size: i32) -> Particle {
//         let mut rng = rand::thread_rng();
//
//         Particle {
//             x,
//             y,
//             vx: 4.0 * rng.gen::<f64>() - 2.0,
//             vy: 4.0 * rng.gen::<f64>() - 2.0,
//             size,
//             color: get_random_rgb(),
//         }
//     }
//
//     fn collides_with(&self, other: &Particle) -> bool {
//         self.x < other.x + other.size as f64 &&
//             self.x + self.size as f64 > other.x &&
//             self.y < other.y + other.size as f64 &&
//             self.size as f64 + self.y > other.y
//     }
//
//     fn invert_vx(&mut self) {
//         self.vx = -self.vx;
//     }
//
//     fn invert_vy(&mut self) {
//         self.vy = -self.vy;
//     }
//
//     fn update(&mut self, seconds_passed:f64) {
//         self.x += self.vx * seconds_passed;
//         self.y += self.vy * seconds_passed;
//     }
// }
//
// //Particles
// pub struct Particles {
//     particles: Vec<Particle>,
//     canvas_width: f64,
//     canvas_height: f64,
// }
//
// impl Particles {
//     fn create(&mut self, size: i32) {
//         for row in 1..self.canvas_width as i32 / size / 4 {
//             for column in 1..self.canvas_height as i32 / size / 4 {
//                 self.particles
//                     .push(Particle::new((row * size * 4) as f64, (column * size * 4) as f64, size));
//             }
//         }
//     }
//
//     fn update(&mut self) {
//         for particle in self.particles.iter_mut() {
//             // particle.update();
//             if particle.x < 0.0 || particle.x > self.canvas_width {
//                 particle.invert_vx();
//             }
//
//             if particle.y < 0.0 || particle.y > self.canvas_height {
//                 particle.invert_vy();
//             }
//         }
//
//         for i in 0..self.particles.len() {
//             for j in (i + 1)..self.particles.len() {
//                 if self.particles[i].collides_with(&self.particles[j]) {
//                     // log!("collides: {:?}", self.particles[i].collides_with(&self.particles[j]));
//                     self.particles[i].invert_vx();
//                     self.particles[i].invert_vy();
//                     self.particles[j].invert_vx();
//                     self.particles[j].invert_vy();
//                 }
//             }
//         }
//
//         // println!("particles: {:?}", self.particles);
//     }
// }

//Helper function
fn get_random_rgb() -> String {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut rng = rand::thread_rng();

    while r < 100 && g < 100 && b < 100 {
        r = rng.gen_range(0..256);
        g = rng.gen_range(0..256);
        b = rng.gen_range(0..256);
    }
    format!("rgb({r},{g},{b})")
}