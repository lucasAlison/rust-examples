use std::collections::LinkedList;

pub fn execute() {
    let mut list1 = LinkedList::new();
    list1.push_back('a');
    list1.push_back('b');
    list1.push_back('c');
    list1.push_back('d');

    let mut iter = list1.iter_mut();
    assert_eq!(iter.next(), Some(&mut 'a'));
    assert_eq!(iter.next(), Some(&mut 'b'));
    assert_eq!(iter.next(), Some(&mut 'c'));
    assert_eq!(iter.next(), Some(&mut 'd'));

    let mut list2 = LinkedList::new();
    list2.push_back('1');
    list2.push_front('2');
    list2.push_back('3');

    let mut iter2 = list2.iter_mut();
    assert_eq!(iter2.next(), Some(&mut '2'));
    assert_eq!(iter2.next(), Some(&mut '1'));
    assert_eq!(iter2.next(), Some(&mut '3'));
}