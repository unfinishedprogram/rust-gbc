pub trait Channel {
	fn write_nrx0(&mut self, value: u8);
	fn read_nrx0(&self) -> u8;

	fn write_nrx1(&mut self, value: u8);
	fn read_nrx1(&self) -> u8;

	fn write_nrx2(&mut self, value: u8);
	fn read_nrx2(&self) -> u8;

	fn write_nrx3(&mut self, value: u8);
	fn read_nrx3(&self) -> u8;

	fn write_nrx4(&mut self, value: u8);
	fn read_nrx4(&self) -> u8;

	fn tick(&mut self);

	fn tick_sweep(&mut self);
	fn tick_length_ctr(&mut self);
	fn tick_vol_env(&mut self);

	fn sample(&self) -> u8;
	fn enabled(&self) -> bool;

	fn reset(&mut self);
}
