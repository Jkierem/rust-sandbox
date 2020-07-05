fn maybe_add (a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a,b) {
        (Some(i), Some(j)) => Some(i+j),
        (Some(i), None) => Some(i),
        (None, Some(i)) => Some(i),
        (_ , _) => None,
    }
}

fn main() {
    println!("{:?}",maybe_add(Some(5),Some(2)));
    println!("{:?}",maybe_add(Some(5),None));
    println!("{:?}",maybe_add(None,Some(2)));
}
