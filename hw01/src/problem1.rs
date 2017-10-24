// problem1.rs

/// Functions are private (only available to this module) by default.
/// Use the `pub` keyword to mark this function as public.

/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    if slice.len() == 0 { 0 }
    else { slice[0] + sum(&slice[1..]) }
}

// Imperative version
// pub fn sum(slice: &[i32]) -> i32 {
//     let mut sum = 0;
    
//     for e in slice {
//         sum += *e
//     }
//     sum
// }

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinc element of `vs`, preserving the
/// original order
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    
    for e in vs {
        if !result.contains(e) {
            result.push(*e)
        }
    }
    result   
}


/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut result = Vec::new();

    for e in vs {
        if pred(*e) {
            result.push(*e)
        }
    }
    result
}
