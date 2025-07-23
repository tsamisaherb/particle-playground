//lib.rs

mod particles;
use particles::*;

const TRAIL_CIRCLE_POSITION: (f32, f32) = (40.0, 162.0);
const EXPLOSION_SQUARE_POSITION: (f32, f32) = (144.0, 168.0);
const DUST_SQUARE_POSITION: (f32, f32) = (330.0, 168.0);


use turbo::*;

#[turbo::game]
struct GameState {
    particle_manager: ParticleManager,
    trail_circle: TrailCircle,
    explosion_square: ExplosionSquare,
    dust_square: DustSquare,
  }
impl GameState {
    fn new() -> Self {
        Self {
            particle_manager: ParticleManager::new(),
            trail_circle: TrailCircle::new(TRAIL_CIRCLE_POSITION.0, TRAIL_CIRCLE_POSITION.1, 16.0, 0x8833AAff),
            explosion_square: ExplosionSquare::new(EXPLOSION_SQUARE_POSITION.0, EXPLOSION_SQUARE_POSITION.1, 16.0, 0xcc3333ff),
            dust_square: DustSquare::new(DUST_SQUARE_POSITION.0, DUST_SQUARE_POSITION.1, 16.0, 0x3333CCff, 186.0),
        }
    }

    fn update(&mut self) {
        clear(0x000000ff);
        let p = pointer::screen();
        let viewport = bounds::screen();
        let bottom_buttons = viewport
            .height(32)
            .inset_left(8)
            .inset_right(8)
            .anchor_bottom(&viewport)
            .inset_bottom(12)
            .columns_with_gap(4, 12);
        for (i, btn) in bottom_buttons.into_iter().enumerate() {
            let label = match i {
                0 => "Trail",
                1 => "Explosion",
                2 => "Confetti",
                3 => "Dust",
                _ => "",
            };
            let (regular_color, hover_color) = match i {
                0 => (0x8833AAff, 0xAA55CCff),
                1 => (0xCC3333ff, 0xFF5555ff),
                2 => (0x33CCFFff, 0x66DDFFFF),
                3 => (0x3333CCff, 0x5555FFff),
                _ => (0x3333CCff, 0x5555FFff),
            };
            let is_btn_hovered = p.intersects_bounds(btn);   
            rect!(
                color = if is_btn_hovered {
                    hover_color
                } else if p.just_pressed_bounds(btn) {
                    hover_color
                } else {
                    regular_color
                },
                w = btn.w(),
                h = btn.h(),
                x = btn.x(),
                y = btn.y(),
                border_radius = 2,
            );
            let btn_inner = btn.inset_left(4).inset_top(4);
            text!(label, x = btn_inner.x(), y = btn_inner.y(), font = "medium");
            if p.just_pressed_bounds(btn) {
                match i {
                    0 => {
                        if !self.trail_circle.active {
                            // Activate the trail circle when clicking fireworks button
                            self
                                .trail_circle
                                .activate(TRAIL_CIRCLE_POSITION.0, TRAIL_CIRCLE_POSITION.1);
                        }
                    }
                    1 => {
                        if self.explosion_square.trigger() {
                            // Create burst at the square's position
                            self
                                .particle_manager
                                .create_burst(&explosion(self.explosion_square.pos));
                        }
                    }
                    2 => {
                        self.particle_manager.create_burst(&confetti(0x33CCFFff));
                        self.particle_manager.create_burst(&confetti(0xAA55CCff));
                        self.particle_manager.create_burst(&confetti(0x5555FFff));
                    }
                    3 => {
                        self.dust_square.trigger();
                    }
                    _ => {
                        //do nothing
                    }
                };
            };
        }

        let mut make_trail = false;
        if self.trail_circle.active {
            make_trail = self.trail_circle.update();
        }
        self.trail_circle.draw();
        if make_trail {
            let trail_pos = (
                self.trail_circle.pos.0 + self.trail_circle.size / 2.0,
                self.trail_circle.pos.1 + self.trail_circle.size / 2.0,
            );
            self.particle_manager.create_burst(&trail(trail_pos));
        }

        self.explosion_square.update();
        self.explosion_square.draw();

        if self.dust_square.update() {
            // Create dust particles when the Square hits the 'ground'.
            let dust_pos = (
                self.dust_square.pos.0,
                self.dust_square.pos.1 + self.dust_square.size / 2.0,
            );
            self.particle_manager.create_burst(&dust(dust_pos));
        }
        self.dust_square.draw();

        // Update and draw particles
        self.particle_manager.update();
        self.particle_manager.draw();
    }
}


fn trail(pos: (f32, f32)) -> BurstConfig {
    BurstConfig {
        source: BurstSource::Point(pos.0, pos.1),
        x_velocity: (-0.25, 0.25),
        y_velocity: (1.0, 2.0),
        lifetime: (0.5, 0.8),
        color: 0xAA55CCff,
        size: (2, 3),
        count: 10,
        shape: Shape::Square,
        should_fade_out: true,
    }
}

fn dust(pos: (f32, f32)) -> BurstConfig {
    BurstConfig {
        source: BurstSource::Rectangle {
            min: (pos.0 - 2.0, pos.1 - 2.0),
            max: (pos.0 + 2.0, pos.1 - 2.0),
        },
        x_velocity: (-0.2, 0.2),
        y_velocity: (0.0, 0.1),
        lifetime: (0.2, 2.0),
        color: 0x777777FF,
        size: (1, 3),
        count: (200),
        shape: Shape::Square,
        should_fade_out: true,
    }
}

fn explosion(pos: (f32, f32)) -> BurstConfig {
    BurstConfig {
        source: BurstSource::Circle {
            center: pos,
            radius: 1.0,
        },
        x_velocity: (-1.0, 1.0),
        y_velocity: (-1.0, 1.0),
        lifetime: (0.4, 0.4),
        color: 0xFF6633ff,
        size: (4, 6),
        count: 10,
        shape: Shape::Square,
        should_fade_out: false,
    }
}

fn confetti(color: u32) -> BurstConfig {
    BurstConfig {
        source: BurstSource::Rectangle {
            min: (0.0, -10.0),
            max: (384.0, 0.0),
        },
        x_velocity: (-0.5, 0.5),
        y_velocity: (1.0, 2.0),
        lifetime: (2.0, 5.0),
        color: color,
        size: (2, 6),
        count: 100,
        shape: Shape::Circle,
        should_fade_out: false,
    }
}

#[turbo::serialize]
pub struct DustSquare {
    pub pos: (f32, f32),
    pub size: f32,
    pub color: u32,
    pub active: bool,
    pub falling: bool,
    pub target_y: f32,
    pub velocity: f32,
    pub reset_timer: u32,
    pub start_y: f32,
}

impl DustSquare {
    pub fn new(x: f32, y: f32, size: f32, color: u32, target_y: f32) -> Self {
        Self {
            pos: (x, y),
            size,
            color,
            active: true,
            falling: false,
            target_y,
            velocity: 0.0,
            reset_timer: 0,
            start_y: y,
        }
    }

    pub fn update(&mut self) -> bool {
        let mut triggered = false;

        // Handle reset timer
        if self.reset_timer > 0 {
            self.reset_timer -= 1;
            if self.reset_timer == 0 {
                self.reset();
            }
            return false;
        }

        if !self.active {
            return triggered;
        }

        if self.falling {
            // Apply gravity
            self.velocity += 0.1;
            self.pos.1 += self.velocity;

            // Check if bottom of square is at the target
            let bottom_y = self.pos.1 + self.size / 2.0;
            if bottom_y >= self.target_y {
                // Position precisely at target
                self.pos.1 = self.target_y - self.size / 2.0;
                self.falling = false;
                //self.active = false;
                self.reset_timer = 120; // Reset after 2 seconds (120 frames)
                return true; // Trigger dust burst
            }
        }

        return triggered;
    }

    pub fn trigger(&mut self) {
        if self.active && !self.falling {
            self.falling = true;
            self.velocity = 0.5; // Initial velocity
        }
    }

    pub fn reset(&mut self) {
        self.pos.1 = self.start_y;
        self.active = true;
        self.falling = false;
        self.velocity = 0.0;
    }

    pub fn draw(&self) {
        if self.active {
            rect!(
                x = self.pos.0 - self.size / 2.0, // Center the rectangle
                y = self.pos.1 - self.size / 2.0,
                w = self.size,
                h = self.size,
                color = self.color
            );
        }
    }
}

#[turbo::serialize]
pub struct ExplosionSquare {
    pub pos: (f32, f32),
    pub size: f32,
    pub color: u32,
    pub visible: bool,
    pub cooldown: u32,
}

impl ExplosionSquare {
    pub fn new(x: f32, y: f32, size: f32, color: u32) -> Self {
        Self {
            pos: (x, y),
            size,
            color,
            visible: true,
            cooldown: 0,
        }
    }

    pub fn update(&mut self) {
        if !self.visible && self.cooldown > 0 {
            self.cooldown -= 1;
            if self.cooldown == 0 {
                self.visible = true;
            }
        }
    }

    pub fn trigger(&mut self) -> bool {
        if self.visible {
            self.visible = false;
            self.cooldown = 60; // Reappear after 1 second (assuming 60fps)
            return true;
        }
        false
    }

    pub fn draw(&self) {
        if self.visible {
            rect!(
                x = self.pos.0 - self.size / 2.0, // Center the rectangle
                y = self.pos.1 - self.size / 2.0,
                w = self.size,
                h = self.size,
                color = self.color
            );
        }
    }
}

#[turbo::serialize]
pub struct TrailCircle {
    pub pos: (f32, f32),
    pub original_pos: (f32, f32),
    pub size: f32,
    pub color: u32,
    pub active: bool,
    pub frame_counter: u32,
}

impl TrailCircle {
    pub fn new(x: f32, y: f32, size: f32, color: u32) -> Self {
        Self {
            pos: (x, y),
            original_pos: (x, y),
            size,
            color,
            active: false,
            frame_counter: 0,
        }
    }

    pub fn update(&mut self) -> bool {
        if !self.active {
            return false;
        }

        // Move upward
        self.pos.1 -= 2.0;

        // Increment frame counter
        self.frame_counter += 1;

        // Reset if it goes off screen
        if self.pos.1 < -20.0 {
            self.reset();
            return false;
        }

        // Return true every 5 frames for trail effect
        self.frame_counter % 5 == 0
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.frame_counter = 0;
        self.pos = self.original_pos;
    }

    pub fn activate(&mut self, start_x: f32, start_y: f32) {
        self.pos = (start_x, start_y);
        self.original_pos = (start_x, start_y);
        self.active = true;
        self.frame_counter = 0;
    }

    pub fn draw(&self) {
        circ!(
            x = self.pos.0,
            y = self.pos.1,
            d = self.size,
            color = self.color
        );
    }
}

#[turbo::serialize]
pub struct Button {
    pub rect: (f32, f32, f32, f32), // (x, y, width, height)
    pub label: String,
    pub color: u32,
    pub hover_color: u32,
    pub text_color: u32,
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str) -> Self {
        Self {
            rect: (x, y, width, height),
            label: label.to_string(),
            color: 0x555555,       // Default gray
            hover_color: 0x777777, // Lighter gray for hover
            text_color: 0xFFFFFF,  // White text
        }
    }

    pub fn with_colors(mut self, color: u32, hover_color: u32, text_color: u32) -> Self {
        self.color = color;
        self.hover_color = hover_color;
        self.text_color = text_color;
        self
    }

    pub fn is_hovering(&self, mouse_pos: (f32, f32)) -> bool {
        let (x, y, width, height) = self.rect;
        mouse_pos.0 >= x
            && mouse_pos.0 <= x + width
            && mouse_pos.1 >= y
            && mouse_pos.1 <= y + height
    }

    pub fn draw(&self, mouse_pos: (f32, f32)) {
        let (x, y, width, height) = self.rect;

        // Draw button background
        let color = if self.is_hovering(mouse_pos) {
            self.hover_color
        } else {
            self.color
        };

        rect!(x = x, y = y, w = width, h = height, color = color);

        // Draw button text (centered)
        let text_width = self.label.len() as f32 * 8.0; // Approximation, adjust based on your font
        let text_x = x + (width - text_width) / 2.0;
        let text_y = y + (height - 8.0) / 2.0; // Assuming 8px text height

        text!(&self.label, x = text_x, y = text_y, color = self.text_color);
    }
}
