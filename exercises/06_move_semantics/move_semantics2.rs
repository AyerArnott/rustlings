// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.



#[test]
fn main() {
    let vec0 = vec![22, 44, 66];
    let vec2 = vec![22,44,66];  //method 1 used to make a new vec with vec0 elements 
    //let mut vec1 = fill_vec(vec2) //since we cannot take original vec as arg for fill_vec.
    let mut vec1 = fill_vec(&vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

//methodd 2 is used to make fill_vec take a reference instead of orginal vec 
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut cloned_vec = vec.clone(); //clone() used to make acopy and store changes in var cloned_vec

    cloned_vec.push(88);

    cloned_vec      //return copied vec
}
