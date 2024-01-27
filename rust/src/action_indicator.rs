use godot::engine::{ColorRect, IColorRect, InputEvent};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=ColorRect)]
struct ActionIndicator {
    #[export]
    action_name: GString,
    #[export]
    press_color: Color,
    #[export]
    release_color: Color,

    base: Base<ColorRect>,
}

#[godot_api]
impl IColorRect for ActionIndicator {
    fn init(base: Base<ColorRect>) -> Self {
        Self {
            action_name: GString::from("both gas"),
            press_color: Color {
                r: 0.0f32,
                g: 1f32,
                b: 0f32,
                a: 1f32,
            },
            release_color: Color {
                r: 1f32,
                g: 1f32,
                b: 1f32,
                a: 1f32,
            },
            base,
        }
    }

    fn ready(&mut self) {
        let color = self.release_color;
        self.base_mut().set_color(color);
    }

    fn input(&mut self, input_event: Gd<InputEvent>) {
        let action_name = StringName::from(&self.action_name);

        if input_event.is_action_pressed(action_name.clone())
		{
            let color = self.press_color;
            self.base_mut().set_color(color);
		}
        if input_event.is_action_released(action_name.clone())
		{
            let color = self.release_color;
            self.base_mut().set_color(color);
		}
    }
}
