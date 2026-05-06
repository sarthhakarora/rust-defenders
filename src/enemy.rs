use raylib::prelude::*;
use rand::Rng;

use crate::types::Circle;
use crate::player::*;
use crate::bullet::*;

#[repr(i32)]
pub enum Type {
    ROCK_RED,
    ROCK_YELLOW,
    ROCK_GREEN,
}

impl Type{
    fn from_i32(x: i32) -> Option<Self> {
        match x {
            0 => Some(Self::ROCK_RED),
            1 => Some(Self::ROCK_YELLOW),
            2 => Some(Self::ROCK_GREEN),
            _ => None,
        }
    }
}

pub struct Enemy {
    pub rect: Rectangle,
    pub speed: f32,
    pub angle: f32,
    pub radius: f32,
    pub theta: f32,
    pub etype: Type,
    pub offset: Vector2,
}

pub fn spawn_outside_screen(rng: &mut impl Rng) -> (f32, f32) {
    let side = rng.gen_range(0..4);

    match side {
        // TOP
        0 => (
            rng.gen_range(0.0..1280.0),
            -30.0,
        ),

        // BOTTOM
        1 => (
            rng.gen_range(0.0..1280.0),
            960.0 + 30.0,
        ),

        // LEFT
        2 => (
            -30.0,
            rng.gen_range(0.0..960.0),
        ),

        // RIGHT
        _ => (
            1280.0 + 30.0,
            rng.gen_range(0.0..960.0),
        ),
    }
}

pub fn create_enemies(enemies: &mut Vec<Enemy>, n: i32) {
    let mut rng = rand::thread_rng();

    for i in 0..n {
        let (x, y) = spawn_outside_screen(&mut rng);
        let mut sizex = rng.gen_range(15.0..60.0);
        let mut sizey = rng.gen_range(15.0..60.0);

        enemies.push(Enemy {
            rect: Rectangle::new(
                x,
                y,
                sizex,
                sizey,
            ),
            speed:  rng.gen_range(30.0..200.0),
            angle:  rng.gen_range(0.0..360.0),
            radius: 15.0,
            theta:  0.0,
            etype: Type::from_i32(rng.gen_range(0..3)).expect("eType Failed"),
            offset: Vector2::new(0.0, 0.0),
        });
    }
}

pub fn draw_enemies(d: &mut RaylibDrawHandle, enemies: &Vec<Enemy>, texture: &Texture2D) {
    let src = Rectangle::new (
        0.0,
        0.0,
        texture.width as f32,
        texture.height as f32,
    );

    for i in 0..enemies.len() {
        let mut color: Color;

        match enemies[i].etype {
            Type::ROCK_RED      => {
                color = Color::RED;
            }
            Type::ROCK_YELLOW   => { 
                color = Color::new(255, 220, 90, 255);
            }
            Type::ROCK_GREEN    => {
                color = Color::GREEN;
            }
            _ => {
                color = Color::WHITE;
            }
        };

        d.draw_texture_pro(texture, src, enemies[i].rect, Vector2::new(enemies[i].rect.width/2.0, enemies[i].rect.height/2.0), enemies[i].angle, color);
    }
}

pub fn update_enemies(d: &mut RaylibDrawHandle, planet: &mut Circle, enemies: &mut Vec<Enemy>, bullets: &mut Vec<Bullet>, player: &mut Player, dt: f32) {
    let mut rng = rand::thread_rng();

    for i in 0..enemies.len() {
        let playerCenter = Vector2 {
            x: player.hitbox.center.x,
            y: player.hitbox.center.y 
        };
        let planetCenter = Vector2 {
            x: planet.center.x,
            y: planet.center.y 
        };
        let enemyCenter  = Vector2 {
            x: enemies[i].rect.x,
            y: enemies[i].rect.y,
        };

        enemies[i].angle += enemies[i].speed * dt;

        let playerRadius = player.hitbox.radius;
        let planetRadius = planet.radius;

        if enemies[i].rect.check_collision_circle_rec(planetCenter, planetRadius) {
            player.score -= 10;
            let (x, y) = spawn_outside_screen(&mut rng);
            enemies[i].rect.x = x;
            enemies[i].rect.y = y;
            enemies[i].offset.x = rng.gen_range(-200.0..200.0);
            enemies[i].offset.y = rng.gen_range(-200.0..200.0);
        }

        if enemies[i].rect.check_collision_circle_rec(playerCenter, playerRadius) {
            let mut contact_penalty: i32 = 0;

            match enemies[i].etype {
                Type::ROCK_RED      => {
                    contact_penalty = -100;
                }
                Type::ROCK_YELLOW   => {
                    contact_penalty = -50;
                }
                Type::ROCK_GREEN    => {
                    contact_penalty = 100;
                }
                _ => {
                    contact_penalty = 0;
                }
            };

            player.score -= contact_penalty;
            let (x, y) = spawn_outside_screen(&mut rng);
            enemies[i].rect.x = x;
            enemies[i].rect.y = y;
        }

        let dx = planet.center.x + enemies[i].offset.x - enemies[i].rect.x;
        let dy = planet.center.y + enemies[i].offset.y - enemies[i].rect.y;

        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 0.0 {
            let speed = enemies[i].speed;

            enemies[i].rect.x += (dx / dist) * speed * dt as f32;
            enemies[i].rect.y += (dy / dist) * speed * dt as f32;
        }
    }
}
