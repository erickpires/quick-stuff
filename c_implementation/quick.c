#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "quick.h"

usize quickpartion_usize(usize* array, usize array_size, usize pivot_index) {

    if (array_size == 1) { return 0; }

    swap_usize(array, 0, pivot_index);

    usize lo = 1;
    usize hi = array_size - 1;

    bool done = FALSE;
    while(TRUE) {
        while (array[0] > array[lo]) {
            if (lo >= hi) { done = TRUE; break; }
            lo += 1;
        }

        if (done) { break; }

        while (array[0] < array[hi]) { hi -= 1; }

        if (lo >= hi) { lo -= 1; break; }

        swap_usize(array, lo, hi);
        lo += 1;
        hi -= 1;
    }

    usize result = lo;
    swap_usize(array, 0, result);

    return result;
}

usize quickpartion_double(double* array, usize array_size, usize pivot_index) {
    if (array_size == 1) { return 0; }

    swap_double(array, 0, pivot_index);

    usize lo = 1;
    usize hi = array_size - 1;

    bool done = FALSE;
    while(TRUE) {
        while (array[0] > array[lo]) {
            if (lo >= hi) { done = TRUE; break; }
            lo += 1;
        }

        if (done) { break; }

        while (array[0] < array[hi]) { hi -= 1; }

        if (lo >= hi) { lo -= 1; break; }

        swap_double(array, lo, hi);
        lo += 1;
        hi -= 1;
    }

    usize result = lo;
    swap_double(array, 0, result);

    return result;
}

void quickselect_usize(usize* array, usize array_size, usize nth) {
    usize pivot_index = rand() % array_size;

    usize pivot_position = quickpartion_usize(array, array_size, pivot_index);
    if (pivot_position == nth) { return; }

    if (nth < pivot_position) {
        quickselect_usize(array, pivot_position, nth);
    } else {
        usize new_size = array_size - (pivot_position + 1);
        usize new_nth  = nth - (pivot_position + 1);
        usize* new_array = array + (pivot_position + 1);
        quickselect_usize(new_array, new_size, new_nth);
    }
}

void quickselect_double(double* array, usize array_size, usize nth) {
    usize pivot_index = rand() % array_size;

    usize pivot_position = quickpartion_double(array, array_size, pivot_index);
    if (pivot_position == nth) { return; }

    if (nth < pivot_position) {
        quickselect_double(array, pivot_position, nth);
    } else {
        usize new_size = array_size - (pivot_position + 1);
        usize new_nth  = nth - (pivot_position + 1);
        double* new_array = array + (pivot_position + 1);
        quickselect_double(new_array, new_size, new_nth);
    }
}

void quickselect_multiple_usize(usize* array, usize array_size,
                          usize* nths, usize nths_size) {
    return _quickselect_multiple_usize(array, array_size, nths, nths_size, 0);
}

void _quickselect_multiple_usize(usize* array, usize array_size,
                            usize* nths, usize nths_size, usize accum) {
    assert(nths_size > 0);

    if (nths_size == 1) {
        return quickselect_usize(array, array_size, nths[0] - accum);
    }

    usize pivot_index = rand() % array_size;

    usize pivot_position = quickpartion_usize(array, array_size, pivot_index);
    usize pivot_nths_index = binary_search(nths, nths_size, pivot_position + accum);

    /* Left */
    usize left_nths_size = pivot_nths_index;
    if (left_nths_size > 0) {
        usize left_array_size = pivot_position;
        _quickselect_multiple_usize(array, left_array_size,
                                     nths, left_nths_size, accum);
    }


    /* Right */

    if (pivot_nths_index == nths_size) { return; }

    usize* right_nths;
    usize right_nths_size;

    if (nths[pivot_nths_index] == pivot_position + accum) {
        right_nths = nths + pivot_nths_index + 1;
        right_nths_size = nths_size - (pivot_nths_index + 1);
    } else {
        right_nths = nths + pivot_nths_index;
        right_nths_size = nths_size - (pivot_nths_index);
    }

    if (right_nths_size > 0) {
        accum += pivot_position + 1;
        usize* right_array = array + pivot_position + 1;
        usize right_array_size = array_size - (pivot_position + 1);
        _quickselect_multiple_usize(right_array, right_array_size,
                              right_nths, right_nths_size, accum);
    }
}

void quickselect_multiple_double(double* array, usize array_size,
                          usize* nths, usize nths_size) {
    return _quickselect_multiple_double(array, array_size, nths, nths_size, 0);
}

void _quickselect_multiple_double(double* array, usize array_size,
                            usize* nths, usize nths_size, usize accum) {
    assert(nths_size > 0);

    if (nths_size == 1) {
        return quickselect_double(array, array_size, nths[0] - accum);
    }

    usize pivot_index = rand() % array_size;

    usize pivot_position = quickpartion_double(array, array_size, pivot_index);
    usize pivot_nths_index = binary_search(nths, nths_size, pivot_position + accum);

    /* Left */
    usize left_nths_size = pivot_nths_index;
    if (left_nths_size > 0) {
        usize left_array_size = pivot_position;
        _quickselect_multiple_double(array, left_array_size,
                                     nths, left_nths_size, accum);
    }


    /* Right */

    if (pivot_nths_index == nths_size) { return; }

    usize* right_nths;
    usize right_nths_size;

    if (nths[pivot_nths_index] == pivot_position + accum) {
        right_nths = nths + pivot_nths_index + 1;
        right_nths_size = nths_size - (pivot_nths_index + 1);
    } else {
        right_nths = nths + pivot_nths_index;
        right_nths_size = nths_size - (pivot_nths_index);
    }

    if (right_nths_size > 0) {
        accum += pivot_position + 1;
        double* right_array = array + pivot_position + 1;
        usize right_array_size = array_size - (pivot_position + 1);
        _quickselect_multiple_double(right_array, right_array_size,
                              right_nths, right_nths_size, accum);
    }
}

void quicksort_usize(usize* array, usize array_size) {
    if (array_size < 2) {
        return;
    }

    if (array_size == 2) {
        if (array[0] > array[1]) {
            swap_usize(array, 0, 1);
        }
        return;
    }

    usize pivot_index = rand() % array_size;
    usize pivot_position = quickpartion_usize(array, array_size, pivot_index);


    usize left_size = pivot_position;
    quicksort_usize(array, left_size);

    usize* right = array + pivot_position + 1;
    usize right_size = array_size - (pivot_position + 1);
    quicksort_usize(right, right_size);
}

void quicksort_double(double* array, usize array_size) {
    if (array_size < 2) {
        return;
    }

    if (array_size == 2) {
        if (array[0] > array[1]) {
            swap_double(array, 0, 1);
        }
        return;
    }

    usize pivot_index = rand() % array_size;
    usize pivot_position = quickpartion_double(array, array_size, pivot_index);


    usize left_size = pivot_position;
    quicksort_double(array, left_size);

    double* right = array + pivot_position + 1;
    usize right_size = array_size - (pivot_position + 1);
    quicksort_double(right, right_size);
}

void swap_double(double* array, usize i0, usize i1) {
    double tmp = array[i0];
    array[i0] = array[i1];
    array[i1] = tmp;
}

void swap_usize(usize* array, usize i0, usize i1) {
    usize tmp = array[i0];
    array[i0] = array[i1];
    array[i1] = tmp;
}

usize binary_search(usize* array, usize array_size, double elem) {
    if (array_size == 0) { return 0; }

    usize mid_index = array_size / 2;
    double mid_elem  = array[mid_index];

    if (mid_elem == elem) { return mid_index; }
    if (mid_elem > elem) {
        return binary_search(array, mid_index , elem);
    } else {
        usize* right = array + (mid_index + 1);
        usize right_size = array_size - (mid_index + 1);
        return mid_index + 1 + binary_search(right, right_size, elem);
    }
}
