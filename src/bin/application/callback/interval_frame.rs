use gloo::{timers::callback::Interval, utils::window};

pub struct IntervalFrame {
	_interval: Interval,
}

impl IntervalFrame {
	pub fn new(interval_ms: u32, cb: &'static (impl Fn(f64) + ?Sized)) -> Self {
		let interval = Interval::new(interval_ms, move || {
			let now = window().performance().unwrap().now();
			cb(now)
		});

		Self {
			_interval: interval,
		}
	}
}
