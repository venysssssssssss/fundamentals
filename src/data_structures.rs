fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    // Call the functions for different data structures
    example_array();
    example_vector();
    example_linked_list();
    example_stack();
    example_queue();
    example_hash_map();
    example_hash_set();
    // ... add more data structures as needed
}

fn example_array() {
    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
}

fn example_vector() {
    // Vector
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);
}

fn example_linked_list() {
    // Linked List
    // Implement your own linked list or use a crate like `linked-list`
    // and demonstrate its usage here
}

fn example_stack() {
    // Stack
    // Implement your own stack or use a crate like `stack`
    // and demonstrate its usage here
}

fn example_queue() {
    // Queue
    // Implement your own queue or use a crate like `queue`
    // and demonstrate its usage here
}

fn example_hash_map() {
    // Hash Map
    let mut map: std::collections::HashMap<&str, i32> = std::collections::HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    println!("{:?}", map);
}

fn example_hash_set() {
    // Hash Set
    let mut set: std::collections::HashSet<i32> = std::collections::HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("{:?}", set);
}