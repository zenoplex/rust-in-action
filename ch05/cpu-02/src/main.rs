struct CPU {
    registers: [u8; 16],
    position_in_memory: usize, // program counter
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16; // settings to u16 so left shift works
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        // Set carry flag
        match overflow_detected {
            true => self.registers[0xF] = 1,
            _ => self.registers[0xF] = 0,
        }
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8; // extracting nibbles
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                // terminate function
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    // 0x8014 opcode add registers 0 and 1
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;
    // 0x8024 opcode add registers 0 and 2
    cpu.memory[2] = 0x80;
    cpu.memory[3] = 0x24;
    // 0x8024 opcode add registers 0 and 3
    cpu.memory[4] = 0x80;
    cpu.memory[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    // Registers[0] is added
    println!("5 + + 10 + 10 + 10 = {}", cpu.registers[0]);
}
