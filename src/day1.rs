use std::collections::HashSet;
mod file_reading;

fn find_pair(nums: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    let mut diffs: HashSet<&i32> = HashSet::new();
    for n in nums {
        diffs.insert(n);
        let diff: i32 = sum - n;
        if diffs.contains(&diff) {
            return Some((diff, *n));
        };
    }
    None
}

fn find_three(mut nums: Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    nums.sort();
    for start in 0..(nums.len() - 2) {
        let mut mid = start + 1;
        let mut end = nums.len() - 1;
        while mid < end {
            let total = nums[start] + nums[mid] + nums[end];
            if total == sum {
                return Some((nums[start], nums[mid], nums[end]));
            } else if total > sum {
                end -= 1
            } else {
                mid += 1
            }
        }
    }
    None
}

fn main() {
    if let Ok(numbers) = file_reading::read_numbers("./input1.txt") {
        let sum = 2020;
        if let Some((p1, p2)) = find_pair(&numbers, sum) {
            let product = p1 * p2;
            println!("Part 1: {}", product);
        }
        if let Some((p1, p2, p3)) = find_three(numbers, sum) {
            let product = p1 * p2 * p3;
            println!("Part 2: {}", product);
        }
    }
}
