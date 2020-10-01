fn main() {
    const EUROPE: u8 = 0;
    const ASIA: u8 = 1;
    const AFRICA: u8 = 2;
    const AMERICA: u8 = 3;
    const OCEANIA: u8 = 4;

    let continent = 5;

    if continent == EUROPE {
        print!("E");
    } else if continent == ASIA {
        print!("As");
    } else if continent == AFRICA {
        print!("Af");
    } else if continent == AMERICA {
        print!("Am");
    } else if continent == OCEANIA {
        print!("O");
    } else {
        print!("Nothing\n\n");
    }

    enum Continent {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }

    let contin = Continent::Oceania;

    match contin {
        Continent::Europe => print!("E"),
        Continent::Asia => print!("As"),
        Continent::Africa => print!("Af"),
        Continent::America => print!("Am"),
        Continent::Oceania => print!("O"),
    }

    enum CardinalPoint {
        North,
        South,
        West,
        East,
    };
    let direction = CardinalPoint::South;
    print!(
        "\n\nCardinal: {}\n",
        match direction {
            CardinalPoint::North => 'N',
            CardinalPoint::South => 'S',
            _ => '*',
        }
    );

    for i in -2..5 {
        println!(
            "\n{} is {}.",
            i,
            match i {
                0 => "zero",
                1 => "one",
                _ if i < 0 => "negative",
                _ => "plural",
            }
        );
    }
}
