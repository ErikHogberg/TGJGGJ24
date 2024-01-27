use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base=Object)]
pub struct CollectionManager {
    registered: i32,
    collected: i32,
    
    base: Base<Object>,
}

#[godot_api]
impl CollectionManager {
    #[func]
    pub fn register(&mut self, _col_name: GString) {
        self.registered += 1;
    }
    #[func]
    pub fn collect(&mut self, _col_name: GString) {
        self.collected += 1;
        self.refresh_text();
    }

    #[func]
    fn get_text(&self) -> GString{
        GString::from(
        // if self.collected>= self.registered {
            format!("{}/{} rings", self.collected, self.registered)
        // }else {
        //     String::from("bruh")
        // }
    )
    }

    pub fn refresh_text(&mut self){
        let out_str = self.get_text();

        self.base_mut().emit_signal(
            "on_text".into(),
            &[Variant::from(out_str)],
        );
    }

    #[signal]
    fn on_text(text: GString);
}
