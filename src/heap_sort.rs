/// Heap sorting algorithm, O(n log n)
///
/// Input: <a1, a2, ..., an>.
/// Output: <a1', a2', ..., an'>, where a1' <= a2' <= ... <= an'.
///

fn heapify <T: Ord + Copy> (array: &mut Vec<T>, idx: usize, max: usize) {
    let mut largest = idx;
    let left = idx * 2 + 1;
    let right = idx * 2 + 2;
    if left < max && array[left] > array[largest] {
        largest = left;
    }
    if right < max && array[right] > array[largest] {
        largest = right;
    }
    if largest != idx {
        let tmp = array[idx];
        array[idx] = array[largest];
        array[largest] = tmp;
        heapify(array, largest, max);
    }
}

fn build_heap <T: Ord + Copy> (array: &mut Vec<T>) {
    for i in (0..array.len()/2-1).rev() {
        heapify(array, i, array.len());
    }
}


pub fn heap_sort <T: Ord + Copy> (array: &mut Vec<T>) {
    build_heap(array);
    for i in (1..array.len()).rev() {
        let tmp = array[0];
        array[0] = array[i];
        array[i] = tmp;
        heapify(array, 0, i);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut array = vec![5, 2, 4, 6, 1, 3];
        let expected_array = vec![1, 2, 3, 4, 5, 6];

        heap_sort(&mut array);

        assert_eq!(array, expected_array);
    }

}
