fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let index = 10;

    // This will panic at runtime if index is out of bounds
    let element = vec[index];
    println!("Element at index {}: {}", index, element);
}