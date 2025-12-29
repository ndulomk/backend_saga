use macroquad::prelude::*;

const GRAVITY: f32 = 0.5;
const JUMP_FORCE: f32 = -12.0;
const MOVE_SPEED: f32 = 5.0;
const MAX_SPEED: f32 = 8.0;

struct Player {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    on_ground: bool,
    facing_right: bool,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 100.0,
            y: 300.0,
            vel_x: 0.0,
            vel_y: 0.0,
            on_ground: false,
            facing_right: true,
        }
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::Right) {
            self.vel_x = (self.vel_x + 0.5).min(MAX_SPEED);
            self.facing_right = true;
        } else if is_key_down(KeyCode::Left) {
            self.vel_x = (self.vel_x - 0.5).max(-MAX_SPEED);
            self.facing_right = false;
        } else {
            self.vel_x *= 0.9;
            if self.vel_x.abs() < 0.1 {
                self.vel_x = 0.0;
            }
        }

        if is_key_pressed(KeyCode::Space) && self.on_ground {
            self.vel_y = JUMP_FORCE;
            self.on_ground = false;
        }

        self.vel_y += GRAVITY;

        self.x += self.vel_x;
        self.y += self.vel_y;

        let ground_y = 400.0;
        if self.y >= ground_y {
            self.y = ground_y;
            self.vel_y = 0.0;
            self.on_ground = true;
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, 20.0, BLUE);

        let eye_offset = if self.facing_right { 8.0 } else { -8.0 };
        draw_circle(self.x + eye_offset, self.y - 5.0, 5.0, WHITE);
    }
}

#[macroquad::main("Sonic Rust")]
async fn main() {
    let mut player = Player::new();

    loop {
        clear_background(Color::from_rgba(135, 206, 235, 255)); 

        draw_rectangle(0.0, 420.0, screen_width(), 80.0, BROWN);

        draw_rectangle(0.0, 400.0, screen_width(), 20.0, GREEN);

        player.update();
        player.draw();

        draw_text(
            &format!("vel_x: {:.1} vel_y: {:.1}", player.vel_x, player.vel_y),
                  10.0, 30.0, 20.0, BLACK
        );

        next_frame().await
    }
}
