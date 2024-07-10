# NES Emulator

This project is a work-in-progress NES emulator written in Rust. It aims to emulate the Nintendo Entertainment System (NES), allowing users to play NES games on their computer.

## Features

- **CPU Emulation**: Implements the Ricoh 2A03 CPU, which includes the MOS Technology 6502 processor used in the NES.
- **PPU Emulation**: Provides emulation for the Picture Processing Unit (PPU) used in the NES for rendering graphics.
- **Memory Mapping**: Manages memory mapping to simulate the NES's memory architecture.
- **Cartridge Support**: Supports NES ROM cartridge loading and emulation.
- **Input Handling**: Basic support for NES controller input.

## Components

The emulator consists of several major components:

- **CPU**: Implements the 6502 CPU with all official instructions and addressing modes.
- **PPU**: Emulates the NES's PPU for rendering graphics.
- **Memory Mapper**: Handles memory mapping to simulate the NES memory layout, including RAM, ROM, and I/O registers.
- **Cartridge Loader**: Loads NES ROM cartridges (`.nes` files) for emulation.
- **Input Handling**: Manages input from the user via keyboard or game controller.

## Usage

1. **Prerequisites**: Ensure you have Rust installed. You can install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).

2. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/nes-emulator.git
   cd nes-emulator
   ```

3. **Build and Run**:

   ```bash
   cargo build
   cargo run -- <path_to_your_nes_rom>.nes
   ```

4. **Controls**:
   - Keyboard keys mapped to NES controller buttons (e.g., arrow keys, A, B, Start, Select).

## Future Enhancements

- Audio emulation (APU).
- Save states support.
- Debugger and profiler tools.
- Improved compatibility with a wider range of NES games.
- Graphical user interface (GUI) for better user interaction.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

