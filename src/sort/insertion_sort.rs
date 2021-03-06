/// Insertion sorting algorithm, O(n^2)
/// 
/// Input: <a1, a2, ..., an>.
/// Output: <a1', a2', ..., an'>, where a1' <= a2' <= ... <= an'.
/// 

pub fn insertion_sort <T: Ord + Copy> (array: &mut Vec<T>) {
    for i in 1..array.len() {
        let key = array[i];
        let mut k = i;
        for j in (0..i).rev() {
            if array[j] > key {
                array[j + 1] = array[j];
                k = j;
            } else {
                break;
            }
        }
        array[k] = key;
    }
}


pub fn insertion_sort2 <T: Ord + Copy> (array: &mut Vec<T>, asc: bool) {
    let op: Box<dyn Fn(T, T) -> bool> = match asc {
        true => Box::new(|x, y| {x > y}),
        false => Box::new(|x, y| {x < y}),
    };
    for i in 1..array.len() {
        let key = array[i];
        let mut k = i;
        for j in (0..i).rev() {
            if op(array[j], key) {
                array[j + 1] = array[j];
                k = j;
            } else {
                break;
            }
        }
        array[k] = key;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_i32() {
        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![1, 2, 3, 4, 5, 6];
        
        insertion_sort(&mut array);
        
        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_insertion_sort2_i32_asc() {
        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![1, 2, 3, 4, 5, 6];
        
        insertion_sort2(&mut array, true);
        
        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_insertion_sort2_i32_desc() {
        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![6, 5, 4, 3, 2, 1];
        
        insertion_sort2(&mut array, false);
        
        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_insertion_sort_chars() {
        let mut array = vec!['b', 'f', 'c', 'a', 'e', 'd'];
        let expected_array = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        
        insertion_sort(&mut array);
        
        assert_eq!(array, expected_array);
    }

    #[test]
    fn test_insertion_sort2_chars_asc() {
        let mut array = vec!['b', 'f', 'c', 'a', 'e', 'd'];
        let expected_array = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        
        insertion_sort2(&mut array, true);
        
        assert_eq!(array, expected_array);
    }
}
