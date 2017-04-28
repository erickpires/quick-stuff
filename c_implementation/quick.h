#ifndef QUICK_H
#define QUICK_H 1

#include <stdint.h>
typedef int8_t int8;
typedef int16_t int16;
typedef int32_t int32;
typedef int64_t int64;

typedef unsigned int uint;
typedef uint8_t uint8;
typedef uint16_t uint16;
typedef uint32_t uint32;
typedef uint64_t uint64;

typedef size_t usize;

typedef int bool;
#define FALSE 0
#define TRUE  1


usize quickpartion_double(double*, usize, usize);
usize quickpartion_usize(usize*, usize, usize);
void quickselect_usize(usize*, usize, usize);
void quickselect_double(double*, usize, usize);
void quickselect_multiple_usize(usize*, usize, usize*, usize);
void _quickselect_multiple_usize(usize*, usize, usize*, usize, usize);
void quickselect_multiple_double(double*, usize, usize*, usize);
void _quickselect_multiple_double(double*, usize, usize*, usize, usize);
void quicksort_double(double*, usize);
void quicksort_usize(usize*, usize);
void swap_double(double*, usize, usize);
void swap_usize(usize*, usize, usize);
usize binary_search(usize*, usize, double);

#if RELEASE_BUILD
#define assert(n) {}
#else
#define assert(n) if(!(n)) { \
        printf("Failed: " #n " ~~~ at line: %d\n", __LINE__);    \
        *((int*)0) = 42; \
    }
#endif

#endif
