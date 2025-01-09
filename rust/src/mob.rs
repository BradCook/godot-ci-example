use godot::classes::CharacterBody3D;
use godot::classes::RandomNumberGenerator;
use godot::classes::VisibleOnScreenNotifier3D;
use godot::engine::AnimationPlayer;
use godot::engine::ICharacterBody3D;
use godot::prelude::*;
use real_consts::PI;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct MobCharacterBody {
    #[export]
    min_speed: i32,
    #[export]
    max_speed: i32,

    is_squashed: bool,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for MobCharacterBody {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            min_speed: 10,
            max_speed: 18,
            is_squashed: false,
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl MobCharacterBody {
    #[func]
    pub fn initialise(&mut self, start_position: Vector3, player_position: Vector3) {
        self.base_mut().add_user_signal(GString::from("squashed"));

        let mut visible_on_screen = self
            .base_mut()
            .get_node_as::<VisibleOnScreenNotifier3D>("VisibleNotifier");
        visible_on_screen.connect(
            StringName::from("screen_exited"),
            self.base_mut().callable("on_screen_exit"),
        );

        self.base_mut()
            .look_at_from_position(start_position, player_position);
        let mut random_generator = RandomNumberGenerator::new_gd();
        random_generator.randomize();
        self.base_mut()
            .rotate_y(random_generator.randf_range(-PI / 4.0, PI / 4.0));

        random_generator.randomize();
        let random_speed = random_generator.randi_range(self.min_speed, self.max_speed);
        let velocity = Vector3::FORWARD * random_speed as f32;
        let y_rotation = self.base().get_rotation().y;

        let mut animation = self
            .base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation.set_speed_scale(random_speed as f32 / self.min_speed as f32);

        self.base_mut()
            .set_velocity(velocity.rotated(Vector3::UP, y_rotation));
    }

    #[func]
    pub fn on_screen_exit(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    pub fn squash(&mut self) {
        // This is needed because the collision gets called in multiple frames, and therefore triggers
        // multiple times.
        if !self.is_squashed {
            self.base_mut()
                .emit_signal(StringName::from("squashed"), &[]);
        }
        self.is_squashed = true;
        self.base_mut().queue_free();
    }
}
