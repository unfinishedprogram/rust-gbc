pub const BIT_0: u8 = 0b00000001;
pub const BIT_1: u8 = 0b00000010;
pub const BIT_2: u8 = 0b00000100;
pub const BIT_3: u8 = 0b00001000;
pub const BIT_4: u8 = 0b00010000;
pub const BIT_5: u8 = 0b00100000;
pub const BIT_6: u8 = 0b01000000;
pub const BIT_7: u8 = 0b10000000;

pub fn interleave(a: u8, b: u8) -> u16 {
	let a = a as u16;
	let b = b as u16;

	let a = (a ^ (a << 4)) & 0x0f0f;
	let b = (b ^ (b << 4)) & 0x0f0f;

	let a = (a ^ (a << 2)) & 0x3333;
	let b = (b ^ (b << 2)) & 0x3333;

	let a = (a ^ (a << 1)) & 0x5555;
	let b = (b ^ (b << 1)) & 0x5555;

	(b << 1) | a
}

#[inline]
pub fn falling_edge(from: u8, to: u8, mask: u8) -> bool {
	from & mask == mask && to & mask != mask
}

pub trait Bits {
	fn set(&mut self, bits: u8, value: bool);
	fn has(&self, bits: u8) -> bool;
}

impl Bits for u8 {
	fn set(&mut self, bits: u8, value: bool) {
		if value {
			*self |= bits
		} else {
			*self &= !bits
		}
	}

	fn has(&self, bits: u8) -> bool {
		self & bits == bits
	}
}

pub trait BitLike {
	fn bits_mut(&mut self) -> &mut u8;
	fn bits(&self) -> u8;
}

impl<T> Bits for T
where
	T: BitLike,
{
	fn set(&mut self, bits: u8, value: bool) {
		self.bits_mut().set(bits, value)
	}

	fn has(&self, bits: u8) -> bool {
		self.bits().has(bits)
	}
}

macro_rules! impl_bitlike {
	($name:ident) => {
		impl BitLike for $name {
			fn bits_mut(&mut self) -> &mut u8 {
				&mut self.0
			}

			fn bits(&self) -> u8 {
				self.0
			}
		}
	};
}

pub(crate) use impl_bitlike;
