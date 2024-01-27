use std::f32::consts::PI;

use godot::engine::{Curve, INode3D, Node3D};
use godot::prelude::*;

use crate::tank::Tank;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Cam {
    #[export]
    follow: Gd<Tank>,
    #[export]
    distance_min_max: Vector2,
    #[export]
    speed_min_max: Vector2,
    #[export]
    speed_curve: Gd<Curve>,
    #[export]
    vertical_mul: f32,

    #[export]
    enable_turning: bool,
    #[export]
    angle_min_max: Vector2,
    #[export]
    turn_min_max: Vector2,
    #[export]
    turn_curve: Gd<Curve>,

    base: Base<Node3D>,
}

#[godot_api]
impl Cam {
    #[signal]
    fn rot_debug(text: String);
    #[signal]
    fn dir_debug(dir: Vector2);
    #[signal]
    fn facing_debug(facing: Vector2);
}

impl Cam {
    fn lerp_v(current: f32, min_max: Vector2) -> f32 {
        Self::lerp_i(current, min_max.x, min_max.y)
    }
    fn lerp_i(current: f32, min: f32, max: f32) -> f32 {
        ((current - min) / (max - min)).clamp(0f32, 1f32)
    }

    fn lerp(
        current: f32,
        min_max_distance: Vector2,
        min_max_speed: Vector2,
        curve: &Gd<Curve>,
    ) -> f32 {
        let lerp = Self::lerp_v(current, min_max_distance);
        min_max_speed
            .x
            .lerp(min_max_speed.y, curve.sample_baked(lerp))
    }
}

#[godot_api]
impl INode3D for Cam {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            follow: Tank::new_alloc(),
            speed_curve: Curve::new_gd(),
            turn_curve: Curve::new_gd(),
            enable_turning: true,
            vertical_mul: 1f32,
            distance_min_max: Vector2::new(1f32, 9f32),
            speed_min_max: Vector2::new(1f32, 9f32),
            angle_min_max: Vector2::new(1f32, 9f32),
            turn_min_max: Vector2::new(1f32, 9f32),
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let delta_time = delta as f32;
        let global_position = self.base().get_global_position();
        let follow_global_position = self.follow.get_global_position();
        let distance = global_position.distance_to(follow_global_position);
        let follow_speed = Self::lerp(
            distance,
            self.distance_min_max,
            self.speed_min_max,
            &self.speed_curve,
        );
        let mut asdf = global_position
            .move_toward(follow_global_position, follow_speed * delta_time)
            - global_position;
        asdf.y *= self.vertical_mul;
        self.base_mut().set_global_position(global_position + asdf);

        if self.enable_turning {
            let dir = self.follow.bind().forward_dir();
            let z_sign = dir.z.sign();
            let cam_dir =
                Tank::rotate_vector_by_quaternion(self.base().get_quaternion(), Vector3::FORWARD);
            let dir_angle = dir.signed_angle_to(Vector3::RIGHT, Vector3::UP)*z_sign;
            //dir.x.atan2(dir.y);
            let current_angle = cam_dir.signed_angle_to(Vector3::RIGHT, Vector3::UP);
            //cam_dir.x.atan2(cam_dir.y);
            // self.base().get_global_rotation().y;

            let angle_delta =
                utilities::angle_difference(current_angle as f64, dir_angle as f64) as f32;
            let turn_speed = Self::lerp(
                angle_delta.abs(),
                self.angle_min_max,
                self.turn_min_max,
                &self.turn_curve,
            );
            let turn_speed_eval = turn_speed * delta_time * angle_delta.sign(); //* z_sign;
            let close_enough = turn_speed_eval.abs() > angle_delta.abs();
            self.base_mut().rotate_y(
                if close_enough {
                angle_delta 
                } else {
                    turn_speed_eval
                }
            );

            self.base_mut().emit_signal(
                "dir_debug".into(),
                // &[Variant::from(Vector2::new(dir.x, dir.z))],
                &[Variant::from(Vector2::DOWN.rotated(dir_angle))],
            );

            // let quat = self.base().get_quaternion();
            // let facing = Tank::rotate_vector_by_quaternion(quat, Vector3::FORWARD);
            self.base_mut().emit_signal(
                "facing_debug".into(),
                // &[Variant::from(Vector2::new(cam_dir.x, cam_dir.z))],
                &[Variant::from(Vector2::DOWN.rotated(current_angle))],
            );

            self.base_mut().emit_signal(
                "rot_debug".into(),
                &[Variant::from(
                    format!(
                        "d{:.1} t{:.1}\nc{:.1} \tf{:.1}\n{}",
                        angle_delta / PI,
                        turn_speed_eval / PI,
                        current_angle / PI,
                        dir_angle / PI,
                        close_enough
                    )
                    .to_string(),
                )],
            );
        }
    }
}
