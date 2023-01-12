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
        Self {
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

    pub fn detect_collisions(&mut self) {
        self.objects
            .iter_mut()
            .for_each(|object| object.is_colliding = false);

        for i in 0..self.objects.len() {
            for j in (i + 1)..self.objects.len() {
                if self.objects[i].collides_with(&self.objects[j]) {
                    // log!("collides: {:?}", self.particles[i].collides_with(&self.particles[j]));
                    self.objects[i].is_colliding = true;
                    self.objects[j].is_colliding = true;

                    let v_collision = Vector {
                        x: self.objects[j].x - self.objects[i].x,
                        y: self.objects[j].y - self.objects[i].y,
                    };

                    let distance = (
                        (self.objects[j].x - self.objects[i].x) * (self.objects[j].x - self.objects[i].x)
                        +
                        (self.objects[j].y - self.objects[i].y) * (self.objects[j].y - self.objects[i].y)
                    ).sqrt();
                    let v_collision_norm = Vector {
                        x: v_collision.x / distance,
                        y: v_collision.y / distance
                    };
                    let v_relative_velocity = Vector {
                        x: self.objects[i].vx - self.objects[j].vx,
                        y: self.objects[i].vy - self.objects[j].vy
                    };
                    let speed = v_relative_velocity.x * v_collision_norm.x + v_relative_velocity.y * v_collision_norm.y;

                    if speed < 0.0 {
                        break;
                    }

                    let impulse = 2.0 * speed / (self.objects[i].size + self.objects[j].size);
                    self.objects[i].vx -= impulse * self.objects[j].size * v_collision_norm.x;
                    self.objects[i].vy -= impulse * self.objects[j].size * v_collision_norm.y;
                    self.objects[j].vx += impulse * self.objects[i].size * v_collision_norm.x;
                    self.objects[j].vy += impulse * self.objects[i].size * v_collision_norm.y;
                }
            }
        }
    }

    pub fn render(&self, render_object: fn(&Square)) {
        self.objects
            .iter()
            .for_each(render_object);
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
    pub x: f64,
    pub y: f64,
    vx: f64,
    vy: f64,
    pub size: f64,
    pub color: String,
    is_colliding: bool,
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
            is_colliding: false,
        }
    }

    fn collides_with(&self, other: &Square) -> bool {
        !(other.x > self.size + self.x
            || self.x > other.size + other.x
            || other.y > self.size + self.y
            || self.y > other.size + other.y)
    }

    fn update(&mut self, seconds_passed: f64) {
        self.x += self.vx * seconds_passed;
        self.y += self.vy * seconds_passed;
    }
}

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

struct Vector {
    x: f64,
    y: f64,
}
