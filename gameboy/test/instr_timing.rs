use crate::cpu::flags::{C, Z};

use super::util::instruction_timing::expect_instr_timing;

#[test]
fn individual_instruction_timing() {
	expect_instr_timing("NOP", &[0], 1, 1, 0);
	expect_instr_timing("LD BC,u16", &[1], 1, 3, 0);
	expect_instr_timing("LD (BC),A", &[2], 1, 2, 0);
	expect_instr_timing("INC BC", &[3], 1, 2, 0);
	expect_instr_timing("INC B", &[4], 1, 1, 0);
	expect_instr_timing("DEC B", &[5], 1, 1, 0);
	expect_instr_timing("LD B,u8", &[6], 1, 2, 0);
	expect_instr_timing("RLCA", &[7], 1, 1, 0);
	expect_instr_timing("LD (u16),SP", &[8], 1, 5, 0);
	expect_instr_timing("ADD HL,BC", &[9], 1, 2, 0);
	expect_instr_timing("LD A,(BC)", &[10], 1, 2, 0);
	expect_instr_timing("DEC BC", &[11], 1, 2, 0);
	expect_instr_timing("INC C", &[12], 1, 1, 0);
	expect_instr_timing("DEC C", &[13], 1, 1, 0);
	expect_instr_timing("LD C,u8", &[14], 1, 2, 0);
	expect_instr_timing("RRCA", &[15], 1, 1, 0);
	expect_instr_timing("STOP", &[16], 1, 1, 0);
	expect_instr_timing("LD DE,u16", &[17], 1, 3, 0);
	expect_instr_timing("LD (DE),A", &[18], 1, 2, 0);
	expect_instr_timing("INC DE", &[19], 1, 2, 0);
	expect_instr_timing("INC D", &[20], 1, 1, 0);
	expect_instr_timing("DEC D", &[21], 1, 1, 0);
	expect_instr_timing("LD D,u8", &[22], 1, 2, 0);
	expect_instr_timing("RLA", &[23], 1, 1, 0);
	expect_instr_timing("JR i8", &[24], 1, 3, 0);
	expect_instr_timing("ADD HL,DE", &[25], 1, 2, 0);
	expect_instr_timing("LD A,(DE)", &[26], 1, 2, 0);
	expect_instr_timing("DEC DE", &[27], 1, 2, 0);
	expect_instr_timing("INC E", &[28], 1, 1, 0);
	expect_instr_timing("DEC E", &[29], 1, 1, 0);
	expect_instr_timing("LD E,u8", &[30], 1, 2, 0);
	expect_instr_timing("RRA", &[31], 1, 1, 0);
	expect_instr_timing("JR NZ,i8", &[32], 1, 3, 0);
	expect_instr_timing("LD HL,u16", &[33], 1, 3, 0);
	expect_instr_timing("LD (HL+),A", &[34], 1, 2, 0);
	expect_instr_timing("INC HL", &[35], 1, 2, 0);
	expect_instr_timing("INC H", &[36], 1, 1, 0);
	expect_instr_timing("DEC H", &[37], 1, 1, 0);
	expect_instr_timing("LD H,u8", &[38], 1, 2, 0);
	expect_instr_timing("DAA", &[39], 1, 1, 0);
	expect_instr_timing("JR Z,i8", &[40], 1, 2, 0);
	expect_instr_timing("JR Z,i8", &[40], 1, 3, Z);
	expect_instr_timing("ADD HL,HL", &[41], 1, 2, 0);
	expect_instr_timing("LD A,(HL+)", &[42], 1, 2, 0);
	expect_instr_timing("DEC HL", &[43], 1, 2, 0);
	expect_instr_timing("INC L", &[44], 1, 1, 0);
	expect_instr_timing("DEC L", &[45], 1, 1, 0);
	expect_instr_timing("LD L,u8", &[46], 1, 2, 0);
	expect_instr_timing("CPL", &[47], 1, 1, 0);
	expect_instr_timing("JR NC,i8", &[48], 1, 3, 0);
	expect_instr_timing("LD SP,u16", &[49], 1, 3, 0);
	expect_instr_timing("LD (HL-),A", &[50], 1, 2, 0);
	expect_instr_timing("INC SP", &[51], 1, 2, 0);
	expect_instr_timing("INC (HL)", &[52], 1, 3, 0);
	expect_instr_timing("DEC (HL)", &[53], 1, 3, 0);
	expect_instr_timing("LD (HL),u8", &[54], 1, 3, 0);
	expect_instr_timing("SCF", &[55], 1, 1, 0);
	expect_instr_timing("JR C,i8", &[56], 1, 2, 0);
	expect_instr_timing("JR C,i8", &[56], 1, 3, C);
	expect_instr_timing("ADD HL,SP", &[57], 1, 2, 0);
	expect_instr_timing("LD A,(HL-)", &[58], 1, 2, 0);
	expect_instr_timing("DEC SP", &[59], 1, 2, 0);
	expect_instr_timing("INC A", &[60], 1, 1, 0);
	expect_instr_timing("DEC A", &[61], 1, 1, 0);
	expect_instr_timing("LD A,u8", &[62], 1, 2, 0);
	expect_instr_timing("CCF", &[63], 1, 1, 0);
	expect_instr_timing("LD B,B", &[64], 1, 1, 0);
	expect_instr_timing("LD B,C", &[65], 1, 1, 0);
	expect_instr_timing("LD B,D", &[66], 1, 1, 0);
	expect_instr_timing("LD B,E", &[67], 1, 1, 0);
	expect_instr_timing("LD B,H", &[68], 1, 1, 0);
	expect_instr_timing("LD B,L", &[69], 1, 1, 0);
	expect_instr_timing("LD B,(HL)", &[70], 1, 2, 0);
	expect_instr_timing("LD B,A", &[71], 1, 1, 0);
	expect_instr_timing("LD C,B", &[72], 1, 1, 0);
	expect_instr_timing("LD C,C", &[73], 1, 1, 0);
	expect_instr_timing("LD C,D", &[74], 1, 1, 0);
	expect_instr_timing("LD C,E", &[75], 1, 1, 0);
	expect_instr_timing("LD C,H", &[76], 1, 1, 0);
	expect_instr_timing("LD C,L", &[77], 1, 1, 0);
	expect_instr_timing("LD C,(HL)", &[78], 1, 2, 0);
	expect_instr_timing("LD C,A", &[79], 1, 1, 0);
	expect_instr_timing("LD D,B", &[80], 1, 1, 0);
	expect_instr_timing("LD D,C", &[81], 1, 1, 0);
	expect_instr_timing("LD D,D", &[82], 1, 1, 0);
	expect_instr_timing("LD D,E", &[83], 1, 1, 0);
	expect_instr_timing("LD D,H", &[84], 1, 1, 0);
	expect_instr_timing("LD D,L", &[85], 1, 1, 0);
	expect_instr_timing("LD D,(HL)", &[86], 1, 2, 0);
	expect_instr_timing("LD D,A", &[87], 1, 1, 0);
	expect_instr_timing("LD E,B", &[88], 1, 1, 0);
	expect_instr_timing("LD E,C", &[89], 1, 1, 0);
	expect_instr_timing("LD E,D", &[90], 1, 1, 0);
	expect_instr_timing("LD E,E", &[91], 1, 1, 0);
	expect_instr_timing("LD E,H", &[92], 1, 1, 0);
	expect_instr_timing("LD E,L", &[93], 1, 1, 0);
	expect_instr_timing("LD E,(HL)", &[94], 1, 2, 0);
	expect_instr_timing("LD E,A", &[95], 1, 1, 0);
	expect_instr_timing("LD H,B", &[96], 1, 1, 0);
	expect_instr_timing("LD H,C", &[97], 1, 1, 0);
	expect_instr_timing("LD H,D", &[98], 1, 1, 0);
	expect_instr_timing("LD H,E", &[99], 1, 1, 0);
	expect_instr_timing("LD H,H", &[100], 1, 1, 0);
	expect_instr_timing("LD H,L", &[101], 1, 1, 0);
	expect_instr_timing("LD H,(HL)", &[102], 1, 2, 0);
	expect_instr_timing("LD H,A", &[103], 1, 1, 0);
	expect_instr_timing("LD L,B", &[104], 1, 1, 0);
	expect_instr_timing("LD L,C", &[105], 1, 1, 0);
	expect_instr_timing("LD L,D", &[106], 1, 1, 0);
	expect_instr_timing("LD L,E", &[107], 1, 1, 0);
	expect_instr_timing("LD L,H", &[108], 1, 1, 0);
	expect_instr_timing("LD L,L", &[109], 1, 1, 0);
	expect_instr_timing("LD L,(HL)", &[110], 1, 2, 0);
	expect_instr_timing("LD L,A", &[111], 1, 1, 0);
	expect_instr_timing("LD (HL),B", &[112], 1, 2, 0);
	expect_instr_timing("LD (HL),C", &[113], 1, 2, 0);
	expect_instr_timing("LD (HL),D", &[114], 1, 2, 0);
	expect_instr_timing("LD (HL),E", &[115], 1, 2, 0);
	expect_instr_timing("LD (HL),H", &[116], 1, 2, 0);
	expect_instr_timing("LD (HL),L", &[117], 1, 2, 0);
	expect_instr_timing("HALT", &[118], 1, 1, 0);
	expect_instr_timing("LD (HL),A", &[119], 1, 2, 0);
	expect_instr_timing("LD A,B", &[120], 1, 1, 0);
	expect_instr_timing("LD A,C", &[121], 1, 1, 0);
	expect_instr_timing("LD A,D", &[122], 1, 1, 0);
	expect_instr_timing("LD A,E", &[123], 1, 1, 0);
	expect_instr_timing("LD A,H", &[124], 1, 1, 0);
	expect_instr_timing("LD A,L", &[125], 1, 1, 0);
	expect_instr_timing("LD A,(HL)", &[126], 1, 2, 0);
	expect_instr_timing("LD A,A", &[127], 1, 1, 0);
	expect_instr_timing("ADD A,B", &[128], 1, 1, 0);
	expect_instr_timing("ADD A,C", &[129], 1, 1, 0);
	expect_instr_timing("ADD A,D", &[130], 1, 1, 0);
	expect_instr_timing("ADD A,E", &[131], 1, 1, 0);
	expect_instr_timing("ADD A,H", &[132], 1, 1, 0);
	expect_instr_timing("ADD A,L", &[133], 1, 1, 0);
	expect_instr_timing("ADD A,(HL)", &[134], 1, 2, 0);
	expect_instr_timing("ADD A,A", &[135], 1, 1, 0);
	expect_instr_timing("ADC A,B", &[136], 1, 1, 0);
	expect_instr_timing("ADC A,C", &[137], 1, 1, 0);
	expect_instr_timing("ADC A,D", &[138], 1, 1, 0);
	expect_instr_timing("ADC A,E", &[139], 1, 1, 0);
	expect_instr_timing("ADC A,H", &[140], 1, 1, 0);
	expect_instr_timing("ADC A,L", &[141], 1, 1, 0);
	expect_instr_timing("ADC A,(HL)", &[142], 1, 2, 0);
	expect_instr_timing("ADC A,A", &[143], 1, 1, 0);
	expect_instr_timing("SUB A,B", &[144], 1, 1, 0);
	expect_instr_timing("SUB A,C", &[145], 1, 1, 0);
	expect_instr_timing("SUB A,D", &[146], 1, 1, 0);
	expect_instr_timing("SUB A,E", &[147], 1, 1, 0);
	expect_instr_timing("SUB A,H", &[148], 1, 1, 0);
	expect_instr_timing("SUB A,L", &[149], 1, 1, 0);
	expect_instr_timing("SUB A,(HL)", &[150], 1, 2, 0);
	expect_instr_timing("SUB A,A", &[151], 1, 1, 0);
	expect_instr_timing("SBC A,B", &[152], 1, 1, 0);
	expect_instr_timing("SBC A,C", &[153], 1, 1, 0);
	expect_instr_timing("SBC A,D", &[154], 1, 1, 0);
	expect_instr_timing("SBC A,E", &[155], 1, 1, 0);
	expect_instr_timing("SBC A,H", &[156], 1, 1, 0);
	expect_instr_timing("SBC A,L", &[157], 1, 1, 0);
	expect_instr_timing("SBC A,(HL)", &[158], 1, 2, 0);
	expect_instr_timing("SBC A,A", &[159], 1, 1, 0);
	expect_instr_timing("AND A,B", &[160], 1, 1, 0);
	expect_instr_timing("AND A,C", &[161], 1, 1, 0);
	expect_instr_timing("AND A,D", &[162], 1, 1, 0);
	expect_instr_timing("AND A,E", &[163], 1, 1, 0);
	expect_instr_timing("AND A,H", &[164], 1, 1, 0);
	expect_instr_timing("AND A,L", &[165], 1, 1, 0);
	expect_instr_timing("AND A,(HL)", &[166], 1, 2, 0);
	expect_instr_timing("AND A,A", &[167], 1, 1, 0);
	expect_instr_timing("XOR A,B", &[168], 1, 1, 0);
	expect_instr_timing("XOR A,C", &[169], 1, 1, 0);
	expect_instr_timing("XOR A,D", &[170], 1, 1, 0);
	expect_instr_timing("XOR A,E", &[171], 1, 1, 0);
	expect_instr_timing("XOR A,H", &[172], 1, 1, 0);
	expect_instr_timing("XOR A,L", &[173], 1, 1, 0);
	expect_instr_timing("XOR A,(HL)", &[174], 1, 2, 0);
	expect_instr_timing("XOR A,A", &[175], 1, 1, 0);
	expect_instr_timing("OR A,B", &[176], 1, 1, 0);
	expect_instr_timing("OR A,C", &[177], 1, 1, 0);
	expect_instr_timing("OR A,D", &[178], 1, 1, 0);
	expect_instr_timing("OR A,E", &[179], 1, 1, 0);
	expect_instr_timing("OR A,H", &[180], 1, 1, 0);
	expect_instr_timing("OR A,L", &[181], 1, 1, 0);
	expect_instr_timing("OR A,(HL)", &[182], 1, 2, 0);
	expect_instr_timing("OR A,A", &[183], 1, 1, 0);
	expect_instr_timing("CP A,B", &[184], 1, 1, 0);
	expect_instr_timing("CP A,C", &[185], 1, 1, 0);
	expect_instr_timing("CP A,D", &[186], 1, 1, 0);
	expect_instr_timing("CP A,E", &[187], 1, 1, 0);
	expect_instr_timing("CP A,H", &[188], 1, 1, 0);
	expect_instr_timing("CP A,L", &[189], 1, 1, 0);
	expect_instr_timing("CP A,(HL)", &[190], 1, 2, 0);
	expect_instr_timing("CP A,A", &[191], 1, 1, 0);
	expect_instr_timing("RET NZ", &[192], 1, 5, 0);
	expect_instr_timing("POP BC", &[193], 1, 3, 0);
	expect_instr_timing("JP NZ,u16", &[194], 1, 4, 0);
	expect_instr_timing("JP u16", &[195], 1, 4, 0);
	expect_instr_timing("CALL NZ,u16", &[196], 1, 6, 0);
	expect_instr_timing("PUSH BC", &[197], 1, 4, 0);
	expect_instr_timing("ADD A,u8", &[198], 1, 2, 0);
	expect_instr_timing("RST 00h", &[199], 1, 4, 0);
	expect_instr_timing("RET Z", &[200], 1, 2, 0);
	expect_instr_timing("RET Z", &[200], 1, 5, Z);
	expect_instr_timing("RET", &[201], 1, 4, 0);
	expect_instr_timing("JP Z,u16", &[202], 1, 3, 0);
	expect_instr_timing("JP Z,u16", &[202], 1, 4, Z);
	expect_instr_timing("PREFIX CB", &[203], 1, 2, 0);
	expect_instr_timing("CALL Z,u16", &[204], 1, 6, Z);
	expect_instr_timing("CALL Z,u16", &[204], 1, 3, 0);
	expect_instr_timing("CALL u16", &[205], 1, 6, 0);
	expect_instr_timing("ADC A,u8", &[206], 1, 2, 0);
	expect_instr_timing("RST 08h", &[207], 1, 4, 0);
	expect_instr_timing("RET NC", &[208], 1, 5, 0);
	expect_instr_timing("POP DE", &[209], 1, 3, 0);
	expect_instr_timing("JP NC,u16", &[210], 1, 4, 0);
	expect_instr_timing("CALL NC,u16", &[212], 1, 6, 0);
	expect_instr_timing("PUSH DE", &[213], 1, 4, 0);
	expect_instr_timing("SUB A,u8", &[214], 1, 2, 0);
	expect_instr_timing("RST 10h", &[215], 1, 4, 0);
	expect_instr_timing("RET C", &[216], 1, 2, 0);
	expect_instr_timing("RET C", &[216], 1, 5, C);
	expect_instr_timing("RETI", &[217], 1, 4, 0);
	expect_instr_timing("JP C,u16", &[218], 1, 3, 0);
	expect_instr_timing("JP C,u16", &[218], 1, 4, C);
	expect_instr_timing("CALL C,u16", &[220], 1, 3, 0);
	expect_instr_timing("CALL C,u16", &[220], 1, 6, C);
	expect_instr_timing("SBC A,u8", &[222], 1, 2, 0);
	expect_instr_timing("RST 18h", &[223], 1, 4, 0);
	expect_instr_timing("LD (FF00+u8),A", &[224], 1, 3, 0);
	expect_instr_timing("POP HL", &[225], 1, 3, 0);
	expect_instr_timing("LD (FF00+C),A", &[226], 1, 2, 0);
	expect_instr_timing("PUSH HL", &[229], 1, 4, 0);
	expect_instr_timing("AND A,u8", &[230], 1, 2, 0);
	expect_instr_timing("RST 20h", &[231], 1, 4, 0);
	expect_instr_timing("ADD SP,i8", &[232], 1, 4, 0);
	expect_instr_timing("JP HL", &[233], 1, 1, 0);
	expect_instr_timing("LD (u16),A", &[234], 1, 4, 0);
	expect_instr_timing("XOR A,u8", &[238], 1, 2, 0);
	expect_instr_timing("RST 28h", &[239], 1, 4, 0);
	expect_instr_timing("LD A,(FF00+u8)", &[240], 1, 3, 0);
	expect_instr_timing("POP AF", &[241], 1, 3, 0);
	expect_instr_timing("LD A,(FF00+C)", &[242], 1, 2, 0);
	expect_instr_timing("DI", &[243], 1, 1, 0);
	expect_instr_timing("PUSH AF", &[245], 1, 4, 0);
	expect_instr_timing("OR A,u8", &[246], 1, 2, 0);
	expect_instr_timing("RST 30h", &[247], 1, 4, 0);
	expect_instr_timing("LD HL,SP+i8", &[248], 1, 3, 0);
	expect_instr_timing("LD SP,HL", &[249], 1, 2, 0);
	expect_instr_timing("LD A,(u16)", &[250], 1, 4, 0);
	expect_instr_timing("EI", &[251], 1, 1, 0);
	expect_instr_timing("CP A,u8", &[254], 1, 2, 0);
	expect_instr_timing("RST 38h", &[255], 1, 4, 0);
}
