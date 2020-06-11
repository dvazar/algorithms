/// Selection sorting algorithm, O(n^2)
///
/// Input: <a1, a2, ..., an>.
/// Output: <a1', a2', ..., an'>, where a1' <= a2' <= ... <= an'.
///

pub fn selection_sort (array: &mut Vec<i32>) {
    for i in (1..array.len()).rev() {
        let mut max_val = i32::MIN;
        let mut max_idx: usize = 0;
        for j in 0..i+1 {
            if array[j] >= max_val {
                max_val = array[j];
                max_idx = j;
            }
        }
        for k in max_idx..i {
            array[k] = array[k+1];
        }
        array[i] = max_val;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![1, 2, 3, 4, 5, 6];

        selection_sort(&mut array);

        assert_eq!(array, expected_array);
    }

}
