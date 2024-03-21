mod memory;
mod cpu;
mod operators;

fn main() {
    let cpu = cpu::Cpu::new();

    println!("{:#?}", cpu);
}
