use godot::prelude::*;

mod action_indicator;
mod vec2_panel;
mod goose;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


