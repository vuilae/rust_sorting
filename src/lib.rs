
// Quick sort algorithm
pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // Base case: If array size is 1 or less, it's already sorted
    }
    let pivot_index = partition(arr); // Choose pivot and partition the array
    quick_sort(&mut arr[..pivot_index]); // Recursively sort the left partition
    quick_sort(&mut arr[pivot_index + 1..]); // Recursively sort the right partition
}

// Helper function for partitioning in quick sort
fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2; // Choose pivot index as middle of the array
    arr.swap(pivot_index, arr.len() - 1); // Swap pivot with the last element
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j); // Swap elements less than pivot to the left
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1); // Swap pivot to its correct position
    i // Return the index where pivot is placed
}

// Selection sort algorithm
pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j; // Find index of the minimum element
            }
        }
        if i != min_index {
            arr.swap(i, min_index); // Swap the minimum element with current element
        }
    }
}

// Insertion sort algorithm
pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j); // Swap elements until the current element is in correct position
            j -= 1;
        }
    }
}

// Merge sort algorithm
pub fn merge_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // Base case: If array size is 1 or less, it's already sorted
    }
    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec(); // Split array into left and right halves
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left); // Recursively sort the left half
    merge_sort(&mut right); // Recursively sort the right half

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            arr[k] = left[i].clone(); // Merge sorted halves into original array
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    // Copy remaining elements from left and right halves if any
    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

// Define a module for sorting utility functions
pub mod utils {
// Function to perform sorting based on given algorithm
pub fn sort<T, F>(arr: &mut [T], algorithm: F)
    where
        T: PartialOrd,
        F: FnOnce(&mut [T]),
{
    algorithm(arr); // Call the sorting algorithm provided as a function argument
}
}
