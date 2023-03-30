fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    for x in a {
        if !b.contains(&x) {
            result.push(x);
        }
    }
    result
}

// Example usage:
fn main() {
    let a = vec![1, 2, 2, 2, 3];
    let b = vec![2];
    let diff = array_diff(a, b);
    println!("Difference: {:?}", diff);
}