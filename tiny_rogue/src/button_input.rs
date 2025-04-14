use pd::controls::api::Default;
use pd::controls::buttons::PDButtonsExt;
use pd::controls::peripherals::Buttons;
use game_logic::input::{Button, Input};

pub struct ButtonInput {
    buttons: Buttons<Default>
}

impl core::default::Default for ButtonInput {
    fn default() -> Self {
        Self {
            buttons: Buttons::Default(),
        }
    }
}
impl Input for ButtonInput {

    fn button_down(&mut self) -> Option<Button> {
        let pushed = self.buttons.pushed();
        // this could be a fallthrough case but I want to exit early for perf reasons.
        /*if pushed.is_empty() {
            pushed = self.buttons.current();
        }*/

        println!("Pressed: {:?}", pushed);

        if pushed.is_empty() {
            return None;
        }

        if pushed.up() {
            return Some(Button::Up);
        }

        if pushed.right() {
            return Some(Button::Right);
        }

        if pushed.down() {
            return Some(Button::Down);
        }

        if pushed.left() {
            return Some(Button::Left);
        }

        if pushed.a() {
            return Some(Button::A);
        }

        if pushed.b() {
            return Some(Button::B);
        }

        None
    }
}