decodeFile = open("insts.csv")

addr_map = { 
    "imp": "Implicit",
    "acc": "Accumulator",
    "imm": "Immediate",
    "zp": "ZeroPage",
    "zpx": "ZeroPageX",
    "zpy": "ZeroPageY",
    "rel": "Relative",
    "abs": "Absolute",
    "abx": "AbsoluteX",
    "aby": "AbsoluteY",
    "ind": "Indirect",
    "inx": "IndirectX",
    "iny": "IndirectY"
}

wb_map = {
    "A": "Accumulator",
    "X": "X",
    "Y": "Y",
    "M": "Memory",
    "PC": "PC",
    "SP": "SP",
    "PS": "PS",
    "NW": "NoWriteback"
}

opcodes = {}

for entry in decodeFile:
    (opcode, instr, addr, wb, byteLen, time) = entry.strip().split(",")
    if opcode == "hex": continue
    line = "/* {} */    Instruction::new(".format(opcode)
    if instr != "":
        opcodes[instr] = 1
        line += "Opcode::{}, AddressingMode::{}, Writeback::{}, {}, {}".format(instr, addr_map[addr], wb_map[wb], byteLen, time)
    else:
        line += "Opcode::Invalid, AddressingMode::None, Writeback::NoWriteback, 0, 0"
    line += "),"

    print(line)

print()

for opcode in sorted(opcodes):
    print("{},".format(opcode))