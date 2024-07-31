// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let static_ref = Vec::leak(v);

    let length = static_ref.len();

    if length == 0 {
        return 0;
    }
    if length == 1 {
        return static_ref[0];
    }
    let half = length / 2;
    println!("length is: {}, half is: {}", length, half);
    let slice1 = &static_ref[..half - 1];
    let slice2 = &static_ref[half - 1..];

    let handle1 = std::thread::spawn(move || {
        let mut sum1: i32 = 0;
        //for num in static_ref[..(static_ref.len() / 2 - 1)].iter() {
        for num in slice1 {
            sum1 += num;
        }
        sum1
    });

    let handle2 = std::thread::spawn(move || {
        let mut sum2: i32 = 0;
        for num in slice2 {
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
