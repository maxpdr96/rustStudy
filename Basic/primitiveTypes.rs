fn main() {
    let hexa = 0x_00FF_F7A3;
    let decimal = 1_234_567;
    let octal = 0o_777_205_162;
    let binary = 0b_0110_1001_1111_0001;

    print!("{} {} {} {} \n\n", hexa, decimal, octal, binary);

    //exponential notation
    let one_thousand = 1e3;
    let one_million = 1e6;
    let thirteen_billions_and_half = 13.5e9;
    let twelve_millionths = 12e-6;

    print!(
        "{} {} {} {} \n\n",
        one_thousand, one_million, thirteen_billions_and_half, twelve_millionths
    );

    let a: i8 = 5;
    let b: i16 = 5;
    let c: i32 = 5;
    let d: i64 = 5;
    print!("{} {} {} {}", a, b, c, d);

    let a: i16 = 5;
    let b: i16 = 18;
    print!(" {} \n\n", a + b);

    let a: f64 = 4.6;
    let b: f32 = 3.91;
    print!("{} {} \n\n", a, b);

    let a: i16 = 12;
    let b: u32 = 4;
    let c: f32 = 3.7;
    print!("{} \n", a as i8 + b as i8 + c as i8);

    let a = 255 as u8;
    let b = 100_000 as u32;
    let c = 10_000_000_000 as u64;
    print!("{} {} {}\n\n", a, b, c);

    let e_grave = 'è';
    let japanese_character = 'さ';
    println!("{} {}", e_grave, japanese_character);

    //Array and Vector types
    let _array1: [char; 3] = ['x', 'y', 'z'];
    let _array2: [f32; 200] = [0f32; 200];
    let _vector1: Vec<char> = vec!['x', 'y', 'z'];
    let _vector2: Vec<i32> = vec![0; 5000];

    const N: usize = 20;
    let teste = [0; N];
    print!("\n{:?}\n\n", teste);

}
