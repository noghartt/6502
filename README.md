## 6502

The proposal of this project is to write a simple 6502 emulator in Rust.

### Memory address

The 6502 has a 8-bit address bus, which means that it can address 256 bytes of memory.

The memory is divided into three regions:

| Region      | Contents        | Description          |
| $0000-$00FF | Zero Page       | 256 bytes of memory  |
| $0100-$01FF | Stack           | 256 bytes of memory  |
| $0200-$07FF | General purpose | 1536 bytes of memory |

- **Zero page**: the first 256 bytes of the memory are called the _zero page_.
  It is a special region of memory that can be accessed with a single byte instruction.
  This is because the zero page is the first page of memory and the 6502 has an 8-bit address bus.
  This means that the zero page can be addressed with a single byte, while the rest of the memory requires two bytes to address.
- **Stack**: the 6502 has a stack that is used to store the return address of subroutines and the state of the processor.
  The stack is located in the second page of memory, from $0100 to $01FF.
- **General purpose**: the rest of the memory is used for general purpose data storage.

### Resources

- [Writing a 6502 emulator](https://www.ahl27.com/posts/2023/01/6502-emu1/)
- [Writing a 6502 emulator in Python](https://dailystuff.nl/projects/writing-a-6502-emulator-in-python.html)
- [Writing your own NES emualator - the 6502 CPU](https://yizhang82.dev/nes-emu-cpu)
