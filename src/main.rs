mod memory;
mod cpu;
mod operators;
mod instructions;
mod address_mode;
mod fixtures;

fn main() {
    let cpu = cpu::Cpu::new();

    println!("{:#?}", cpu);
}
