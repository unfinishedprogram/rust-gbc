pub trait PpuFlags {
	fn get_mem() -> Ref<Memory>;
}

// Bit 6 - LYC=LY Coincidence Interrupt (1=Enable)
// Bit 5 - Mode 2 OAM Interrupt         (1=Enable)
// Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable)
// Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable)
// Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY)
// Bit 1-0 - Mode Flag       (Mode 0-3, see below)
