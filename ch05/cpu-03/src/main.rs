struct CPU {
    registers: [u8; 16],
    position_in_memory: usize, // program counter
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
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

        println!("register[0]: {}", val);

        // Set carry flag
        match overflow_detected {
            true => self.registers[0xF] = 1,
            _ => self.registers[0xF] = 0,
        }
    }

    fn call(&mut self, addr: u16) {
        let pointer = self.stack_pointer;
        let stack = &mut self.stack;

        if pointer > stack.len() {
            panic!("Stack overflow");
        }

        println!("call function at memory address: {:0x}", addr);

        stack[pointer] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;

        println!(
            "current memory position in call: {}",
            self.position_in_memory
        );
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;

        println!(
            "current memory position after ret: {}",
            self.position_in_memory
        );
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8; // extracting nibbles
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;

            println!("opcode: {:0x}", opcode);
            println!("memory postiion: {}", self.position_in_memory);

            match (c, x, y, d) {
                // terminate function
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
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
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // 0x2100 Call the function at 0x100
    // 2100 -> 8014 -> 8014 -> 00EE
    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    // 0x2100 Call the function at 0x100
    // 2100 -> 8014 -> 8014 -> 00EE
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;

    // 0x0814 Add register[1] to register[0]
    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    // 0x0814 Add register[1] to register[0]
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    // 0x00EE Return
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;
    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    // Also can be broken down "(5 + 10) + (15 + 10) + (25 + 10) + (35 + 10)"
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
