/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T: Copy + std::cmp::PartialOrd>(array: &mut [T]){
	//TODO
    if array.len() <= 1 {
        return;
    }
    let mut tail = array.len() / 2 - 1;
    while tail >= 0 {
        downAdjust(array, tail, array.len());
        if(tail == 0) {
            break;
        }
        tail -= 1;
    }
    
    for i in 1..(array.len() - 1) {
        (array[0], array[array.len() - i]) = (array[array.len() - i], array[0]);
        downAdjust(array, 0, array.len() - i)
    }
    (array[0], array[1]) = (array[1], array[0]);
}

fn downAdjust<T: Copy + std::cmp::PartialOrd>(array: &mut[T], mut parent: usize, len: usize) {
    let mut child = parent * 2 + 1;
    while child < len {
        if child + 1 < len && array[child+1] > array[child] {
            child += 1;
        }
        if array[child] <= array[parent] {
            break;
        }
        (array[child], array[parent]) = (array[parent], array[child]);
        parent = child;
        child = parent * 2 + 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}