use super::{
	decode_tables::DT,
	opcode::{parse_opcode, Opcode},
	CPURegister16::*,
	CPURegister8::*,
	Condition, Instruction,
	Instruction::*,
	ValueRefU8,
};

use crate::{
	arg, inst, mem, memory_mapper::SourcedMemoryMapper, registers::CPURegister16,
	values::ValueRefU16, SM83,
}; // Macros

// Three Chunks
// 00..40
// 40..C0

pub fn fetch<T: SourcedMemoryMapper>(cpu: &mut impl SM83<T>) -> Instruction {
	let opcode = cpu.next_byte();

	match opcode {
		0x00 => NOP,
		0x10 => STOP,
		// 0x20 => inst!(cpu, JR, NZ,),
		_ => NOP,
		// 0x00 => NOP
		// ld bc, $50C3
		// ld [bc], a
		// inc bc
		// inc b
		// dec b
		// ld b, $C3
		// rlca
		// ld [$50C3], sp
		// add hl, bc
		// ld a, [bc]
		// dec bc
		// inc c
		// dec c
		// ld c, $C3
		// rrca
		// stop
		// ld de, $50C3
		// ld [de], a
		// inc de
		// inc d
		// dec d
		// ld d, $C3
		// rla
		// jr $00C5
		// add hl, de
		// ld a, [de]
		// dec de
		// inc e
		// dec e
		// ld e, $C3
		// rra
		// jr nz, $00C5
		// ld hl, $50C3
		// ld [hl+], a
		// inc hl
		// inc h
		// dec h
		// ld h, $C3
		// daa
		// jr z, $00C5
		// jr z, $00C5
		// add hl, hl
		// ld a, [hl+]
		// dec hl
		// inc l
		// dec l
		// ld l, $C3
		// cpl
		// jr nc, $00C5
		// ld sp, $50C3
		// ld [hl-], a
		// inc sp
		// inc [hl]
		// dec [hl]
		// ld [hl], $C3
		// scf
		// jr c, $00C5
		// jr c, $00C5
		// add hl, sp
		// ld a, [hl-]
		// dec sp
		// inc a
		// dec a
		// ld a, $C3
		// ccf
		// ld b, b
		// ld b, c
		// ld b, d
		// ld b, e
		// ld b, h
		// ld b, l
		// ld b, [hl]
		// ld b, a
		// ld c, b
		// ld c, c
		// ld c, d
		// ld c, e
		// ld c, h
		// ld c, l
		// ld c, [hl]
		// ld c, a
		// ld d, b
		// ld d, c
		// ld d, d
		// ld d, e
		// ld d, h
		// ld d, l
		// ld d, [hl]
		// ld d, a
		// ld e, b
		// ld e, c
		// ld e, d
		// ld e, e
		// ld e, h
		// ld e, l
		// ld e, [hl]
		// ld e, a
		// ld h, b
		// ld h, c
		// ld h, d
		// ld h, e
		// ld h, h
		// ld h, l
		// ld h, [hl]
		// ld h, a
		// ld l, b
		// ld l, c
		// ld l, d
		// ld l, e
		// ld l, h
		// ld l, l
		// ld l, [hl]
		// ld l, a
		// ld [hl], b
		// ld [hl], c
		// ld [hl], d
		// ld [hl], e
		// ld [hl], h
		// ld [hl], l
		// halt
		// ld [hl], a
		// ld a, b
		// ld a, c
		// ld a, d
		// ld a, e
		// ld a, h
		// ld a, l
		// ld a, [hl]
		// ld a, a
		// add a, b
		// add a, c
		// add a, d
		// add a, e
		// add a, h
		// add a, l
		// add a, [hl]
		// add a, a
		// adc a, b
		// adc a, c
		// adc a, d
		// adc a, e
		// adc a, h
		// adc a, l
		// adc a, [hl]
		// adc a, a
		// sub a, b
		// sub a, c
		// sub a, d
		// sub a, e
		// sub a, h
		// sub a, l
		// sub a, [hl]
		// sub a, a
		// sbc a, b
		// sbc a, c
		// sbc a, d
		// sbc a, e
		// sbc a, h
		// sbc a, l
		// sbc a, [hl]
		// sbc a, a
		// and a, b
		// and a, c
		// and a, d
		// and a, e
		// and a, h
		// and a, l
		// and a, [hl]
		// and a, a
		// xor a, b
		// xor a, c
		// xor a, d
		// xor a, e
		// xor a, h
		// xor a, l
		// xor a, [hl]
		// xor a, a
		// or a, b
		// or a, c
		// or a, d
		// or a, e
		// or a, h
		// or a, l
		// or a, [hl]
		// or a, a
		// cp a, b
		// cp a, c
		// cp a, d
		// cp a, e
		// cp a, h
		// cp a, l
		// cp a, [hl]
		// cp a, a
		// ret nz
		// pop bc
		// jp nz, $50C3
		// jp $50C3
		// call nz, $50C3
		// push bc
		// add a, $C3
		// rst $00
		// ret z
		// ret z
		// ret
		// jp z, $50C3
		// jp z, $50C3
		// set 0, e
		// call z, $50C3
		// call z, $50C3
		// call $50C3
		// adc a, $C3
		// rst $08
		// ret nc
		// pop de
		// jp nc, $50C3
		// call nc, $50C3
		// push de
		// sub a, $C3
		// rst $10
		// ret c
		// ret c
		// reti
		// jp c, $50C3
		// jp c, $50C3
		// call c, $50C3
		// call c, $50C3
		// sbc a, $C3
		// rst $18
		// ldh [$FFC3], a
		// pop hl
		// ldh [c], a
		// push hl
		// and a, $C3
		// rst $20
		// add sp, -$3D
		// jp hl
		// ld [$50C3], a
		// xor a, $C3
		// rst $28
		// ldh a, [$FFC3]
		// pop af
		// ldh a, [c]
		// di
		// push af
		// or a, $C3
		// rst $30
		// ld hl, sp + -$3D
		// ld sp, hl
		// ld a, [$50C3]
		// ei
		// cp a, $C3
		// rst $38
	}
}
