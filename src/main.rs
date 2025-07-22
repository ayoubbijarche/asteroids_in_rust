use raylib::prelude::*;
use crate::{asteroids::Asteroids, player::{Bullets, Player}};
mod player;
mod asteroids;


fn main() {
    let (mut rl , thread) = raylib::init().size(800, 600).title("asteroids").build();
    let mut player = Player::init(&mut rl, &thread);
    let mut asteroids = Asteroids::init(&mut rl , &thread);
    let mut bullets : Vec<Bullets> = Vec::new();

    while !rl.window_should_close(){
        player.update(&mut rl);
        asteroids.update(&mut rl);
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE){
            bullets.push(player.shoot(&mut rl, &thread));
            println!("{:?}" , bullets)
        }
        
        for bullet in bullets.iter_mut(){
            bullet.pos += bullet.vel * rl.get_frame_time();
            bullet.lifetime -= rl.get_frame_time();
        }
        
        bullets.retain(|b| b.lifetime > 0.0);
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("Score : 0", 650 , 20, 20, Color::WHITE);
        player.draw(&mut d);
        asteroids.draw(&mut d);
        
        for bullet in &bullets{
            d.draw_texture_ex(&bullet.sprite, bullet.pos, bullet.rotation, bullet.scale, Color::WHITE);
        }
    }

}