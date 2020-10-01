fn main() {
    let x = ["English", "This", "sentence", "a", "in", "is"];
    print!("{} {} {} {} {} {} \n\n", x[1], x[5], x[3], x[2], x[4], x[0]);

    //mutable array
    let mut x = ["This", "is", "a", "sentence"];
    x[2] = "a nice";
    print!("{} {} {} {}. \n\n", x[0], x[1], x[2], x[3]);

    let mut x = ["a", "b", "c"];
    print!("{}{}{}. ", x[0], x[1], x[2]);
    x = ["X", "Y", "Z"];
    print!("{}{}{}. ", x[0], x[1], x[2]);
    let y = ["1", "2", "3"];
    x = y;
    print!("{}{}{}. \n\n", x[0], x[1], x[2]);

    //Multidimensional Arrays
    let mut x = [[[[23; 4]; 6]; 8]; 15];
    x[14][7][5][3] = 56;
    x[14][6][5][3] = 53;
    print!("{}, {}, {} \n\n", x[0][0][0][0], x[14][7][5][3], x[14][6][5][3]);

    //Vector
    let mut y = vec!["This", "is"];
    print!("{} \n", y.len());
    y.push("a");
    print!("{} \n", y.len());
    y.push("vector!");
    print!("{} \n", y.len());

    for i in 0..y.len(){
        print!(" {}", y[i]);
    }

}
