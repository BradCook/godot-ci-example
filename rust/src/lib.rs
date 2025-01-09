use godot::prelude::*;

mod main_scene;
mod mob;
mod player_character;
mod score_label;

struct SquashCreepsExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SquashCreepsExtension {}
