use raylib::prelude::*;

use crate::player::{Bullets, Player};
mod player;



fn main() {
    let (mut rl , thread) = raylib::init().size(800, 600).title("asteroids").build();
    let mut player = Player::init(&mut rl, &thread);
    let mut bullets : Vec<Bullets> = Vec::new();

    while !rl.window_should_close(){
        player.update(&mut rl);
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE){
            bullets.push(player.shoot(&mut rl, &thread));
            println!("{:?}" , bullets)
        }
        
        for bullet in bullets.iter_mut(){
            bullet.pos += bullet.vel * rl.get_frame_time();
        }
        
        /* 
        bullets.retain(|b| b.pos.x > 800.0);
        bullets.retain(|b| b.pos.x <  -10.0);
        bullets.retain(|b| b.pos.y > 600.0);
        bullets.retain(|b| b.pos.y <  -10.0);
        */
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("Score : ",300 , 50, 20, Color::WHITE);
        player.draw(&mut d);
        
        for bullet in &bullets{
            d.draw_texture_ex(&bullet.sprite, bullet.pos, bullet.rotation, bullet.scale, Color::WHITE);
        }
    }

}