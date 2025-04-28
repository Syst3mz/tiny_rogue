#![cfg_attr(not(test), no_std)]

mod ascii_renderer;
mod button_input;

extern crate alloc;

#[macro_use]
extern crate playdate as pd;

use alloc::format;
use core::ptr::NonNull;
use pd::controls::buttons::PDButtonsExt;
use pd::controls::peripherals::Buttons;
use pd::sys::EventLoopCtrl;
use pd::sys::ffi::PlaydateAPI;
use pd::system::update::UpdateCtrl;
use pd::display::Display;
use pd::graphics::*;
use pd::graphics::text::*;
use pd::graphics::bitmap::*;
use pd::system::prelude::*;
use game_logic::{Game, LOGGER};
use game_logic::renderer::Renderer;
use shared::constants::FONT_METRICS;
use crate::ascii_renderer::AsciiRenderer;
use crate::button_input::ButtonInput;

enum GameState {
	MainMenu,
	Gameplay(Game<AsciiRenderer, ButtonInput>),
	GameOver(u32),
}

impl GameState {
	fn initialize_game() -> Self {
		let seed = System::Default().current_time().as_micros().clamp(0, u64::MAX as u128) as u64;
		let mut game = Game::new(AsciiRenderer::default(), ButtonInput::default(), seed);
		game.make_level();

		GameState::Gameplay(game)
	}
	
	fn restart_on_a(&mut self) {
		if Buttons::Default().get().pushed.a() {
			*self = GameState::initialize_game();
		}
	}
	
	fn update(&mut self) {
		match self {
			GameState::MainMenu => {
				let _ = draw_text("Tiny Rogue", 0, 0);
				let _ = draw_text("Press A to start", 0, FONT_METRICS.y as i32);
				
				self.restart_on_a();
			}
			GameState::Gameplay(game) => {
				game.update();
				if game.is_game_over() {
					*self = GameState::GameOver(game.score());
					return;
				}
				while let Some(message) = LOGGER.next_message() {
					println!("{}", message);
				}

				//System::Default().draw_fps(0, 0);

				game.renderer.render(&mut ());
				
			},
			GameState::GameOver(s) => {
				let line = FONT_METRICS.y as i32;
				let _ = draw_text("Game Over!", 0, 0);
				let _ = draw_text(format!("You scored: {}", s), 0, line);
				let _ = draw_text("Press A to restart", 0, line * 2);

				self.restart_on_a();
			}
		}	
	}
}

/// Game state
struct State {
	state: GameState,
}


impl State {
	fn new() -> Self {

		// TODO: Init the state
		// Causes crashes on startup if it can't find MonoCarlo
		let font = load_font("assets/fonts/MonoCarlo")
			.expect("Could not load font: \"MonoCarlo\"");
		set_font(&font);

		Self {
			state: GameState::MainMenu
		}
	}


	/// System event handler
	fn event(&'static mut self, event: SystemEvent) -> EventLoopCtrl {
		match event {
			// Initial setup
			SystemEvent::Init => {
				// Set FPS to 30
				Display::Default().set_refresh_rate(30.0);

				// Register our update handler that defined below
				self.set_update_handler();

				println!("Game init complete");
			},
			// TODO: React to other events
			_ => {},
		}
		EventLoopCtrl::Continue
	}
}


impl Update for State {
	/// Updates the state
	fn update(&mut self) -> UpdateCtrl {
		clear(Color::WHITE);
		// TODO: update the state of game
		
		self.state.update();
		
		UpdateCtrl::Continue
	}
}


/// Entry point
#[unsafe(no_mangle)]
#[allow(static_mut_refs)]
pub fn event_handler(_api: NonNull<PlaydateAPI>, event: SystemEvent, _sim_key_code: u32) -> EventLoopCtrl {
	// Unsafe static storage for our state.
	// Usually it's safe because there's only one thread.
	pub static mut STATE: Option<State> = None;
	if unsafe { STATE.is_none() } {
		let state = State::new();
		unsafe { STATE = Some(state) }

	}

	// Call state.event
	unsafe { STATE.as_mut().expect("impossible") }.event(event)
}

// Needed for debug build, absolutely optional
ll_symbols!();
