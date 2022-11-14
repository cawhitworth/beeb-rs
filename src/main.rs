mod roms;

use roms::test_rom1;

fn main() {
    for inst in test_rom1() {
        println!("Decode: {:02x}", inst);
    }
}
