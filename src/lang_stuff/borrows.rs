// One mutable OR many immutable
// RAII

pub mod borrows {
    fn take_ownwership_sum(v: Vec<i32>) -> i32 {
        let mut sum = 0;
        for value in v {
            sum += value;
        }
        sum
    }

    // VS

    fn borrow_sum(v: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for value in v {
            sum += *value; // de-reference looks at the actual value
        }
        sum
    }

    pub fn imperative_sum() {
        let values = vec![1,2,3,4,5];
       // let sum = take_ownwership_sum(values);
        let sum = borrow_sum(&values);
        println!("{} {}", values.len(), sum);
    }
}
