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

    hit_ground_once: bool,

    base: Base<Node2D>,
}

#[godot_api]
impl Goose {
    #[signal]
    fn x_vel_update(new_x_vel: f32);
    #[signal]
    fn on_shoot();
    #[signal]
    fn on_hit_ground();
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
            hit_ground_once: false,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let delta_time = delta as f32;
        let input = Input::singleton();

        let global_position = self.base().get_global_position();

        if input.is_action_just_pressed("jump".into()) {
            self.velocity.y = -self.shoot_vel.y;
            self.velocity.x += self.shoot_vel.x;
        }

        self.velocity.y += self.damping.y * delta_time;
        let up_speed = self.velocity.y * delta_time;
        let mut new_pos = global_position + Vector2::DOWN * up_speed;
        if new_pos.y > self.ground_y {
            if !self.hit_ground_once{
                self.hit_ground_once = true;
                self.base_mut().emit_signal("on_hit_ground".into(), &[]);
                
            }
            self.velocity.y = 0.0;
            new_pos.y = self.ground_y;
        }

        self.base_mut()
            .set_global_position(new_pos);

        self.velocity.x = Self::move_toward_f32(self.velocity.x, 0.0, self.damping.x * delta_time);

        let x_vel = self.velocity.x;
        self.base_mut().emit_signal("x_vel_update".into(), &[Variant::from(x_vel)]);
    }
}
