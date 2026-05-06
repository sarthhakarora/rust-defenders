use raylib::prelude::*;

use rand::Rng;
use crate::types::Circle;
use crate::player::*;
use crate::enemy::*;

pub struct Bullet{
    pub rect: Rectangle,
    pub speed: f32,
    pub vel: Vector2,
    pub acceleration: Vector2,
    pub angle: f32,
    pub theta: f32,
}

pub fn create_bullet(d: &mut RaylibDrawHandle, bullet: &mut Vec<Bullet>, player: &Player) {
    if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        let mut rng = rand::thread_rng();

        bullet.push(Bullet {
            rect: Rectangle::new(
                player.rect.x,
                player.rect.y,
                30.0,
                20.0,
            ),
            speed: rng.gen_range(50.0..150.0),
            vel: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            angle: player.angle - 90.0,
            theta: 0.0,
        });
        println!("bullet count: {}", bullet.len());
        println!("speed: {}", bullet[bullet.len()-1].speed);
    }
}

pub fn update_bullet(d: &mut RaylibDrawHandle, player: &mut Player, bullet: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>, planet: &Circle, dt: f32) {
    let mut rng = rand::thread_rng();

    let playerCenter = Vector2 { 
        x: player.hitbox.center.x,
        y: player.hitbox.center.y 
    };
    let playerRadius = player.hitbox.radius;

    for i in 0..enemies.len() {
        let enemyCenter  = Vector2 { 
            x: enemies[i].rect.x,
            y: enemies[i].rect.y,
        };
        let enemyRadius  = enemies[i].radius;

        bullet.retain(|b| {
            b.rect.x >= -100.0 && b.rect.x <= 1380.0 &&
            b.rect.y >= -100.0 && b.rect.y <= 1060.0
        });

        for j in 0..bullet.len() {

            if bullet[j].rect.check_collision_circle_rec(enemyCenter, enemyRadius) {
                player.score += 100;
                enemies[i].rect.x = spawn_outside_screen(&mut rng).0;
                enemies[i].rect.y = spawn_outside_screen(&mut rng).1;
            }

            let angle_rad = bullet[j].angle.to_radians();

            bullet[j].vel = Vector2 {
                x: angle_rad.cos(),
                y: angle_rad.sin(),
            };

            bullet[j].rect.x += bullet[j].vel.x * bullet[j].speed * dt;
            bullet[j].rect.y += bullet[j].vel.y * bullet[j].speed * dt;
        }
    }
}

pub fn draw_bullet(d: &mut RaylibDrawHandle, bullet: &Vec<Bullet>, texture: &Texture2D) {
    let src = Rectangle::new (
        0.0,
        0.0,
        texture.width as f32,
        texture.height as f32,
    );

    for i in 0..bullet.len() {
        d.draw_texture_pro(texture, src, bullet[i].rect, Vector2::new(bullet[i].rect.width/2.0, bullet[i].rect.height/2.0), bullet[i].angle, Color::RED);
    }
}
