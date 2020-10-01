fn main() {
    let mut i = 1;
    let mut limit = 20;
    // using loop
    loop {
        let ii = i + i;
        if ii > 200 {
            break;
        }
        print!("Using loop: {} \n\n", ii);
        i += 1;
    }

    // using while
    i = 1;
    while i < 50 {
        print!("Using while: {} \n\n", i);
        i += 1;
    }

    //using for

    for i in 1..limit {
        print!("Using for: {} \n\n", i);
    }
}
