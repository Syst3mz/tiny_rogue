
#![cfg_attr(not(test), no_std)]

mod ascii_renderer;

extern crate alloc;

#[macro_use]
extern crate playdate as pd;

use alloc::string::String;
use core::ffi::*;
use core::ptr::NonNull;
use libm::{sin, sinf};
use pd::controls::peripherals::Crank;
use pd::sys::EventLoopCtrl;
use pd::sys::ffi::{playdate_graphics, PlaydateAPI};
use pd::system::update::UpdateCtrl;
use pd::display::Display;
use pd::ext::PlaydateAPIExt;
use pd::fs::api::Api;
use pd::graphics::*;
use pd::graphics::text::*;
use pd::graphics::bitmap::*;
use pd::system::prelude::*;
use pd::sound::prelude::*;
use pd::fs::Path;
use game_logic::Game;
use game_logic::renderer::Renderer;
use crate::ascii_renderer::AsciiRenderer;

/// Game state
struct State {
	game: Game<AsciiRenderer>,
}


impl State {
	fn new() -> Self {

		// TODO: Init the state
		// Causes crashes on startup if it can't find MonoCarlo
		let font = load_font("assets/fonts/MonoCarlo")
			.expect("Could not load font: \"MonoCarlo\"");
		set_font(&font);

		Self {
			game: Game::new(AsciiRenderer::default()),
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

		self.game.renderer.render(&mut ());

		// TODO: update the state of game

		// System::Default().draw_fps(0, 0);

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
		let mut state = State::new();
		state.game.init();
		unsafe { STATE = Some(state) }

	}

	// Call state.event
	unsafe { STATE.as_mut().expect("impossible") }.event(event)
}


// Needed for debug build, absolutely optional
ll_symbols!();
