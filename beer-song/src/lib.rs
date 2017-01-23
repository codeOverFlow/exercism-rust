pub fn beer_string(num: i64) -> String {
    match num {
        0 => String::from("No more bottles"),
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", num),
    }
}

pub fn verse(beer_on_the_wall: i64) -> String {
    let left = beer_on_the_wall - 1;
    let do_not_left = left == 0;

    let take_it_or_go_to_the_store = match beer_on_the_wall == 0 {
        true => String::from("Go to the store and buy some more"),
        false => String::from("Take ") + if do_not_left { "it" } else { "one" } + " down and pass it around",
    };

    let refill = match left < 0 {
        true => beer_string(99),
        false => beer_string(left).to_lowercase(),
    };

    format!("{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n", 
            beer_string(beer_on_the_wall), 
            beer_string(beer_on_the_wall).to_lowercase(),
            take_it_or_go_to_the_store,
            refill)
}

pub fn sing(beer_on_the_wall: i64, target: i64) -> String {
    (target..beer_on_the_wall+1).rev().map(|beer| verse(beer)).collect::<Vec<_>>().join("\n")
}
