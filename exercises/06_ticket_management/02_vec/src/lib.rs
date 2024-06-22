// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// 3: 1
// 4: 2
// 5: 3
// 6: 5
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
// fibs = Vec[u32]
// const FIBS: Vec<u32> = Vec::new();
const FIBS: Vec<u32> = Vec::new();

pub fn fibonacci(n: u32) -> u32 {
    // Declare an array to store
    // Fibonacci numbers.
    // 1 extra to handle
    // case, n = 0
    // int f[n + 2];
    let mut fibs: Vec<u32> = Vec::new();
    fibs.push(0);
    fibs.push(1);

    // 0th and 1st number of the
    // series are 0 and 1
    // f[0] = 0;
    // f[1] = 1;

    let mut i = 2;
    while i <= n {
        println!("i:{}", i);
        // f[i] = f[i - 1] + f[i - 2];
        fibs.push(fibs[(i - 1) as usize] + fibs[(i - 2) as usize]);
        i += 1;
    }
    // while for (i = 2; i <= n; i++) {

    // Add the previous 2 numbers
    // in the series and store it
    // }
    return fibs[n as usize];
    // return f[n];
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    // todo!()
    // let itsinthere: bool = false;
    // let x = &mut [0, 1, 2];

    // let index = n as usize;
    // if let Some(elem) = FIBS.get(index) {
    // println!("found element");
    // return 7; // println!("found element")
    // } else {
    // println!("did not find element");
    // return 7; // println!("found element")
    // }
    // // *elem = 42;
    // // }
    // // assert_eq!(x, &[0, 42, 2]);
    // // if let Some(ans) = FIBS.get(n.into()) {
    // // itsinthere = true;
    // // }
    // // match FIBS.get(n as u8) {
    // // None => {
    // if n == 0 || n == 1 {
    // FIBS.push(n);
    // n
    // } else {
    // FIBS.push(n);
    // fibonacci(n - 1) + fibonacci(n - 2)
    // }
    // }
    // Some(&n) => n,
    // }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
