#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

#define SL_MAX_CNT 4

typedef struct slider
{
    uint8_t count : 2;
    uint16_t sum;
    uint16_t prev_sum;
} slider_t;

uint16_t circ_buffer[3] = {0, 0, 0};
uint16_t total_count = 0;
uint16_t sum_counter = 0;

FILE *ptr_out;

void main()
{
    FILE *ptr = fopen("/home/thomas/Documents/Advent/2021/001", "r");
    FILE *ptr_out = fopen("/home/thomas/Documents/Advent/2021/002", "w");

    if (ptr == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    if (ptr_out == NULL)
    {
        printf("Error creating output !\n");
        return;
    }
    uint16_t num = 0;
    size_t looper = 0;
    // first read
    int cond = 1;
    size_t i = 0;

    while (cond == 1)
    {
        cond = fscanf(ptr, "%hd\n", &num);
        if (cond == 1)
        {
            total_count++;
            circ_buffer[i] = num;
            if (++i > 2)
                i = 0;
            if (total_count > 2)
            {
                fprintf(ptr_out, "%d\n", circ_buffer[0] + circ_buffer[1] + circ_buffer[2]);
                printf("%u=%d {%d,%d,%d} %d\n", total_count, num, circ_buffer[0], circ_buffer[1], circ_buffer[2], circ_buffer[0] + circ_buffer[1] + circ_buffer[2]);
                // cond = 10;
            }
        }
    }

    fclose(ptr);
}
