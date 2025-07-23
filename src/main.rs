use raylib::{prelude::*};
use crate::{asteroids::Asteroids, player::{Bullets, Player}};
mod player;
mod asteroids;
use raylib::audio::RaylibAudio;




fn main() {

    let (mut rl , thread) = raylib::init().size(800, 600).title("asteroids").build();
    let mut player = Player::init(&mut rl, &thread);
    let mut bullets : Vec<Bullets> = Vec::new();
    let mut asteroids : Vec<Asteroids> = Vec::new(); 
    let mut asteroid_spawn_timer = 0.0f32;

    let audio = RaylibAudio::init_audio_device().expect("error loading audio device");
    let shooting_sound = audio.new_sound("assets/shoot.wav").expect("shooting audio didn t load");
    let bg_music = audio.new_music("assets/bgmusic.wav").expect("couldnt load bg music");
    audio.set_master_volume(0.2);
    
    Music::play_stream(&bg_music);
    
    
    
    
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

        if asteroid_spawn_timer >= 3.0 {
            asteroids.push(Asteroids::init(&mut rl , &thread));
            println!("{:?}" , asteroids);
            asteroid_spawn_timer = 0.0;
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

        for bullet in &bullets{
            d.draw_texture_ex(&bullet.sprite, bullet.pos, bullet.rotation, bullet.scale, Color::WHITE);
        }

        for asteroid in &asteroids{
            d.draw_texture_ex(&asteroid.sprite, asteroid.pos, asteroid.rotation, asteroid.scale, Color::WHITE);
        }
    }
    
    

}