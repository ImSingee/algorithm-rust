#[cfg(feature = "brute_force")]
mod brute_force {
    use std::cmp::min;

    struct CustomStack {
        max_size: usize,
        vec: Vec<i32>,
    }

    impl CustomStack {
        fn new(max_size: i32) -> Self {
            CustomStack {
                max_size: max_size as usize,
                vec: Vec::with_capacity(max_size as usize),
            }
        }

        fn push(&mut self, x: i32) {
            if self.vec.len() >= self.max_size {
                return;
            }

            self.vec.push(x);
        }

        fn pop(&mut self) -> i32 {
            self.vec.pop().unwrap_or(-1)
        }

        fn increment(&mut self, k: i32, val: i32) {
            let k = min(self.vec.len(), k as usize);

            for i in 0..k {
                self.vec[i] += val;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let mut obj = CustomStack::new(3);
            obj.push(1);
            obj.push(2);
            assert_eq!(obj.pop(), 2);
            obj.push(2);
            obj.push(3);
            obj.push(4);
            obj.increment(5, 100);
            obj.increment(2, 100);
            assert_eq!(obj.pop(), 103);
            assert_eq!(obj.pop(), 202);
            assert_eq!(obj.pop(), 201);
            assert_eq!(obj.pop(), -1);
        }
    }
}

#[cfg(feature = "pre_sum")]
mod pre_sum {
    use std::cmp::min;

    struct CustomStack {
        max_size: usize,
        vec: Vec<(i32, i32)>,
    }

    impl CustomStack {
        fn new(max_size: i32) -> Self {
            CustomStack {
                max_size: max_size as usize,
                vec: Vec::with_capacity(max_size),
            }
        }

        fn push(&mut self, x: i32) {
            if self.vec.len() >= self.max_size {
                return;
            }

            self.vec.push((x, 0));
        }

        fn pop(&mut self) -> i32 {
            let v = self.vec.pop();

            match v {
                Some((v, inc)) => {
                    if let Some((a, x)) = self.vec.pop() {
                        self.vec.push((a, x + inc));
                    }

                    v + inc
                }
                None => -1,
            }
        }

        fn increment(&mut self, k: i32, val: i32) {
            let i = min(self.vec.len(), k as usize);

            if i > 0 {
                let i = i - 1;
                let (v, inc) = self.vec[i];
                self.vec[i] = (v, inc + val);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let mut obj = CustomStack::new(3);
            obj.push(1);
            obj.push(2);
            assert_eq!(obj.pop(), 2);
            obj.push(2);
            obj.push(3);
            obj.push(4);
            obj.increment(5, 100);
            obj.increment(2, 100);
            assert_eq!(obj.pop(), 103);
            assert_eq!(obj.pop(), 202);
            assert_eq!(obj.pop(), 201);
            assert_eq!(obj.pop(), -1);
        }
    }
}
