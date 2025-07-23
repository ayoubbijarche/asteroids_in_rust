use raylib::prelude::*;
use crate::{asteroids::Asteroids, player::{Bullets, Player}};
mod player;
mod asteroids;
use raylib::audio::RaylibAudio;


fn aabb_collision(pos1: Vector2, size1: Vector2, pos2: Vector2, size2: Vector2) -> bool {
    pos1.x < pos2.x + size2.x &&
    pos1.x + size1.x > pos2.x &&
    pos1.y < pos2.y + size2.y &&
    pos1.y + size1.y > pos2.y
}



fn main() {

    let (mut rl , thread) = raylib::init().size(800, 600).title("asteroids").build();
    let mut player = Player::init(&mut rl, &thread);
    let mut bullets : Vec<Bullets> = Vec::new();
    let mut asteroids : Vec<Asteroids> = Vec::new(); 
    let mut asteroid_spawn_timer = 0.0f32;
    
    let mut dead = false;
    
    let audio = RaylibAudio::init_audio_device().expect("error loading audio device");
    let shooting_sound = audio.new_sound("assets/shoot.wav").expect("shooting audio didn t load");
    let dead_sound = audio.new_sound("assets/dead.wav").expect("shooting audio didn t load");
    let bg_music = audio.new_music("assets/bgmusic.wav").expect("couldnt load bg music");
    audio.set_master_volume(0.2);

    Music::play_stream(&bg_music);


    let mut score = 0;


    if Music::is_stream_playing(&bg_music){
        println!("its playing");
    }else{
        println!("bg music is not playing")
    }

    while !rl.window_should_close(){
        player.update(&mut rl);
        Music::update_stream(&bg_music);
        
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE){
            bullets.push(player.shoot(&mut rl, &thread));
            println!("{:?}" , bullets);
            Sound::play(&shooting_sound);
        }
        


        for asteroid in asteroids.iter_mut(){
            asteroid.update(&mut rl);
            asteroid.spawn_rate -= rl.get_frame_time();

        }


        asteroid_spawn_timer += rl.get_frame_time();

        if asteroid_spawn_timer >= 2.0 {
            asteroids.push(Asteroids::init(&mut rl , &thread));
            println!("{:?}" , asteroids);
            asteroid_spawn_timer = 0.0;
        }

        for bullet in bullets.iter_mut(){
            bullet.pos += bullet.vel * rl.get_frame_time();
            bullet.lifetime -= rl.get_frame_time();
        }

        bullets.retain(|b| b.lifetime > 0.0);

        //player & asteroid collision
        for asteroid in asteroids.iter_mut(){
            if aabb_collision(player.pos, Vector2::new(player.sprite.width as f32 / 2.0, player.sprite.height as f32 / 2.0), asteroid.pos, Vector2::new(asteroid.sprite.width as f32 / 2.0, asteroid.sprite.height as f32 / 2.0)){
                println!("they collided sir");
                dead = true;
            }
        }
        
        //bullets and asteroids collision
        asteroids.retain(|asteroid| {
            let mut hit = false;
            for bullet in bullets.iter_mut(){
                if aabb_collision(asteroid.pos, Vector2::new(asteroid.sprite.width as f32 / 2.0, asteroid.sprite.height as f32 / 2.0) , bullet.pos, Vector2::new(bullet.sprite.width as f32 / 2.0, bullet.sprite.height as f32 / 2.0), ){
                    println!("they collided sir bullets");
                    score += 1;
                    hit = true;
                    break;
                }
            }
            !hit
        });
        


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("Score : ", 650 , 20, 20, Color::WHITE);
        d.draw_text(&score.to_string(), 730 , 20, 20, Color::WHITE);
        
        if dead == false{
            player.draw(&mut d);
        }
        
        if dead == true {
            d.draw_text("You are Dead", 250 , 320, 40, Color::WHITE);
        }

        for bullet in &bullets{
            d.draw_texture_ex(&bullet.sprite, bullet.pos, bullet.rotation, bullet.scale, Color::WHITE);
        }

        for asteroid in &asteroids{
            d.draw_texture_ex(&asteroid.sprite, asteroid.pos, asteroid.rotation, asteroid.scale, Color::WHITE);
        }
    }


}
