use godot::classes::CharacterBody3D;
use godot::engine::{AnimationPlayer, Area3D, ICharacterBody3D, KinematicCollision3D, Timer};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::mob::MobCharacterBody;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct PlayerCharacterBody {
    #[export]
    speed: i64,
    #[export]
    fall_acceleration: i64,
    #[export]
    jump_impulse: i32,
    #[export]
    bounce_impulse: i32,

    target_velocity: Vector3,
    animation: Option<Gd<AnimationPlayer>>,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for PlayerCharacterBody {
    // Is called when it is first constructed, even in the editor
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: 14,
            fall_acceleration: 75,
            jump_impulse: 20,
            bounce_impulse: 16,
            target_velocity: Vector3::ZERO,
            animation: None,
            base,
        }
    }

    // Called once when it and all its child nodes are ready.
    fn ready(&mut self) {
        self.animation = Some(
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer"),
        );
        self.setup();
    }

    fn physics_process(&mut self, delta: f64) {
        let mut direction: Vector3 = Vector3::ZERO;

        if Input::singleton().is_action_pressed(StringName::from("move_right")) {
            direction.x += 1.0 as f32;
        }
        if Input::singleton().is_action_pressed(StringName::from("move_left")) {
            direction.x -= 1.0 as f32;
        }
        if Input::singleton().is_action_pressed(StringName::from("move_back")) {
            direction.z += 1.0 as f32;
        }
        if Input::singleton().is_action_pressed(StringName::from("move_forward")) {
            direction.z -= 1.0 as f32;
        }

        if direction != Vector3::ZERO {
            direction = direction.normalized();
            let look: Basis = Basis::new_looking_at(direction, Vector3::UP, false);
            self.base_mut()
                .get_node_as::<Node3D>("Pivot")
                .set_basis(look);
            self.animation
                .as_mut()
                .expect("AnimationPlayer not found")
                .set_speed_scale(4.0);
        } else {
            self.animation
                .as_mut()
                .expect("AnimationPlayer not found")
                .set_speed_scale(1.0);
        }

        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;

        if !self.base().is_on_floor() {
            self.target_velocity.y -= (self.fall_acceleration as f64 * delta) as f32;
        }

        if self.base().is_on_floor()
            && Input::singleton().is_action_pressed(StringName::from("jump"))
        {
            self.target_velocity.y = self.jump_impulse as f32;
        }

        let num_of_collisions = self.base().get_slide_collision_count();
        for i in 0..num_of_collisions {
            let collision = self
                .base_mut()
                .get_slide_collision(i)
                .expect("Not a collider");
            let collider = collision.get_collider();
            self.mob_collision(collider, collision);
        }

        let final_velocity: Vector3 = self.target_velocity;
        self.base_mut().set_velocity(final_velocity);

        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl PlayerCharacterBody {
    // Can't create signal connections for custom functions from the base implementation, so need to do it here.
    #[func]
    fn setup(&mut self) {
        let mut mob_detector = self.base().get_node_as::<Area3D>("MobDetector");
        mob_detector.connect(
            StringName::from("body_entered"),
            self.base_mut().callable("die"),
        );
        self.base_mut().add_user_signal(GString::from("hit"));

        let die_callable = self.base_mut().callable("game_over");
        self.base_mut()
            .connect(StringName::from("hit"), die_callable);
    }

    #[func]
    fn mob_collision(&mut self, collider: Option<Gd<Object>>, collision: Gd<KinematicCollision3D>) {
        match collider {
            Some(collider) => {
                let mob = collider.try_cast::<MobCharacterBody>();
                match mob {
                    Ok(mut mob) => {
                        if Vector3::UP.dot(collision.get_normal()) > 0.1 {
                            mob.bind_mut().squash();
                            self.target_velocity.y = self.bounce_impulse as f32;
                        }
                    }
                    Err(mut _mob) => {}
                }
            }
            None => {}
        }
    }

    #[allow(dead_code)]
    // This is called from signal body_entered, so called dynamically
    #[func]
    fn die(&mut self, _body: Gd<Node3D>) {
        godot_print!("I'm hit!");
        self.base_mut().emit_signal(StringName::from("hit"), &[]);
        self.base_mut().queue_free();
    }

    #[func]
    fn game_over(&mut self) {
        // Find the mob time node
        let main = self.base_mut().find_parent("Main".into()).expect("No Main");
        let mut mob_timer = main.get_node_as::<Timer>("MobTimer");
        // stop it.
        mob_timer.stop();
    }
}
