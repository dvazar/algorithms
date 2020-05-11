/// Insertion sorting algorithm
/// 
/// Input: <a1, a2, ..., an>.
/// Output: <a1', a2', ..., an'>, where a1' <= a2' <= ... <= an'.
/// 

pub fn insertion_sort (array: &mut Vec<i32>) {
    for j in 1..array.len() {
        let key = array[j];
        let mut i = (j - 1) as isize;
        while (i >= 0) && (array[i as usize] > key) {
            array[(i + 1) as usize] = array[i as usize];
            i -= 1;
        }
        array[(i + 1) as usize] = key;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut array = vec![1, 3, 2, 4, 0];
        let expected_array = vec![0, 1, 2, 3, 4];

        insertion_sort(&mut array);
        
        assert_eq!(array, expected_array);

        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![1, 2, 3, 4, 5, 6];
        
        insertion_sort(&mut array);
        
        assert_eq!(array, expected_array);
    }
}
