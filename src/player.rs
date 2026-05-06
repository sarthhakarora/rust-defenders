use raylib::prelude::*;

use crate::types::Circle;

pub struct Player {
    pub rect: Rectangle,
    pub hitbox: Circle,
    pub angle: f32,
    pub radius: f32,
    pub theta: f32,
    pub speed: f32,
    pub score: i32,
}

pub fn update_player(d: &mut RaylibDrawHandle, player: &mut Player, planet: &mut Circle, dt: f32) {
    if d.is_key_down(KeyboardKey::KEY_D) || d.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.theta += player.speed * dt as f32;
    }

    if d.is_key_down(KeyboardKey::KEY_A) || d.is_key_down(KeyboardKey::KEY_LEFT) {
        player.theta -= player.speed * dt as f32;
    }

    let cx = planet.center.x;
    let cy = planet.center.y;

    let dx = cx - player.rect.x;
    let dy = cy - player.rect.y;

    let rotation_deg = (dy.atan2(-dx)).to_degrees();

    player.rect.x = cx + player.radius * player.theta.cos();
    player.rect.y = cy + player.radius * player.theta.sin();

    player.hitbox.center = Vector2::new(player.rect.x, player.rect.y);

    player.angle = -rotation_deg + 90.0;
}

pub fn draw_player(d: &mut RaylibDrawHandle, player: &Player, thread: &RaylibThread, texture: &Texture2D){
    let src = Rectangle::new (
        0.0,
        0.0,
        texture.width as f32,
        texture.height as f32,
    );

    d.draw_texture_pro(texture, src, player.rect, Vector2::new(player.rect.width/2.0, player.rect.height/2.0), player.angle, Color::WHITE);
}
