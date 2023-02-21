use lazy_static::lazy_static;
use sm83::instruction::Instruction;
use std::sync::{Arc, Mutex};

use crate::ppu::PPUMode;

lazy_static! {
	pub static ref DEBUGGER: Arc<Mutex<Debugger>> = Arc::new(Mutex::new(Debugger::default()));
}

pub enum Event {
	UpdatePC(u16),
	ExecInstruction(Instruction),
	PPUEnterMode(PPUMode),
}

pub enum Breakpoint {
	Addr(u16),
	PPUEnterMode(PPUMode),
	ExecInstruction(Instruction),
	PPUModeChange,
}

impl Breakpoint {
	pub fn break_on(&self, event: &Event) -> bool {
		match self {
			Breakpoint::Addr(br_addr) => matches!(event, Event::UpdatePC(addr) if addr == br_addr),
			Breakpoint::PPUEnterMode(br_mode) => {
				matches!(event, Event::PPUEnterMode(mode) if mode == br_mode)
			}
			Breakpoint::PPUModeChange => matches!(event, Event::PPUEnterMode(_)),
			_ => todo!(),
		}
	}
}

#[derive(Default)]
pub struct Debugger {
	breakpoints: Vec<Breakpoint>,
	events: Vec<Event>,
	running: bool,
}

impl Debugger {
	pub fn emit(event: Event) {
		let Ok(mut debugger) = DEBUGGER.lock() else {return};
		for breakpoint in &debugger.breakpoints {
			if breakpoint.break_on(&event) {
				debugger.running = false;
				break;
			}
		}

		debugger.events.push(event);
	}

	pub fn start() {
		let Ok(mut debugger) = DEBUGGER.lock() else {return};
		debugger.running = true;
	}

	pub fn running() -> bool {
		if let Ok(debugger) = DEBUGGER.lock() {
			debugger.running
		} else {
			false
		}
	}

	pub fn add_breakpoint(breakpoint: Breakpoint) {
		let Ok(mut debugger) = DEBUGGER.lock() else {return};
		debugger.breakpoints.push(breakpoint)
	}
}
