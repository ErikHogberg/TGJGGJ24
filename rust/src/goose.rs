
use godot::engine::utilities::move_toward;
use godot::engine::{Curve, INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Goose {
    #[export]
    y: f32,
    #[export]
    velocity: Vector2,
    #[export]
    damping: Vector2,
    #[export]
    shoot_vel: Vector2,

    #[export]
    distance_min_max: Vector2,
    #[export]
    speed_min_max: Vector2,
    #[export]
    speed_curve: Gd<Curve>,
    #[export]
    vertical_mul: f32,

    base: Base<Node2D>,
}

#[godot_api]
impl Goose {
    #[signal]
    fn x_vel_update(new_x_vel: f32);
}

// impl Goose {
//     fn lerp_v(current: f32, min_max: Vector2) -> f32 {
//         Self::lerp_i(current, min_max.x, min_max.y)
//     }
//     fn lerp_i(current: f32, min: f32, max: f32) -> f32 {
//         ((current - min) / (max - min)).clamp(0f32, 1f32)
//     }

//     fn lerp(
//         current: f32,
//         min_max_distance: Vector2,
//         min_max_speed: Vector2,
//         curve: &Gd<Curve>,
//     ) -> f32 {
//         let lerp = Self::lerp_v(current, min_max_distance);
//         min_max_speed
//             .x
//             .lerp(min_max_speed.y, curve.sample_baked(lerp))
//     }
// }

#[godot_api]
impl INode2D for Goose {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            y: 0.0,
            velocity: Vector2::new(0.0, 0.0),
            damping: Vector2::new(1.0, 1.0),
            shoot_vel: Vector2::new(99.0, 9.0),
            speed_curve: Curve::new_gd(),
            vertical_mul: 1.0,
            distance_min_max: Vector2::new(1.0, 9.0),
            speed_min_max: Vector2::new(1.0, 9.0),
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let delta_time = delta as f32;
        let input = Input::singleton();

        // let global_position = self.base().get_global_position();

        if input.is_action_just_pressed("jump".into()) {
            self.velocity.y += self.shoot_vel.y;
            self.velocity.x += self.shoot_vel.x;
        }

        self.velocity.x = move_toward(self.velocity.x as f64, 0.0, delta) as f32;

        let x_vel = self.velocity.x;
        self.base_mut().emit_signal(
            "x_vel_update".into(),
            &[Variant::from(x_vel)],
        );
    }
}
