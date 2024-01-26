use godot::engine::{Area3D, IArea3D};
use godot::prelude::*;

use crate::collection_manager::CollectionManager;

#[derive(GodotClass)]
#[class(base=Area3D)]
struct Collectable {
    #[export]
    collection: GString,
    #[base]
    base: Base<Area3D>,
}

#[godot_api]
impl Collectable {
    #[func]
    pub fn yeet(&mut self) {
        let colman = godot::engine::Engine::singleton().get_singleton(StringName::from("colman"));
        if let Some(x) = colman {
            x.cast::<CollectionManager>()
                .bind_mut()
                .collect(self.collection.clone());
        }
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IArea3D for Collectable {
    fn init(base: Base<Area3D>) -> Self {
        Self {
            collection: "rings".into(),
            base,
        }
    }

    fn enter_tree(&mut self) {
        let callable = self.base().callable("yeet");
        self.base_mut().connect("body_entered".into(), callable);

        let colman = godot::engine::Engine::singleton().get_singleton(StringName::from("colman"));
        if let Some(x) = colman {
            x.cast::<CollectionManager>()
                .bind_mut()
                .register(self.collection.clone());
        }
    }
}
