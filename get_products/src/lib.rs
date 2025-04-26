pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    let mut result = vec![1; n];

   
    if n == 0 || n == 1 {
        return Vec::new();
    }

   
    let mut left_product = 1;
    for i in 0..n {
        result[i] = left_product;
        left_product *= arr[i];
    }

   
    let mut right_product = 1;
    for i in (0..n).rev() {
        result[i] *= right_product;
        right_product *= arr[i];
    }

    result
}