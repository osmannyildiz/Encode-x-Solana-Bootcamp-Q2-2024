fn main() {
    println!("{:?}", two_sum(vec![2, 3, 4, 5,], 9));
}

// Using the brute force solution
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i.try_into().unwrap(), j.try_into().unwrap()];
            }
        }
    }

    // Panic if no match
    // (Might make sense to return an Option instead)
    panic!("No solutions found");
}

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let actual_output = two_sum(nums, target);

        let expected_output = vec![0, 1];
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        let actual_output = two_sum(nums, target);

        let expected_output = vec![1, 2];
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![3, 3];
        let target = 6;

        let actual_output = two_sum(nums, target);

        let expected_output = vec![0, 1];
        assert_eq!(actual_output, expected_output);
    }
}
