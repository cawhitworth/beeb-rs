mod ROMs;

use ROMs::testROM1;

fn main() {
    for inst in testROM1() {
        println!("Decode: {:02x}", inst);
    }
}
