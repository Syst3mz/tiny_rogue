#![cfg_attr(not(test), no_std)]

mod ascii_renderer;
mod button_input;

extern crate alloc;

#[macro_use]
extern crate playdate as pd;

use alloc::format;
use alloc::string::{String, ToString};
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
use simple_vector2::Vector2;
use game_logic::{Game, LOGGER};
use game_logic::renderer::Renderer;
use shared::constants::{FONT_METRICS, SCREEN_SIZE_IN_PIXELS};
use crate::ascii_renderer::AsciiRenderer;
use crate::button_input::ButtonInput;

enum GameState {
	MainMenu,
	Gameplay(Game<AsciiRenderer, ButtonInput>),
	GameOver(u32, String),
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

	fn centered_paragraph(lines: &[impl AsRef<str>]) {
		let longest_line_in_pixels = lines
			.iter()
			.map(|line| line.as_ref().len())
			.max()
			.unwrap();
		let longest_line_in_pixels = longest_line_in_pixels * FONT_METRICS.x;
		let paragraph_size = Vector2::new(longest_line_in_pixels, lines.len() * FONT_METRICS.y);
		let paragraph_offset_in_pixels = paragraph_size / 2;
		let paragraph_offset_in_pixels = (SCREEN_SIZE_IN_PIXELS / 2) - paragraph_offset_in_pixels;
		
		

		for (line_index, line) in lines.iter().enumerate() {
			let line = line.as_ref();
			let line_in_pixels = line.len() * FONT_METRICS.x;
			let line_indent = (longest_line_in_pixels - line_in_pixels) / 2;
			let _ = draw_text(
				line,
				(paragraph_offset_in_pixels.x + line_indent) as i32,
				(paragraph_offset_in_pixels.y + line_index * FONT_METRICS.y) as i32
			);
		}
	}
	
	fn update(&mut self) {
		match self {
			GameState::MainMenu => {
				Self::centered_paragraph(&[
					"Tiny Rogue",
					"Press A to start",
				]);
				
				self.restart_on_a();
			}
			GameState::Gameplay(game) => {
				game.update();
				if game.is_game_over() {
					*self = GameState::GameOver(game.score(), game.player_killer().unwrap_or("The Planet".to_string()));
					return;
				}
				while let Some(message) = LOGGER.next_message() {
					println!("{}", message);
				}

				//System::Default().draw_fps(0, 0);

				game.renderer.render(&mut ());
				
			},
			GameState::GameOver(s, k) => {
				Self::centered_paragraph(&[
					"Game Over!".to_string(),
					format!("You scored: {}", s),
					format!("Killed by: {}.", k),
					"Press A to restart".to_string(),
				]);

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
