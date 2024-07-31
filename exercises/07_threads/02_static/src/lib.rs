// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let length = slice.len();

    if length == 0 {
        return 0;
    }
    if length == 1 {
        return slice[0];
    }
    let half = length / 2;
    println!("length is: {}, half is: {}", length, half);

    let handle1 = std::thread::spawn(move || {
        let mut sum1: i32 = 0;
        for num in slice[..half - 1].iter() {
            sum1 += num;
        }
        sum1
    });

    let handle2 = std::thread::spawn(move || {
        let mut sum2: i32 = 0;
        for num in slice[half - 1..].iter() {
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
