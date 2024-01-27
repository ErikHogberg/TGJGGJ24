use godot::engine::{Control, IControl};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
struct Vec2Panel {
    #[export]
    dir1: Vector2,
    #[export]
    dir2: Vector2,
    #[export]
    dir_mul: f32,
    #[export]
    bg: Color,
    #[export]
    line1: Color,
    #[export]
    line2: Color,

    base: Base<Control>,
}

#[godot_api]
impl Vec2Panel {
    #[func]
    pub fn set_debug_dir(&mut self, dir: Vector2) {
        self.dir1 = dir;
        self.base_mut().queue_redraw();
    }
    #[func]
    pub fn set_debug_facing(&mut self, dir: Vector2) {
        self.dir2 = dir;
        self.base_mut().queue_redraw();
    }
}

#[godot_api]
impl IControl for Vec2Panel {
    fn init(base: Base<Control>) -> Self {
        Self {
            dir1: Vector2::RIGHT,
            dir2: Vector2::UP,
            dir_mul: 1f32,
            bg: Color::from_rgb(0.1f32, 0.1f32, 0.2f32),
            line1: Color::from_rgb(0.8f32, 0.5f32, 0.2f32),
            line2: Color::from_rgb(0.2f32, 0.5f32, 0.8f32),
            base,
        }
    }

    fn draw(&mut self) {
        let bg = self.bg;
        let line1 = self.line1;
        let line2 = self.line2;
        let dir1 = self.dir1;
        let dir2 = self.dir2;
        let base_dims = self.base().get_size();
        let center = Vector2::new(0f32, 0f32) + base_dims / 2f32;
        let dir_mul = self.dir_mul;
        let mut ctrl = self.base_mut();

        ctrl.draw_rect(
            Rect2::from_components(0.0f32, 0.0f32, base_dims.x, base_dims.y),
            bg,
        );
        ctrl.draw_line(center, center + dir1 * dir_mul, line1);
        ctrl.draw_line(center, center + dir2 * dir_mul, line2);
        
    }
}
