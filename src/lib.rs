extern crate rand;

use std::cmp::Ord;

pub fn quickpartion<T>(slice: &mut [T], pivot_index: usize) -> usize
    where T: Ord + Copy {

    if slice.len() == 1 { return 0; }

    swap(slice, 0, pivot_index);

    let mut lo = 1;
    let mut hi = slice.len() - 1;

    'outter: loop {
        while slice[0] > slice[lo] {
            if lo >= hi { break 'outter; }
            lo += 1;
        }

        while slice[0] < slice[hi] { hi -= 1; }

        if lo >= hi { lo -= 1; break; }

        swap(slice, lo, hi);
        lo += 1;
        hi -= 1;
    }

    let result = lo;
    swap(slice, 0, result);

    result
}

pub fn quickselect<T>(_slice: &mut [T], mut nth: usize) where T: Ord + Copy {
    let mut lo = 0;
    let mut hi = _slice.len();

    loop {
        let slice = &mut _slice[lo .. hi];
        let pivot_index = rand::random::<usize>() % slice.len();

        let pivot_position = quickpartion(slice, pivot_index);
        if pivot_position == nth { return; }

        if nth < pivot_position {
            hi = lo + pivot_position;
        } else {
            lo += pivot_position + 1;
            nth = nth - (pivot_position + 1);
        }
    }
}

pub fn quickselect_multiple<T>(slice: &mut [T], nths: &[usize], mut accum: usize)
    where T: Ord + Copy {
    assert!(nths.len() > 0);

    if nths.len() == 1 {
        return quickselect(slice, nths[0] - accum);
    }

    let pivot_index = rand::random::<usize>() % slice.len();

    let pivot_position = quickpartion(slice, pivot_index);
    let pivot_nths_index = binary_search(nths, pivot_position + accum);

    /* Left */
    {
        let left_nths = & nths[0 .. pivot_nths_index];
        if left_nths.len() > 0 {
            let left_slice = &mut slice[0 .. pivot_position];
            quickselect_multiple(left_slice, left_nths, accum);
        }
    }

    /* Right */
    {
        if pivot_nths_index == nths.len() { return; }

        let right_nths;
        if nths[pivot_nths_index] == pivot_position + accum {
            right_nths = & nths[pivot_nths_index + 1 ..];
        } else {
            right_nths = & nths[pivot_nths_index ..];
        }
        if right_nths.len() > 0 {
            accum += pivot_position + 1;
            let right_slice = &mut slice[pivot_position + 1 ..];
            quickselect_multiple(right_slice, right_nths, accum);
        }
    }
}

pub fn quicksort<T>(slice: &mut [T]) where T: Ord + Copy {
    if slice.len() < 2 {
        return;
    }

    if slice.len() == 2 {
        if slice[0] > slice[1] {
            swap(slice, 0, 1);
        }
        return;
    }

    let pivot_index = rand::random::<usize>() % slice.len();
    let pivot_position = quickpartion(slice, pivot_index);

    {
        let left  = &mut slice[0 .. pivot_position];
        quicksort(left);
    }

    {
        let right = &mut slice[pivot_position + 1 ..];
        quicksort(right);
    }
}


#[inline]
fn swap<T>(slice: &mut [T], i0: usize, i1: usize) where T: Copy {
    let tmp = slice[i0];
    slice[i0] = slice[i1];
    slice[i1] = tmp;
}

pub fn binary_search<T>(slice: &[T], elem: T) -> usize where T: Ord + Copy {
    if slice.len() == 0 { return 0; }

    let mid_index = slice.len() / 2;
    let mid_elem  = slice[mid_index];

    if mid_elem == elem { return mid_index; }
    if mid_elem > elem {
        return binary_search(&slice[0 .. mid_index] ,elem);
    } else {
        return mid_index + 1 + binary_search(&slice[mid_index + 1 ..], elem);
    }
}
