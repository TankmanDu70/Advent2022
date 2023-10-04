#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdint.h>

#define EXIT_WITH_ERROR(...) \
    {                       \
        printf(__VA_ARGS__);   \
        exit(-1);           \
    }
