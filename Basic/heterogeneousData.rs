fn main() {
    let data = (10, 'x', 12, 183.19, 'Q', false, -9);
    print!("{}", data.2 + data.6);

    struct SomeData {
        integer: i32,
        fractional: f32,
        character: char,
        five_bytes: [u8; 5],
    }
    let data = SomeData {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200],
    };
    print!(
        "{}, {}, {}, {}",
        data.five_bytes[3], data.integer, data.fractional, data.character
    );

    //Lexical Conventions
    const MAXIMUM_POWER: u16 = 600;
    enum VehicleKind {
        Motorcycle,
        Car,
        Truck,
    };
    struct VehicleData {
        kind: VehicleKind,
        registration_year: u16,
        registration_month: u8,
        power: u16,
    };
    let vehicle = VehicleData {
        kind: VehicleKind::Motorcycle,
        registration_year: 2003,
        registration_month: 11,
        power: 677,
    };

    if vehicle.power > MAXIMUM_POWER {
        println!("\n\nToo powerful");
    }
}
