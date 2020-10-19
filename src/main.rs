mod data_structure;


use data_structure::binary_tree;
use data_structure::queue::Queue;
use data_structure::stack::Stack;

fn test_stack() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct {
        a: i32,
    }
    let a = TestStruct { a: 5 };
    let b = TestStruct { a: 9 };
    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);
    s.push(&a);
    s.push(&b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}

fn test_queue() {}



fn main() {
    test_stack();


}
