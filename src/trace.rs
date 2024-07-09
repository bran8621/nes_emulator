use crate::cpu::{AddressingMode, CPU, Mem};
use crate::opcodes;
use std::collections::HashMap;
use crate::bus::Bus;
use crate::cartridge::test::test_rom;
use crate::ppu::NesPPU;


pub fn trace(cpu: &mut CPU) -> String {
    // Fetch the opcode map from the module
    let opcodes_map: &HashMap<u8, &'static opcodes::OpCode> = &*opcodes::OPCODES_MAP;

    // Read the opcode byte from memory and fetch its corresponding OpCode struct
    let code = cpu.mem_read(cpu.program_counter);
    let ops = opcodes_map.get(&code).expect("Invalid opcode");

    // Start building the hex dump with the opcode byte
    let begin = cpu.program_counter;
    let mut hex_dump = vec![code];

    // Variables to store memory address and value based on addressing mode
    let (mem_addr, stored_value) = match ops.mode {
        AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
        _ => {
            let (addr, _) = cpu.get_absolute_address(&ops.mode, begin + 1);
            (addr, cpu.mem_read(addr))
        }
    };

    // String to store the operand representation
    let tmp = match ops.len {
        1 => match ops.code {
            0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
            _ => String::new(),
        },
        2 => {
            let address: u8 = cpu.mem_read(begin + 1);
            hex_dump.push(address);

            match ops.mode {
                AddressingMode::Immediate => format!("#${:02x}", address),
                AddressingMode::ZeroPage | AddressingMode::ZeroPage_X | AddressingMode::ZeroPage_Y => {
                    format!("${:02x},{:?} @ {:02x} = {:02x}", address, ops.mode, mem_addr, stored_value)
                },
                AddressingMode::Indirect_X => {
                    let address = address.wrapping_add(cpu.register_x);
                    format!("(${:02x},X) @ {:02x} = {:04x} = {:02x}", address, mem_addr, stored_value)
                },
                AddressingMode::Indirect_Y => {
                    let address = mem_addr.wrapping_sub(cpu.register_y as u16);
                    format!("(${:02x}),Y = {:04x} @ {:04x} = {:02x}", address, mem_addr, stored_value)
                },
                AddressingMode::NoneAddressing => {
                    let address: usize = (begin as usize + 2).wrapping_add((address as i8) as usize);
                    format!("${:04x}", address)
                },
                _ => panic!(
                    "Unexpected addressing mode {:?} with ops-len 2. code {:02x}",
                    ops.mode, ops.code
                ),
            }
        },
        3 => {
            let address_lo = cpu.mem_read(begin + 1);
            let address_hi = cpu.mem_read(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address = cpu.mem_read_u16(begin + 1);

            match ops.mode {
                AddressingMode::NoneAddressing => {
                    if ops.code == 0x6c {
                        let jmp_addr = if address & 0x00FF == 0x00FF {
                            let lo = cpu.mem_read(address);
                            let hi = cpu.mem_read(address & 0xFF00);
                            (hi as u16) << 8 | (lo as u16)
                        } else {
                            cpu.mem_read_u16(address)
                        };

                        format!("(${:04x}) = {:04x}", address, jmp_addr)
                    } else {
                        format!("${:04x}", address)
                    }
                },
                AddressingMode::Absolute | AddressingMode::Absolute_X | AddressingMode::Absolute_Y => {
                    format!("${:04x},{:?} @ {:04x} = {:02x}", address, ops.mode, mem_addr, stored_value)
                },
                _ => panic!(
                    "Unexpected addressing mode {:?} with ops-len 3. code {:02x}",
                    ops.mode, ops.code
                ),
            }
        },
        _ => String::new(), // Handle unsupported opcode lengths
    };

    // Convert hex dump to a formatted string
    let hex_str = hex_dump.iter().map(|z| format!("{:02x}", z)).collect::<Vec<String>>().join(" ");

    // Build the final assembly string
    let asm_str = format!("{:04x}  {:8} {: >4} {}", begin, hex_str, ops.mnemonic, tmp).trim().to_ascii_uppercase();

    // Format and return the trace output string
    format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        asm_str, cpu.register_a, cpu.register_x, cpu.register_y, cpu.status, cpu.stack_pointer,
    )
}

#[test]
fn test_format_opcode_with_immediate_addressing_mode() {
    let mut bus = Bus::new(test_rom(), |ppu: &NesPPU| {});
    bus.mem_write(100, 0xa9);
    bus.mem_write(101, 0x42);

    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x64;
    cpu.register_a = 0x00;
    let mut result: Vec<String> = vec![];
    cpu.run_with_callback(|cpu| {
        result.push(trace(cpu));
    });
    assert_eq!(
        "0064  A9 42     LDA #$42                        A:42 X:00 Y:00 P:24 SP:FD",
        result[0]
    );
}

#[test]
fn test_format_opcode_with_zero_page_addressing_mode() {
    let mut bus = Bus::new(test_rom(), |ppu: &NesPPU| {});
    bus.mem_write(100, 0xa5);
    bus.mem_write(101, 0x10);
    bus.mem_write(0x10, 0x55);

    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x64;
    cpu.register_a = 0x00;
    let mut result: Vec<String> = vec![];
    cpu.run_with_callback(|cpu| {
        result.push(trace(cpu));
    });
    assert_eq!(
        "0064  A5 10     LDA $10 = 55                    A:55 X:00 Y:00 P:24 SP:FD",
        result[0]
    );
}

#[test]
fn test_format_opcode_with_absolute_addressing_mode() {
    let mut bus = Bus::new(test_rom(), |ppu: &NesPPU| {});
    bus.mem_write(100, 0xad);
    bus.mem_write(101, 0x34);
    bus.mem_write(102, 0x12);
    bus.mem_write(0x1234, 0x99);

    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x64;
    cpu.register_a = 0x00;
    let mut result: Vec<String> = vec![];
    cpu.run_with_callback(|cpu| {
        result.push(trace(cpu));
    });
    assert_eq!(
        "0064  AD 34 12  LDA $1234 = 99                  A:99 X:00 Y:00 P:24 SP:FD",
        result[0]
    );
}

#[test]
fn test_format_opcode_with_indirect_x_addressing_mode() {
    let mut bus = Bus::new(test_rom(), |ppu: &NesPPU| {});
    bus.mem_write(100, 0xa1);
    bus.mem_write(101, 0x10);
    bus.mem_write(0x12, 0x34);
    bus.mem_write(0x35, 0x56);
    bus.mem_write(0x5634, 0x78);

    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x64;
    cpu.register_x = 0x02;
    cpu.register_a = 0x00;
    let mut result: Vec<String> = vec![];
    cpu.run_with_callback(|cpu| {
        result.push(trace(cpu));
    });
    assert_eq!(
        "0064  A1 10     LDA ($10,X) @ 12 = 5634 = 78    A:78 X:02 Y:00 P:24 SP:FD",
        result[0]
    );
}

#[test]
fn test_format_opcode_with_indirect_y_addressing_mode() {
    let mut bus = Bus::new(test_rom(), |ppu: &NesPPU| {});
    bus.mem_write(100, 0xb1);
    bus.mem_write(101, 0x10);
    bus.mem_write(0x10, 0x34);
    bus.mem_write(0x11, 0x12);
    bus.mem_write(0x1234, 0x78);

    let mut cpu = CPU::new(bus);
    cpu.program_counter = 0x64;
    cpu.register_y = 0x02;
    cpu.register_a = 0x00;
    let mut result: Vec<String> = vec![];
    cpu.run_with_callback(|cpu| {
        result.push(trace(cpu));
    });
    assert_eq!(
        "0064  B1 10     LDA ($10),Y = 1234 @ 1236 = 78  A:78 X:00 Y:02 P:24 SP:FD",
        result[0]
    );
}
