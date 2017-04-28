extern crate rand;

use std::ops::FnMut;

use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::cmp::Ordering;

pub fn quickpartion<T>(slice: &mut [T], pivot_index: usize) -> usize
    where T: PartialOrd + Copy {

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

pub fn quickpartion_by<T, F>(slice: &mut [T], pivot_index: usize,
                             compare: &mut F) -> usize
    where T: Copy, F: FnMut(&T, &T) -> Ordering {

    if slice.len() == 1 { return 0; }

    swap(slice, 0, pivot_index);

    let mut lo = 1;
    let mut hi = slice.len() - 1;

    'outter: loop {
        while compare(&slice[0], &slice[lo]) == Ordering::Greater {
            if lo >= hi { break 'outter; }
            lo += 1;
        }

        while compare(&slice[0], &slice[hi]) == Ordering::Less {
            hi -= 1;
        }

        if lo >= hi { lo -= 1; break; }

        swap(slice, lo, hi);
        lo += 1;
        hi -= 1;
    }

    let result = lo;
    swap(slice, 0, result);

    result
}

pub fn quickselect<T>(slice: &mut [T], nth: usize)
    where T: Ord + Copy {
    quickselect_partial_ord(slice, nth);
}

pub fn quickselect_partial_ord<T>(slice: &mut [T], nth: usize)
    where T: PartialOrd + Copy {

    let pivot_index = rand::random::<usize>() % slice.len();

    let pivot_position = quickpartion(slice, pivot_index);
    if pivot_position == nth { return; }

    if nth < pivot_position {
        quickselect_partial_ord(&mut slice[0 .. pivot_position], nth);
    } else {
        quickselect_partial_ord(&mut slice[pivot_position + 1 ..],
                                nth - (pivot_position + 1));
    }
}

pub fn quickselect_by<T, F>(slice: &mut [T], nth: usize, mut compare: F)
    where T: Copy, F: FnMut(&T, &T) -> Ordering {
    quickselect_ref_by(slice, nth, &mut compare)
}

pub fn quickselect_ref_by<T, F>(slice: &mut [T], nth: usize, compare: &mut F)
    where T: Copy, F: FnMut(&T, &T) -> Ordering {

    let pivot_index = rand::random::<usize>() % slice.len();

    let pivot_position = quickpartion_by(slice, pivot_index, compare);
    if pivot_position == nth { return; }

    if nth < pivot_position {
        quickselect_ref_by(&mut slice[0 .. pivot_position], nth, compare);
    } else {
        quickselect_ref_by(&mut slice[pivot_position + 1 ..],
                                nth - (pivot_position + 1),
                                compare);
    }
}

pub fn quickselect_multiple_partial_ord<T>(slice: &mut [T], nths: &[usize])
    where T: PartialOrd + Copy {
    return _quickselect_multiple(slice, nths, 0);
}

pub fn quickselect_multiple<T>(slice: &mut [T], nths: &[usize]) where T: Ord + Copy {
    return _quickselect_multiple(slice, nths, 0);
}

fn _quickselect_multiple<T>(slice: &mut [T], nths: &[usize], mut accum: usize)
    where T: PartialOrd + Copy {
    assert!(nths.len() > 0);

    if nths.len() == 1 {
        return quickselect_partial_ord(slice, nths[0] - accum);
    }

    let pivot_index = rand::random::<usize>() % slice.len();

    let pivot_position = quickpartion(slice, pivot_index);
    let pivot_nths_index = binary_search(nths, pivot_position + accum);

    /* Left */
    {
        let left_nths = & nths[0 .. pivot_nths_index];
        if left_nths.len() > 0 {
            let left_slice = &mut slice[0 .. pivot_position];
            _quickselect_multiple(left_slice, left_nths, accum);
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
            _quickselect_multiple(right_slice, right_nths, accum);
        }
    }
}

#[allow(dead_code)]
pub fn quickselect_multiple_ref_by<T, F>(slice: &mut [T], nths: &[usize],
                                         compare: &mut F)
    where T: Copy, F: FnMut(&T, &T) -> Ordering {
    return _quickselect_multiple_by(slice, nths, 0, compare);
}

pub fn quickselect_multiple_by<T, F>(slice: &mut [T], nths: &[usize], mut compare: F)
    where T: Copy, F: FnMut(&T, &T) -> Ordering {
    return _quickselect_multiple_by(slice, nths, 0, &mut compare);
}

fn _quickselect_multiple_by<T, F>(slice: &mut [T], nths: &[usize], mut accum: usize,
                               compare: &mut F)
    where T: Copy, F: FnMut(&T, &T) -> Ordering {

    assert!(nths.len() > 0);

    if nths.len() == 1 {
        return quickselect_ref_by(slice, nths[0] - accum, compare);
    }

    let pivot_index = rand::random::<usize>() % slice.len();

    let pivot_position = quickpartion_by(slice, pivot_index, compare);
    let pivot_nths_index = binary_search(nths, pivot_position + accum);

    /* Left */
    {
        let left_nths = & nths[0 .. pivot_nths_index];
        if left_nths.len() > 0 {
            let left_slice = &mut slice[0 .. pivot_position];
            _quickselect_multiple_by(left_slice, left_nths, accum, compare);
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
            _quickselect_multiple_by(right_slice, right_nths, accum, compare);
        }
    }
}


pub fn quicksort<T>(slice: &mut [T]) where T: Ord + Copy {
    quicksort_partial_ord(slice)
}

pub fn quicksort_partial_ord<T>(slice: &mut [T]) where T: PartialOrd + Copy {
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
        quicksort_partial_ord(left);
    }

    {
        let right = &mut slice[pivot_position + 1 ..];
        quicksort_partial_ord(right);
    }
}

pub fn quicksort_by<T, F>(slice: &mut [T], mut compare: F)
    where T: Copy, F: FnMut(&T, &T) -> Ordering {
    quicksort_ref_by(slice, &mut compare)
}

fn quicksort_ref_by<T, F>(slice: &mut [T], compare: &mut F)
      where T: Copy, F: FnMut(&T, &T) -> Ordering {
    if slice.len() < 2 {
        return;
    }

    if slice.len() == 2 {
        if compare(&slice[0], &slice[1]) == Ordering::Greater {
            swap(slice, 0, 1);
        }

        return;
    }

    let pivot_index = rand::random::<usize>() % slice.len();
    let pivot_position = quickpartion_by(slice, pivot_index, compare);

    {
        let left  = &mut slice[0 .. pivot_position];
        quicksort_ref_by(left, compare);
    }

    {
        let right = &mut slice[pivot_position + 1 ..];
        quicksort_ref_by(right, compare);
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
