/// Quick sorting algorithm, O(n^2)
///
/// Input: <a1, a2, ..., an>.
/// Output: <a1', a2', ..., an'>, where a1' <= a2' <= ... <= an'.
///

fn partition <T> (array: &mut Vec<T>, left: usize, right: usize) -> usize
where
    T: Ord + Copy
{
    let pivot = array[right];
    let mut store = left;
    for j in left..right {
        if array[j] <= pivot {
            let tmp = array[j];
            array[j] = array[store];
            array[store] = tmp;
            store += 1;
        }
    }
    let tmp = array[right];
    array[right] = array[store];
    array[store] = tmp;
    
    store
}

fn qsort <T: Ord + Copy> (array: &mut Vec<T>, left: usize, right: usize) {
    if left < right {
        let pivot_idx = partition(array, left, right);
        if pivot_idx != 0 {
            qsort(array, left, pivot_idx-1);
        }
        qsort(array, pivot_idx+1, right);
    }
}


pub fn quick_sort <T: Ord + Copy> (array: &mut Vec<T>) {
    qsort(array, 0, array.len()-1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![1, 2, 3, 4, 5, 6];

        quick_sort(&mut array);

        assert_eq!(array, expected_array);
    }

}
