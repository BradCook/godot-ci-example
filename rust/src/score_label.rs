use godot::classes::Label;
use godot::engine::ILabel;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct ScoreLabel {
    score: i32,

    base: Base<Label>,
}

#[godot_api]
impl ILabel for ScoreLabel {
    fn init(base: Base<Label>) -> Self {
        Self { score: 0, base }
    }
}

#[godot_api]
impl ScoreLabel {
    #[func]
    pub fn update_score(&mut self) {
        self.score += 1;
        let text = GString::from(format!("Score: {}", self.score));
        self.base_mut().set_text(text);
    }
}
