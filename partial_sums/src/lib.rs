pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    let mut sum = arr.iter().copied().sum::<u64>(); // Start with the total sum of the array
    
    result.push(sum); // Add the total sum to the result vector
    
    for &val in arr.iter().rev() {
        sum -= val; // Subtract the current value from the sum
        result.push(sum); // Add the new sum after subtraction
    }
    
    result
}
