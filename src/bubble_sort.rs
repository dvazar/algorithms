/// Bubble sorting algorithm, O(n^2)
/// 
/// Input: <a1, a2, ..., an>.
/// Output: <a1', a2', ..., an'>, where a1' <= a2' <= ... <= an'.
/// 

pub fn bubble_sort <T: Ord + Copy> (array: &mut Vec<T>) {
    for i in 0..array.len()-2 {
        for j in (i+1..array.len()).rev() {
            if array[j] < array[j-1] {
                let tmp = array[j];
                array[j] = array[j-1];
                array[j-1] = tmp;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_i32() {
        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![1, 2, 3, 4, 5, 6];
        
        bubble_sort(&mut array);
        
        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_bubble_sort_chars() {
        let mut array = vec!['b', 'f', 'c', 'a', 'e', 'd'];
        let expected_array = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        
        bubble_sort(&mut array);
        
        assert_eq!(array, expected_array);
    }

}
