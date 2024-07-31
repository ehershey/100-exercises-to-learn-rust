// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(v: Vec<i32>) -> i32 {
    let length = v.len();

    if length == 0 {
        return 0;
    }
    if length == 1 {
        return v[0];
    }
    let half = length / 2;
    println!("length is: {}, half is: {}", length, half);
    let slice1 = &v[..half - 1];
    let slice2 = &v[half - 1..];

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for num in slice1 {
                sum1 += num;
                println!("summing in first thread");
            }
        });

        scope.spawn(|| {
            for num in slice2 {
                sum2 += num;
                println!("summing in second thread");
            }
        });
    });
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
