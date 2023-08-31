/*
Vector is continuous in memory according to the rust programming book. chap 8.1
It should be understood how it is continuous in memory.
*/
pub fn crud_vector() {
    //let mut vector : Vec<i32> = Vec::from([1,2,3]);
    let mut vector = vec![1, 2, 3];

    vector.push(5);
    vector.push(6);
    vector.push(7);

    println!("vector: {:?}", vector);

    println!("vector's first element is {}", vector[1]);

    let tenth_element = vector.get(10);

    match tenth_element {
        Some(tenth) => println!("vector's tenth element is {}", tenth),
        None => println!("vector's tenth element is Nothing!"),
    };

    /*
    The reason using these two way is that,
    the firstone will panic the program when accessing out of its bound.
    the secondone will not panic and just return None.
    Two method can be used together by its purpose.
     */
    let first_element1 = &vector[0];
    let first_element2 = vector.get(0);
}

pub fn vector_borrowing_modifying() {
    let mut v = vec![1, 2, 3];

    let first = v[0];

    v.push(21);

    println!("test {:?}", first);
}
