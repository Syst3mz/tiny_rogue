use core::time::Duration;
use pd::controls::api::Default;
use pd::controls::buttons::PDButtonsExt;
use pd::controls::peripherals::Buttons;
use pd::system::System;
use game_logic::input::{Button, Input};

pub struct ButtonInput {
    system: System,
    buttons: Buttons<Default>,
    last_press: Duration
}

impl core::default::Default for ButtonInput {
    fn default() -> Self {
        let system = System::Default();
        let now = system.current_time();
        Self {
            system,
            buttons: Buttons::Default(),
            last_press: now
        }
    }
}
impl Input for ButtonInput {

    fn button_down(&mut self) -> Option<Button> {
        let now = self.system.current_time();
        if now - self.last_press <= Duration::from_millis(125) { 
            return None;
        }
        
        let mut pushed = self.buttons.pushed();
        
        // this could be a fallthrough case but I want to exit early for perf reasons.
        if pushed.is_empty() {
            pushed = self.buttons.current()
        }

        if pushed.is_empty() {
            return None;
        }
        
        let mut ret = None;

        if pushed.up() {
            ret = Some(Button::Up);
        }

        if pushed.right() {
            ret = Some(Button::Right);
        }

        if pushed.down() {
            ret = Some(Button::Down);
        }

        if pushed.left() {
            ret = Some(Button::Left);
        }

        if pushed.a() {
            ret = Some(Button::A);
        }

        if pushed.b() {
            ret = Some(Button::B);
        }
        
        self.last_press = now;
        ret
    }
}