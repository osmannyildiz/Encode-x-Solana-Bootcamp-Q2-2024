# Week 1 Session 4 Homework

## Sending Lamports

```sh
solana config set --url devnet
solana airdrop 1 <ACCOUNT_1_ADDRESS>

solana balance <ACCOUNT_1_ADDRESS>
solana balance <ACCOUNT_2_ADDRESS>

solana transfer --from <ACCOUNT_1_KEYPAIR_PATH> --fee-payer <ACCOUNT_1_KEYPAIR_PATH> <ACCOUNT_2_ADDRESS> 0.2 --allow-unfunded-recipient

solana balance <ACCOUNT_1_ADDRESS>
solana balance <ACCOUNT_2_ADDRESS>
```

## Coding Challenge: Fizz Buzz

1. Create a new project using Cargo.
2. The main function should print a welcome message.
3. Write a "fizz buzz" function that will be called from your main function.
    - The function should have a loop counting up to 301.
    - If the count is divisible by 3, print "fizz".
    - If the count is divisible by 5, print "buzz".
    - If the count is divisible by 3 and 5, print "fizz buzz".
    - At the end print the number of times "fizz buzz" occurred.

## Coding Challenge: Two Sum

We have a vector of integers called `nums` and a `target` integer. Return the two indices that add up to the `target` value.

Rules:

- There's always one unique solution for each list.
- You can't use the same number twice. *(Osman's note: Here "same" refers to the index, not to the value. See Example 3.)*

Example 1:

```text
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0,1].
```

Example 2:

```text
Input: nums = [3,2,4], target = 6
Output: [1,2]
```

Example 3:

```text
Input: nums = [3,3], target = 6
Output: [0,1]
```

Starting code:

```rust
fn main() {
    println!("{:?}", two_sum(vec![2, 3, 4, 5,], 9));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Your code goes here
}
```

There's a brute force solution that's a bit easier to figure out, but see if you can also use a `HashMap` for a more efficient solution.
