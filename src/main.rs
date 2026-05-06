use raylib::prelude::*;
use rand::Rng;
use std::fmt::Write;

mod player;
mod bullet;
mod types;
mod enemy;

use crate::types::Circle;
use player::*;
use enemy::*;
use bullet::*;

fn draw_planet(d: &mut RaylibDrawHandle, cir: &Circle) {
    d.draw_circle(cir.center.x as i32, cir.center.y as i32, cir.radius, Color::ORANGE);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 960)
        .title("Rusty ahh window")
        .build();

    let width = rl.get_screen_width();
    let height = rl.get_screen_height();

    let mut planet = Circle {
        radius: 200.0,
        center: Vector2::new(width as f32 / 2.0, height as f32 / 2.0),
    };

    let mut player = Player {
        rect: Rectangle::new(
            0.0,
            0.0,
            100.0,
            100.0,
        ),
        hitbox: Circle::new(35.0, Vector2::new(0.0, 0.0)),
        angle: 0.0,
        radius: 270.0,
        theta: 0.0,
        speed: 5.0,
        score: 0,
    };

    let mut enemies: Vec<Enemy> = Vec::new();
    create_enemies(&mut enemies, 10);

    let mut bullets: Vec<Bullet> = Vec::new();

    let mut playerTex = rl.load_texture(&thread, "ufo.png").unwrap();
    let mut bulletTex = rl.load_texture(&thread, "bullet.png").unwrap();
    let mut rockTex   = rl.load_texture(&thread, "whiteRock.png").unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let dt = d.get_frame_time();

        d.clear_background(Color::WHITE);

        draw_planet(&mut d, &planet);

        update_player(&mut d, &mut player, &mut planet, dt);
        draw_player(&mut d, &player, &thread, &playerTex);

        draw_enemies(&mut d, &enemies, &rockTex);
        update_enemies(&mut d, &mut planet, &mut enemies, &mut player, dt);

        create_bullet(&mut d, &mut bullets, &player);
        // pub fn update_bullet(d: &mut RaylibDrawHandle, player: &mut Player, bullet: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>, planet: Circle, dt: f32) {
        update_bullet(&mut d, &mut player, &mut bullets, &mut enemies, &planet, dt);
        draw_bullet(&mut d, &bullets, &bulletTex);

        let mut s = String::new();
        write!(&mut s, "score = {}", player.score).unwrap();

        d.draw_text(&s, 10, 10, 32, Color::ORANGE);

    }
}
