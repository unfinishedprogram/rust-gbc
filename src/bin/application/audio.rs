use std::collections::VecDeque;

use crate::APPLICATION;
use js_sys::Function;
use tracing_wasm::set_as_global_default;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::AudioProcessingEvent;

pub fn audio_cb(evt: AudioProcessingEvent) {
	APPLICATION.with_borrow_mut(|app| {
		if let Some(audio) = app.audio.as_mut() {
			let mut buffer = evt.output_buffer().unwrap();

			let mut left = vec![0.0; buffer.length() as usize];
			let mut right = vec![0.0; buffer.length() as usize];

			for i in 0..buffer.length() as usize {
				if let Some((l, r)) = audio.pop_sample() {
					left[i] = l;
					right[i] = r;
				} else {
					log::error!("Buffer ran dry");
					break;
				}
			}

			buffer.copy_to_channel(&left, 0);
			buffer.copy_to_channel(&right, 1);
		};
	})
}

pub struct AudioHandler {
	ctx: web_sys::AudioContext,
	source_node: web_sys::ConstantSourceNode,
	script_node: web_sys::ScriptProcessorNode,

	min_buffer_size: usize,
	audio_buffer: VecDeque<(f32, f32)>,
	running: bool,
	cb: js_sys::Function,
}

// TODO: improve this
// The only way I could find fo pass a rust function as if it was a js_sys::Function
pub fn audio_callback_as_js_func() -> js_sys::Function {
	let closure =
		Closure::<dyn FnMut(AudioProcessingEvent)>::wrap(Box::new(|evt: AudioProcessingEvent| {
			audio_cb(evt);
		})
			as Box<dyn FnMut(AudioProcessingEvent)>);
	let function: js_sys::Function = closure.into_js_value().unchecked_into();
	function
}

impl AudioHandler {
	pub fn new(sample_rate: f32, min_buffer_size: usize) -> Result<Self, JsValue> {
		let ctx =
			web_sys::AudioContext::new_with_context_options(&web_sys::AudioContextOptions::new())?;

		log::error!("Sample rate: {}", ctx.sample_rate());
		let cb = audio_callback_as_js_func();

		// The script node pulls samples from the audio buffer as they are needed
		// TODO: In the future, when threaded WASM has better support:
		// 		consider using an audio worklet instead since this is deprecated
		let script_node = ctx.create_script_processor()?;
		script_node.set_onaudioprocess(Some(&cb));
		script_node.connect_with_audio_node(&ctx.destination())?;

		// Source node only exists to drive the script node
		let source_node = ctx.create_constant_source()?;

		source_node.set_channel_count(2);
		source_node.connect_with_audio_node(&script_node)?;

		Ok(Self {
			running: true,
			audio_buffer: VecDeque::new(),
			min_buffer_size,
			ctx,
			source_node,
			script_node,
			cb,
		})
	}

	pub fn pull_samples(&mut self, gb_audio: &mut gameboy::audio::Audio, delta_ms: f64) {
		let delta_seconds = delta_ms / 1000.0;
		let samples = (self.sample_rate() as f64 * delta_seconds) as usize;
		self.audio_buffer.extend(gb_audio.pull_samples(samples));
	}

	fn pop_sample(&mut self) -> Option<(f32, f32)> {
		self.audio_buffer.pop_front()
	}

	pub fn play(&mut self) {
		self.source_node.start().unwrap();
		self.running = true;
	}

	pub fn stop(&mut self) {
		self.source_node.stop().unwrap();
		self.running = false;
	}

	pub fn running(&self) -> bool {
		self.running
	}

	pub fn remaining_samples(&self) -> usize {
		self.audio_buffer.len()
	}

	pub fn sample_rate(&self) -> f32 {
		self.ctx.sample_rate()
	}
}