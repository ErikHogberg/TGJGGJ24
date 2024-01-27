use godot::{prelude::*, engine::Engine};

mod action_indicator;
mod vec2_panel;
mod collection_manager;
mod collectable;
mod goose;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // The StringName identifies your singleton and can be
            // used later to access it.
            Engine::singleton().register_singleton(
                StringName::from("colman"),
                collection_manager::CollectionManager::new_alloc().upcast(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Unregistering is needed to avoid memory leaks and 
            // warnings, especially for hot reloading.
            Engine::singleton().unregister_singleton(
                StringName::from("colman")
            );
        }
    }
}


