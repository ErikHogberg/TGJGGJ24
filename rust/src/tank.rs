use godot::engine::{Curve, Generic6DofJoint3D, IGeneric6DofJoint3D, VehicleBody3D};
use godot::obj::NewAlloc;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Generic6DofJoint3D)]
pub(crate) struct Tank {
    #[export]
    left_tread: Gd<VehicleBody3D>,
    #[export]
    right_tread: Gd<VehicleBody3D>,

    #[export]
    speed_min_max: Vector2,
    #[export]
    speed_lerp_curve: Gd<Curve>,
    #[export]
    rev_mul: f32,
    #[export]
    sprint_mul: f32,
    #[export]
    weight_point: f32,
    #[export]
    weight_point_offset: Vector3,

    start_pos_l: Vector3,
    start_pos_r: Vector3,

    speed_lerp: f32,
    speed_cache: f32,

    #[base]
    base: Base<Generic6DofJoint3D>,
}

#[godot_api]
impl Tank {
    #[func]
    fn set_speed_lerp(&mut self, lerp: f32) {
        self.speed_lerp = lerp * 0.01f32;
        let min = self.speed_min_max.x;
        let max = self.speed_min_max.y;
        self.speed_cache = min.lerp(max, self.speed_lerp_curve.sample_baked(self.speed_lerp));
        let signal_speed = self.speed_cache;
        self.base_mut().emit_signal(
            "speed_eval".into(),
            &[Variant::from(format!("{:.0}", signal_speed).to_string())],
        );
    }

    #[func]
    fn set_weight_lerp(&mut self, lerp: f32) {
        // self.weight_point = lerp * 0.01f32;
        let center_z = Vector3::FORWARD*(self.weight_point * -lerp * 0.01f32) + self.weight_point_offset;
        self.left_tread.set_center_of_mass(center_z);
        self.right_tread.set_center_of_mass(center_z);
    }

    #[func]
    fn reset_pos(&mut self) {
        self.left_tread.set_global_position(self.start_pos_l);
        self.right_tread.set_global_position(self.start_pos_r);
        self.left_tread.set_linear_velocity(Vector3::ZERO);
        self.right_tread.set_linear_velocity(Vector3::ZERO);
        self.left_tread.set_rotation(Vector3::ZERO);
        self.right_tread.set_rotation(Vector3::ZERO);
    }

    pub fn rotate_vector_by_quaternion(quaternion: Quaternion, vector: Vector3) -> Vector3 {
        // Extract the vector part of the quaternion
        let q_vector = Vector3::new(quaternion.x, quaternion.y, quaternion.z);

        // Extract the scalar part of the quaternion
        let q_scalar = quaternion.w;

        // Do the math
        q_vector.dot(vector) * q_vector * 2.0f32
            + (q_scalar * q_scalar - q_vector.dot(q_vector)) * vector
            + q_scalar * q_vector.cross(vector) * 2.0f32
    }

    #[func]
    pub fn forward_dir(&self) -> Vector3 {
        let avg_vel = self.left_tread.get_linear_velocity();
        // + self.right_tread.get_linear_velocity()) * 0.5f32;
        // avg_vel.y = 0f32;
        // avg_vel.z *= -1f32;

        if avg_vel.length() < 1f32 {
            Self::rotate_vector_by_quaternion(self.base().get_quaternion(), Vector3::FORWARD)
        } else {
            avg_vel.normalized()
        }
    }

    #[signal]
    fn l_gas(lgas: f32);
    #[signal]
    fn r_gas(rgas: f32);
    #[signal]
    fn speed_eval(speedEval: String);
}

#[godot_api]
impl IGeneric6DofJoint3D for Tank {
    fn init(base: Base<Generic6DofJoint3D>) -> Self {
        Self {
            right_tread: VehicleBody3D::new_alloc(),
            left_tread: VehicleBody3D::new_alloc(),
            speed_min_max: Vector2::new(9f32, 9999f32),
            speed_lerp_curve: Curve::new_gd(),
            rev_mul: 1f32,
            sprint_mul: 2f32,
            start_pos_l: Vector3::ZERO,
            start_pos_r: Vector3::ZERO,
            speed_lerp: 1f32,
            speed_cache: 1f32,
            weight_point: 0f32,
            weight_point_offset: Vector3::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        self.start_pos_l = self.left_tread.get_global_position();
        self.start_pos_r = self.right_tread.get_global_position();
        self.set_speed_lerp(50f32);
    }

    fn physics_process(&mut self, _delta: f64) {
        let input = Input::singleton();
        if input.is_action_just_pressed(StringName::from("reset player")) {
            self.reset_pos();
        }

        let gas = input.get_action_strength(StringName::from("both gas"));
        let lgas = input.get_action_strength(StringName::from("left gas"));
        let rgas = input.get_action_strength(StringName::from("right gas"));
        let rev = input.get_action_strength(StringName::from("both reverse"));
        let lrev = input.get_action_strength(StringName::from("left reverse"));
        let rrev = input.get_action_strength(StringName::from("right reverse"));
        let sprint = input.is_action_pressed(StringName::from("sprint"));

        let left_gas_eval = gas + lgas - (rev + lrev);
        let right_gas_eval = gas + rgas - (rev + rrev);

        let mut left_thread_engine_force = left_gas_eval;
        let mut right_thread_engine_force = right_gas_eval;
        if sprint {
            left_thread_engine_force *= self.sprint_mul;
            right_thread_engine_force *= self.sprint_mul;
        }

        self.base_mut().emit_signal(
            "l_gas".into(),
            &[Variant::from(left_thread_engine_force * 25f32)],
        );
        self.base_mut().emit_signal(
            "r_gas".into(),
            &[Variant::from(right_thread_engine_force * 25f32)],
        );

        // EmitSignal(SignalName.LGas, leftThreadEngineForce * 25);
        // EmitSignal(SignalName.RGas, rightThreadEngineForce * 25);

        left_thread_engine_force *= self.speed_cache;
        right_thread_engine_force *= self.speed_cache;

        if left_thread_engine_force < 0f32 {
            left_thread_engine_force *= self.rev_mul;
        }
        if right_thread_engine_force < 0f32 {
            right_thread_engine_force *= self.rev_mul;
        }

        self.left_tread.set_engine_force(left_thread_engine_force);
        self.right_tread.set_engine_force(right_thread_engine_force);
    }
}
