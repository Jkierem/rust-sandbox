fn main() {
}

fn gives_ownership() -> String {
    let st = String::from("some string");
    st
}

fn takes_and_gives_back(some_string: String) -> {
    println!("{}", some_string);
    some_string
}


