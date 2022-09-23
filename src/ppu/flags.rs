type BitFlagRef = (u16, u8);

pub enum PpuFlag {
	// VBlankInterruptEnable = (0xFFFF,),
}

pub trait PpuFlags {
	// fn get_mem(&self) -> Ref<Memory>;

	fn test_flag(&self, flag: PpuFlag) -> bool {
		// let mem = self.get_mem();
		true
	}
	fn set_flag(&mut self, flag: PpuFlag) {}
	fn reset_flag(&mut self, flag: PpuFlag) {}
}

// impl PpuFlags for Ppu {
// 	fn get_mem(&self) {
// 		return self.memory;
// 	}
// }
