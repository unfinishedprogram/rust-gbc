pub trait U16Value<'a> {
	fn get(&self) -> u16;
	fn set(&mut self, value:u16);
}