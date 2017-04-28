#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "quick.h"

#define N_ELEM (1000)
#define N_SEARCHES (4)
#define WINDOW_SIZE (N_ELEM / N_SEARCHES)
#define N_ITER (1000)

void shuffle_usize(usize*, usize);
void shuffle_double(double*, usize);
bool is_sorted_usize(usize*, usize);
bool is_sorted_double(double*, usize);
bool is_sorted_usize(usize*, usize);

typedef struct timespec time_spec;

int main(void) {
    srand(time(NULL));

    usize array[N_ELEM];
    usize nths[N_SEARCHES];

    for(int i = 0; i < N_ELEM; i++) {
        array[i] = i;
    }

    double time_sum = 0.0;

    for(int _ = 0; _ < N_ITER; _++) {

        shuffle_usize(array, N_ELEM);
        assert(!is_sorted_usize(array, N_ELEM));

        for(int i = 0; i < N_SEARCHES; i++) {
            nths[i] = i * WINDOW_SIZE + rand() % WINDOW_SIZE;
        }
        quicksort_usize(nths, N_SEARCHES);
        assert(is_sorted_usize(nths, N_SEARCHES));

        time_spec t0;
        time_spec t1;

        clock_gettime(CLOCK_MONOTONIC, &t0);
        quickselect_multiple_usize(array, N_ELEM, nths, N_SEARCHES);
        clock_gettime(CLOCK_MONOTONIC, &t1);

        for(int i = 0; i < N_SEARCHES; i++) {
            usize index = nths[i];
            assert(array[index] == index);
        }

        double micros0 = t0.tv_sec * 1000000.0 + t0.tv_nsec * 0.001;
        double micros1 = t1.tv_sec * 1000000.0 + t1.tv_nsec * 0.001;
        time_sum += (micros1 - micros0);
    }

    double time_mean = time_sum / N_ITER;
    printf("Took: %0.3lfus", time_mean);
}


void shuffle_usize(usize* array, usize array_size) {
    for(usize _ = 0; _ < array_size * 2; _++) {
        usize i0 = rand() % array_size;
        usize i1 = rand() % array_size;

        swap_usize(array, i0, i1);
    }
}

void shuffle_double(double* array, usize array_size) {
    for(usize _ = 0; _ < array_size * 2; _++) {
        usize i0 = rand() % array_size;
        usize i1 = rand() % array_size;

        swap_double(array, i0, i1);
    }
}

bool is_sorted_double(double* array, usize array_size) {
    for(int i = 0; i < array_size - 1; i++) {
        if(array[i] > array[i + 1]) { return FALSE; }
    }

    return TRUE;
}

bool is_sorted_usize(usize* array, usize array_size) {
    for(int i = 0; i < array_size - 1; i++) {
        if(array[i] > array[i + 1]) { return FALSE; }
    }

    return TRUE;
}
