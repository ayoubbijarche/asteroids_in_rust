use rand::prelude::*;
use raylib::prelude::*;

#[derive(Debug)]
pub struct Asteroids {
    pub sprite: Texture2D,
    pub pos: Vector2,
    pub vel: Vector2,
    pub rotation: f32,
    pub scale: f32,
    pub spawn_rate: f32,
}

impl Asteroids {
    pub fn init(rl: &mut RaylibHandle, thread: &RaylibThread) -> Asteroids {
        let mut rng = rand::rng();
        let paths = vec![
            "assets/roid1.png",
            "assets/roid2.png",
            "assets/roid3.png",
            "assets/roid4.png",
            "assets/roid5.png",
        ];

        let index = rng.random_range(0..paths.len());
        let texture = rl
            .load_texture(&thread, paths[index])
            .expect("failed to pick a random texture");
        let x_range = rng.random_range(0.0..800.0);
        let y_range = rng.random_range(0.0..600.0);
        let vx_range = rng.random_range(-150.0..150.0);
        let vy_range = rng.random_range(-150.0..150.0);

        Asteroids {
            sprite: texture,
            pos: Vector2::new(x_range, y_range),
            vel: Vector2::new(vx_range, vy_range),
            rotation: 0.0,
            scale: 0.5,
            spawn_rate: 3.0,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        self.rotation += 50.0 * rl.get_frame_time();

        if self.pos.x > 800.0 {
            self.pos.x = 0.1;
        } else if self.pos.x < -10.0 {
            self.pos.x = 799.0
        }

        if self.pos.y > 600.0 {
            self.pos.y = 0.0;
        } else if self.pos.y < -10.0 {
            self.pos.y = 599.0
        }

        self.pos += self.vel * rl.get_frame_time();
    }
}
