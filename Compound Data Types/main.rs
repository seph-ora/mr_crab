// Compound Data Types
// arrays, tuples, slices, and strings (sliced string)

// Arrays 
// Fixed sized of elements of the same (homogenous)type
fn main(){
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element {}", fruits[0]);
    println!("Fruits Array 2nd element {}", fruits[1]);
    println!("Fruits Array 2nd element {}", fruits[2]);
    // //////////////////////////////////////////////////

    // Tuples
    // Contain heterogenous collection of elements of fixed size 
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5]
    // Dynamically sized view into a contagious(uninterupted, adjacent to one another ) sequence of elements 
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // Strings Vs String Slices (&str)
    // Strings [growable, mutable, owned string type (not borrowed)]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says: {}", stone_cold);

    // B- &str (String slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string;
    println!("Slice Value: {}", slice);
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}