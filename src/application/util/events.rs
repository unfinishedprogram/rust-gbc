use gloo::{events::EventListener, utils::document};
use web_sys::Event;

pub fn add_click_listener<F>(selector: &str, callback: F)
where
	F: FnMut(&Event) + 'static,
{
	if let Ok(Some(element)) = document().query_selector(selector) {
		EventListener::new(&element, "click", callback).forget();
	}
}
