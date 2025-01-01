// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

// This example uses the main thread and the spawned thread but what we are 
// looking for is the use of two spawned threads. In this example, because
// there is only one spawned thread there are no 'competing' threads.
// This means that the 'move' keyword is not needed to take ownership of the
// variable within the closure.
// pub fn sum(slice: &'static [i32]) -> i32 {
//     let (first, second) = slice.split_at(slice.len() / 2);
//     let sum1: i32;
//     let sum2;
//     let handle = thread::spawn(|| {
//         first.iter().sum::<i32>()
//     });

//     sum1 = handle.join().unwrap();
//     sum2 = second.iter().sum::<i32>();
//     sum1 + sum2
// }

// Here is what the solution looks like - it is best practice to use the 
// move keyword in this instance
pub fn sum(slice: &'static [i32]) -> i32 {
    let mid = slice.len() / 2;
    let (slice1, slice2) = slice.split_at(mid);

    let handle1 = thread::spawn(move || slice1.iter().sum::<i32>());
    let handle2 = thread::spawn(move || slice2.iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
