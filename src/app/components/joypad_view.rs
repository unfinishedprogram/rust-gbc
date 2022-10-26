use crate::emulator::{cpu::Cpu, flags::*, state::EmulatorState};
use egui::Context;

#[derive(Debug, Clone, Copy)]
enum JoypadInput {
	Start,
	Select,
	A,
	B,
	Up,
	Down,
	Left,
	Right,
}

fn input_flag(input: JoypadInput) -> JoyPadFlag {
	use JoyPadFlag::*;
	use JoypadInput::*;
	match input {
		Start | Down => DownOrStart,
		Select | Up => UpOrSelect,
		A | Right => RightOrA,
		B | Left => LeftOrB,
	}
}

// pub fn joypad_view(ctx: &Context, cpu: &mut Cpu) {
// 	use JoypadInput::*;

// 	let buttons = [Start, Select, A, B, Up, Down, Left, Right];
// 	let mut pressed: Option<JoypadInput> = Option::None;

// 	let mut mem = cpu.memory.borrow_mut();

// 	set_bit_flag(&mut mem, BitFlag::JoyPad(JoyPadFlag::DownOrStart));
// 	set_bit_flag(&mut mem, BitFlag::JoyPad(JoyPadFlag::UpOrSelect));
// 	set_bit_flag(&mut mem, BitFlag::JoyPad(JoyPadFlag::LeftOrB));
// 	set_bit_flag(&mut mem, BitFlag::JoyPad(JoyPadFlag::RightOrA));

// 	egui::Window::new("Joypad View")
// 		.resizable(true)
// 		.show(ctx, |ui| {
// 			let dir_enable = !get_bit_flag(&mem, BitFlag::JoyPad(JoyPadFlag::SelectActionButtons));
// 			let act_enable =
// 				!get_bit_flag(&mem, BitFlag::JoyPad(JoyPadFlag::SelectDirectionButtons));
// 			ui.monospace(format!("Direction : {:?}", dir_enable));
// 			ui.monospace(format!("Actions   : {:?}", act_enable));
// 			for button in buttons {
// 				if ui.button(format!("{:?}", button)).clicked() {
// 					_ = pressed.insert(button);
// 				};
// 			}
// 		});

// 	if let Some(button) = pressed {
// 		let dir_enable = !get_bit_flag(&mem, BitFlag::JoyPad(JoyPadFlag::SelectActionButtons));
// 		let act_enable = !get_bit_flag(&mem, BitFlag::JoyPad(JoyPadFlag::SelectDirectionButtons));

// 		match (dir_enable, act_enable, button) {
// 			(_, true, Start | Select | A | B) => {
// 				clear_bit_flag(&mut mem, BitFlag::JoyPad(input_flag(button)))
// 			}
// 			(true, _, Up | Down | Left | Right) => {
// 				clear_bit_flag(&mut mem, BitFlag::JoyPad(input_flag(button)))
// 			}
// 			_ => {}
// 		}
// 	};
// }
