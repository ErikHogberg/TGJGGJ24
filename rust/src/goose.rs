use godot::engine::utilities::move_toward;
use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Goose {
    #[export]
    y: f32,
    #[export]
    ground_y: f32,
    #[export]
    velocity: Vector2,
    #[export]
    damping: Vector2,
    #[export]
    shoot_vel: Vector2,
    #[export]
    velocity_cap: Vector2,

    paused: bool,
    should_restart: bool,
    stamina: i32,

    hit_ground_once: bool,

    base: Base<Node2D>,
}

#[godot_api]
impl Goose {
    #[func]
    fn pause(&mut self) {
        self.paused = true;
    }

    #[func]
    fn hit_stop_guy(&mut self) {
        godot_print!("hit stop guy");
        self.stamina = 0;
        self.should_restart = true;
        self.base_mut()
            .emit_signal("x_vel_update".into(), &[Variant::from(0.0f32)]);
    }
    #[func]
    fn hit_plane(&mut self) {
        self.velocity.x = 0.0;
    }
    #[func]
    fn hit_ufo(&mut self) {
        godot_print!("hit ufo")
    }

    #[func]
    fn hit_hawk(&mut self) {
        godot_print!("hit hawk")
    }

    #[signal]
    fn x_vel_update(new_x_vel: f32);
    #[signal]
    fn on_shoot();
    #[signal]
    fn on_hit_ground();
    #[signal]
    fn on_consume_one_stamina_and_flap(new_stamina_amount: i32);
    #[signal]
    fn on_cannot_flap();
    #[signal]
    fn on_fill_stamina_and_bounce();
    #[signal]
    fn on_stopped();
    #[signal]
    fn on_sit();
    #[signal]
    fn on_restart();
}

impl Goose {
    fn move_toward_f32(from: f32, to: f32, delta: f32) -> f32 {
        move_toward(from as f64, to as f64, delta as f64) as f32
    }
}

#[godot_api]
impl INode2D for Goose {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            y: 0.0,
            ground_y: 50.0,
            velocity: Vector2::new(0.0, 0.0),
            damping: Vector2::new(1.0, 1.0),
            shoot_vel: Vector2::new(99.0, 9.0),
            velocity_cap: Vector2::ZERO,
            hit_ground_once: false,
            paused: true,
            should_restart: false,
            stamina: 12,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();

        if self.should_restart {
            if input.is_action_just_pressed("jump".into()) {
                godot_print!("restarting");
                self.base_mut().emit_signal("on_restart".into(), &[]);
                self.should_restart = false;
                self.paused = true;
            }
            return;
        }

        if input.is_action_just_pressed("jump".into()) {
            if self.paused {
                self.velocity.y = -self.shoot_vel.y;
                self.velocity.x += self.shoot_vel.x;
                self.paused = false;
                self.stamina = 12;
                self.base_mut()
                    .emit_signal("on_fill_stamina_and_bounce".into(), &[]);
            } else {
                if self.stamina > 0 {
                    self.stamina -= 1;
                    self.velocity.y = -self.shoot_vel.y;
                    self.velocity.x += self.shoot_vel.x;
                    let stamina = self.stamina;
                    self.hit_ground_once = false;
                    self.base_mut().emit_signal(
                        "on_consume_one_stamina_and_flap".into(),
                        &[Variant::from(stamina)],
                    );
                } else {
                    self.base_mut().emit_signal("on_cannot_flap".into(), &[]);
                }
            }
        }
        if self.paused {
            self.base_mut()
                .emit_signal("x_vel_update".into(), &[Variant::from(0.0f32)]);
            return;
        }

        let delta_time = delta as f32;
        let global_position = self.base().get_global_position();

        self.velocity.y += self.damping.y * delta_time;

        if self.velocity.y < -self.velocity_cap.y {
            self.velocity.y = -self.velocity_cap.y;
        }
        if self.velocity.y > self.velocity_cap.y {
            self.velocity.y = self.velocity_cap.y;
        }

        let up_speed = self.velocity.y * delta_time;
        let mut new_pos = global_position + Vector2::DOWN * up_speed;
        if new_pos.y > self.ground_y {
            if !self.hit_ground_once {
                self.hit_ground_once = true;
                self.base_mut().emit_signal("on_hit_ground".into(), &[]);
            }
            self.velocity.y = 0.0;
            new_pos.y = self.ground_y;
        }

        self.base_mut().set_global_position(new_pos);

        if self.velocity.x < -self.velocity_cap.x {
            self.velocity.x = -self.velocity_cap.x;
        }
        if self.velocity.x > self.velocity_cap.x {
            self.velocity.x = self.velocity_cap.x;
        }

        self.velocity.x = Self::move_toward_f32(self.velocity.x, 0.0, self.damping.x * delta_time);

        let x_vel = self.velocity.x;
        self.base_mut()
            .emit_signal("x_vel_update".into(), &[Variant::from(x_vel)]);

        if x_vel < 0.001 {
            self.base_mut().emit_signal("on_sit".into(), &[]);
            if self.stamina < 1 {
                self.should_restart = true;
                self.base_mut()
                    .emit_signal("x_vel_update".into(), &[Variant::from(0.0f32)]);
                self.paused = true;
                self.base_mut().emit_signal("on_stopped".into(), &[]);
            }
        }
    }
}
