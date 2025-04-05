pub fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    
    // Outer loop: go through the entire list multiple times
    while n > 1 {
        let mut new_n = 0;
        
        // Inner loop: compare adjacent elements and swap if necessary
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i); 
                new_n = i; // Update the new_n to avoid rechecking the sorted part
            }
        }
        
        // Decrease n to avoid checking the last sorted element
        n = new_n;
    }
}
