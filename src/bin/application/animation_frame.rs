use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{closure::Closure, JsCast};

pub struct AnimationFrame {
	handle: Rc<RefCell<i32>>,
}

impl Drop for AnimationFrame {
	fn drop(&mut self) {
		gloo::utils::window()
			.cancel_animation_frame(*self.handle.borrow())
			.unwrap();
	}
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) -> i32 {
	gloo::utils::window()
		.request_animation_frame(f.as_ref().unchecked_ref())
		.unwrap()
}

impl AnimationFrame {
	pub fn new(cb: &'static (impl Fn(f64) + ?Sized)) -> Self {
		let f = Rc::new(RefCell::new(None));
		let g = f.clone();

		let handle = Rc::new(RefCell::new(0));
		let h = handle.clone();

		*g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
			*h.borrow_mut() = request_animation_frame(f.borrow().as_ref().unwrap());
			cb(time);
		}) as Box<dyn FnMut(f64)>));

		*handle.borrow_mut() = request_animation_frame(g.borrow().as_ref().unwrap());

		Self { handle }
	}
}
