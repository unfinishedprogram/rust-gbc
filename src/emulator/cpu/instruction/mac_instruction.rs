#[macro_export]
macro_rules! arg {
	($cpu:tt, d) => {
		$cpu.next_displacement().into()
	};
	($cpu:tt, n) => {
		$cpu.next_byte().into()
	};
	($cpu:tt, nn) => {
		$cpu.next_chomp().into()
	};

	($cpu:tt, $p:tt) => {
		$p.into()
	};
}

#[macro_export]
macro_rules! mem {
	($cpu:tt, [$p1:tt]u8) => {{
		ValueRefU8::Mem(arg!($cpu, $p1))
	}};

	($cpu:tt, [$p1:tt]u16) => {{
		ValueRefU16::Mem(arg!($cpu, $p1);)
	}};
}

#[macro_export]
macro_rules! inst {
	($cpu:ident, $inst:ident, $p1:tt, $p2:tt) => {
		$inst(arg!($cpu, $p1), arg!($cpu, $p2))
	};

	($cpu:ident, $inst:ident, $p1:tt, $p2:tt, $p3:tt) => {
		$inst(arg!($cpu, $p1), arg!($cpu, $p2), arg!($cpu, $p3))
	};

	($cpu:ident, $inst:ident, [$p1:tt]$t1:tt, [$p2:tt]$t2:ty) => {
		$inst(mem!($cpu, [$p1]$t1), mem!($cpu, [$p2]$t2))
	};

	($cpu:ident, $inst:ident, [$p1:tt]$t:tt, $p2:tt) => {
		$inst(mem!($cpu, [$p1]$t), arg!($cpu, $p2))
	};

	($cpu:ident, $inst:ident, $p1:tt, [$p2:tt]$t:tt) => {
		$inst(arg!($cpu, $p1), mem!($cpu, [$p2]$t))
	};

	($cpu:ident, $inst:ident, $p1:tt) => {
		$inst(arg!($cpu, $p1))
	};

	($cpu:ident, $inst:ident, [$p1:tt]$t:ty) => {
		$inst(mem!($cpu, [$p1]$t))
	};

	($cpu:tt, $inst:tt) => { $inst };
}
