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


fn draw_planet(d: &mut RaylibDrawHandle, cir: &Circle, texture: &Texture2D, angle: f32) {
    //d.draw_circle_lines(cir.center.x as i32, cir.center.y as i32, cir.radius, Color::ORANGE);
    let src = Rectangle::new (
        0.0,
        0.0,
        texture.width as f32,
        texture.height as f32,
    );
    let rect: Rectangle = Rectangle::new(cir.center.x, cir.center.y, cir.radius*2.0, cir.radius*2.0);

    d.draw_texture_pro(texture, src, rect, Vector2::new(rect.width/2.0, rect.height/2.0), angle, Color::WHITE);
}

fn draw_bg(d: &mut RaylibDrawHandle, texture: &Texture2D) {
    //d.draw_circle_lines(cir.center.x as i32, cir.center.y as i32, cir.radius, Color::ORANGE);
    let src = Rectangle::new (
        0.0,
        0.0,
        texture.width as f32,
        texture.height as f32,
    );

    let rect: Rectangle = Rectangle::new(
        0.0,
        0.0,
        1280.0,
        960.0
    );

    d.draw_texture_pro(texture, src, rect, Vector2::new(0.0, 0.0), 0.0, Color::WHITE);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 960)
        .title("Rust defenders")
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
        speed: 2.0,
        score: 0,
    };

    let mut angle: f32 = 0.0;

    let mut enemies: Vec<Enemy> = Vec::new();
    create_enemies(&mut enemies, 10);

    let mut bullets: Vec<Bullet> = Vec::new();

    let playerTex = rl.load_texture(&thread, "ufo.png").unwrap();
    let bulletTex = rl.load_texture(&thread, "bullet.png").unwrap();
    let rockTex   = rl.load_texture(&thread, "whiteRock.png").unwrap();
    let planetTex = rl.load_texture(&thread, "rustplanet.png").unwrap();
    let bgTex     = rl.load_texture(&thread, "bg.png").unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let dt = d.get_frame_time();

        d.clear_background(Color::WHITE);


        angle += 10.0 * dt;
        draw_bg(&mut d, &bgTex);

        update_player(&mut d, &mut player, &mut planet, dt);
        draw_player(&mut d, &player, &thread, &playerTex);

        draw_enemies(&mut d, &enemies, &rockTex);
        update_enemies(&mut d, &mut planet, &mut enemies, &mut bullets, &mut player, dt);

        create_bullet(&mut d, &mut bullets, &player);
        // pub fn update_bullet(d: &mut RaylibDrawHandle, player: &mut Player, bullet: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>, planet: Circle, dt: f32) {
        update_bullet(&mut d, &mut player, &mut bullets, &mut enemies, &planet, dt);
        draw_bullet(&mut d, &bullets, &bulletTex);

        draw_planet(&mut d, &planet, &planetTex, angle);

        let mut s = String::new();
        write!(&mut s, "score = {}", player.score).unwrap();

        d.draw_text(&s, 10, 10, 32, Color::ORANGE);

    }
}
