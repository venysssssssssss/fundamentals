

fn main() {
    println!("Hello, world!");


    example_array();
}


fn example_array() {
    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    // insert number 6 on the array
    let mut arr: [i32; 6] = [0; 6];
    arr[..5].copy_from_slice(&[1, 2, 3, 4, 5]);
    arr[5] = 6;
    println!("{:?}", arr);
}



