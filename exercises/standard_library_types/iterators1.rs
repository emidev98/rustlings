// iterators1.rs
//
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and
// how to go through elements within an iterable collection.
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a hint.

fn main () {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry", "randomfruit"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
<<<<<<< HEAD
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 2.1
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"randomfruit"));     // TODO: Step 3
=======
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 4
>>>>>>> 96098d228a90c435549847698d2c12b771af7464
}
