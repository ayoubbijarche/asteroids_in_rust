use raylib::prelude::*;

pub struct Player{
    pub sprite : Texture2D,
    pub pos : Vector2,
    pub vel : Vector2,
    pub rotation : f32,
    pub speed : f32,
    pub scale : f32
}


#[derive(Debug)]
pub struct Bullets{
    pub sprite : Texture2D,
    pub vel : Vector2,
    pub pos : Vector2,
    pub rotation : f32,
    pub scale : f32,
    pub lifetime : f32
}



impl Player{
    pub fn init(rl : &mut  RaylibHandle, thread : &RaylibThread) -> Player{
        let texture = rl.load_texture(&thread, "assets/player.png").expect("didnt load the player texture properly");
        
        Player{
            sprite : texture,
            pos : Vector2::new(400.0, 300.0),
            vel : Vector2::zero(),
            rotation : 0.0,
            speed : 150.0,
            scale : 0.4
        }
    }
    
    pub fn draw(&mut self , d : &mut RaylibDrawHandle){
        d.draw_texture_ex(&self.sprite, self.pos, self.rotation, self.scale , Color::WHITE);

    }

    pub fn update(&mut self , rl : &mut RaylibHandle ){
        
        if rl.is_key_down(KeyboardKey::KEY_RIGHT){
            self.rotation += 150.0 * rl.get_frame_time();
        }       
        
        if rl.is_key_down(KeyboardKey::KEY_LEFT){
            self.rotation -= 150.0 * rl.get_frame_time();
        }
        
        if self.rotation  > 360.0 || self.rotation < -360.0{
            self.rotation = 0.0;
        }
        
        if rl.is_key_down(KeyboardKey::KEY_UP){
            let angle_rad = self.rotation.to_radians() + 90.0;
            self.vel.x += -angle_rad.cos() * self.speed * rl.get_frame_time();
            self.vel.y += -angle_rad.sin() * self.speed * rl.get_frame_time();
        }
        
        self.pos += self.vel * rl.get_frame_time();
        
        
        
        if self.pos.x > 800.0 {
            self.pos.x = 0.0;
        }else if self.pos.x  < -10.0 {
            self.pos.x = 799.0
        }
        
        if self.pos.y > 600.0 {
            self.pos.y = 0.0;
        }else if self.pos.y  < -10.0 {
            self.pos.y = 599.0
        }
        
    }
    

    pub fn shoot(&self , rl : &mut RaylibHandle , thread : &RaylibThread) ->Bullets {
        let angle_rad = self.rotation.to_radians();
        let texture = rl.load_texture(&thread, "assets/bullet.png").expect("didnt load bullet texture properly");
        let bullet_speed = 550.0;
        Bullets {
            sprite: texture, 
            pos: self.pos, 
            vel : Vector2::new(angle_rad.sin(), -angle_rad.cos()) * bullet_speed,
            rotation : 0.0,
            scale: 0.5,
            lifetime : 5.0
        }
    }
}


