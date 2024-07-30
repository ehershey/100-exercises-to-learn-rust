// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // create new vectors for first half and second half of given vector
    // spawn one thread to sum the first half
    // spawn another thread to sum the second half
    // join first handle
    // join second handle
    // return sum of two half-sums
    //

    let length = v.len();

    if length == 0 {
        return 0;
    }
    if length == 1 {
        return v[0];
    }
    let half = length / 2;
    println!("length is: {}, half is: {}", length, half);
    let vector1: Vec<i32> = v[..half - 1].to_vec();
    let vector2: Vec<i32> = v[half - 1..].to_vec();
    println!("vector1 is: {:?}, vector2 is: {:?}", vector1, vector2);

    let handle1 = std::thread::spawn(move || {
        let mut sum1: i32 = 0;
        for num in vector1.iter() {
            sum1 += num;
        }
        sum1
    });

    let handle2 = std::thread::spawn(move || {
        let mut sum2: i32 = 0;
        for num in vector2.iter() {
            sum2 += num;
        }
        sum2
    });
    let sum1 = handle1.join().unwrap();
    let sum2 = handle2.join().unwrap();
    println!("sum1 is: {:?}, sum2 is: {:?}", sum1, sum2);

    sum1 + sum2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
