use lazy_static::lazy_static;
use log::Log;
use sm83::instruction::Instruction;
use std::{
	fmt::Debug,
	sync::{Arc, Mutex},
};

use crate::{cgb::Speed, ppu::PPUMode};

lazy_static! {
	pub static ref DEBUGGER: Arc<Mutex<DebuggerState>> =
		Arc::new(Mutex::new(DebuggerState::default()));
}

#[derive(Debug, Clone)]
pub enum Event {
	UpdatePC(u16),
	ExecInstruction(Instruction),
	PPUEnterMode(PPUMode),
	WriteMem(u16, u8),
	ReadMem(u16),
	Error(String),
	Warn(String),
	Info(String),
	Debug(String),
	Trace(String),
	SpeedSwitch(Speed),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Breakpoint {
	Addr(u16),
	PPUEnterMode(PPUMode),
	ExecInstruction(Instruction),
	PPUModeChange,
	WriteMem(u16),
	ReadMem(u16),
	SpeedSwitch(Speed),
}

impl Breakpoint {
	pub fn break_on(&self, event: &Event) -> bool {
		match self {
			Breakpoint::Addr(br_addr) => matches!(event, Event::UpdatePC(addr) if addr == br_addr),
			Breakpoint::PPUEnterMode(br_mode) => {
				matches!(event, Event::PPUEnterMode(mode) if mode == br_mode)
			}
			Breakpoint::PPUModeChange => matches!(event, Event::PPUEnterMode(_)),
			Breakpoint::ReadMem(br_addr) => {
				matches!(event, Event::ReadMem(addr) if addr == br_addr)
			}
			Breakpoint::WriteMem(br_addr) => {
				matches!(event, Event::WriteMem(addr, _) if addr == br_addr)
			}

			Breakpoint::SpeedSwitch(_) => matches!(event, Event::SpeedSwitch(_)),
			_ => todo!(),
		}
	}
}

pub struct Debugger;

impl Log for Debugger {
	fn enabled(&self, _metadata: &log::Metadata) -> bool {
		cfg!(feature = "debug")
	}

	fn log(&self, _record: &log::Record) {
		#[cfg(feature = "debug")]
		{
			let args = _record.args();
			let args = format!("{args}");

			let event = match _record.metadata().level() {
				log::Level::Error => Event::Error(args),
				log::Level::Warn => Event::Warn(args),
				log::Level::Info => Event::Info(args),
				log::Level::Debug => Event::Debug(args),
				log::Level::Trace => Event::Trace(args),
			};
			emit(event)
		}
	}

	fn flush(&self) {
		let Ok(mut debugger) = DEBUGGER.lock() else {return};
		debugger.events.clear();
	}
}

#[derive(Default)]
pub struct DebuggerState {
	pub breakpoints: Vec<Breakpoint>,
	pub events: Vec<Event>,
	pub running: bool,
}

impl DebuggerState {
	pub fn emit(&mut self, event: Event) {
		if cfg!(feature = "debug") {
			for breakpoint in &self.breakpoints {
				if breakpoint.break_on(&event) {
					self.running = false;
					self.events.clear();
					break;
				}
			}
			self.events.push(event);
		}
	}
	pub fn add_breakpoint(&mut self, breakpoint: Breakpoint) {
		self.breakpoints.push(breakpoint);
	}

	pub fn remove_breakpoint(&mut self, breakpoint: Breakpoint) {
		self.breakpoints.retain(|br| br != &breakpoint);
	}
}

pub fn emit(event: Event) {
	if cfg!(feature = "debug") {
		let Ok(mut debugger) = DEBUGGER.lock() else {return};
		debugger.emit(event);
	}
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

pub fn log_count() -> usize {
	let Ok(debugger) = DEBUGGER.lock() else {return 0};
	debugger.events.len()
}

pub fn get_range(start: usize, end: usize) -> Vec<Event> {
	let Ok(debugger) = DEBUGGER.lock() else {return vec![]};
	if start > 0 && end < debugger.events.len() {
		debugger.events[start..end].to_vec()
	} else {
		vec![]
	}
}
