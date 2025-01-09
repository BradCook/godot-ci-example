use crate::score_label::ScoreLabel;

use super::mob::MobCharacterBody;
use super::player_character::PlayerCharacterBody;
use godot::classes::PackedScene;
use godot::classes::Path3D;
use godot::classes::PathFollow3D;
use godot::classes::RandomNumberGenerator;
use godot::classes::Timer;
use godot::engine::ColorRect;
use godot::engine::InputEvent;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct MainScene {
    #[export]
    mob_scene: Gd<PackedScene>,
    #[export]
    score_label: Option<Gd<ScoreLabel>>,

    retry_screen: Option<Gd<ColorRect>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for MainScene {
    fn init(base: Base<Node>) -> Self {
        Self {
            mob_scene: PackedScene::new_gd(),
            score_label: None,
            retry_screen: None,
            base,
        }
    }

    fn ready(&mut self) {
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");
        mob_timer.connect(
            StringName::from("timeout"),
            self.base_mut().callable("on_timeout"),
        );

        self.retry_screen = Some(self.base().get_node_as::<ColorRect>("UI/Retry"));
        self.retry_screen
            .as_mut()
            .expect("Retry screen not found")
            .hide();

        let mut player = self.base().get_node_as::<PlayerCharacterBody>("Player");
        player.connect(
            StringName::from("hit"),
            self.base_mut().callable("on_player_hit"),
        );
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed(StringName::from("ui_accept"))
            && self
                .retry_screen
                .as_ref()
                .expect("Retry Screen not found")
                .is_visible()
        {
            self.base_mut()
                .get_tree()
                .expect("No Tree")
                .reload_current_scene();
        }
    }
}

#[godot_api]
impl MainScene {
    #[func]
    fn on_timeout(&mut self) {
        let mut random_generator = RandomNumberGenerator::new_gd();
        random_generator.randomize();
        let mut mob = self.mob_scene.instantiate_as::<MobCharacterBody>();

        let mut mob_spawn_location = self
            .base_mut()
            .get_node_as::<Path3D>("SpawnLocation")
            .get_node_as::<PathFollow3D>("SpawnPath");
        mob_spawn_location.set_progress_ratio(random_generator.randf());

        let player_position = self
            .base()
            .get_node_as::<PlayerCharacterBody>("Player")
            .get_position();
        mob.bind_mut()
            .initialise(mob_spawn_location.get_position(), player_position);

        let update_score = self
            .score_label
            .as_mut()
            .expect("Score Label not set")
            .callable("update_score");
        mob.connect(StringName::from("squashed"), update_score);

        self.base_mut().add_child(mob.upcast::<Node>());
    }

    #[func]
    fn on_player_hit(&mut self) {
        self.retry_screen
            .as_mut()
            .expect("Retry Screen Not set")
            .show();
    }
}
