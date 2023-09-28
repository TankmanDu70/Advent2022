#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

#define ms *1000

bool isWrapped(int *ranges);
int get_range_pairs(FILE *ptr_in, int *ranges);

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");
    int ranges[4] = {0, 0, 0, 0};
    int ret = 1;
    int wrap_count = 0;
    while (ret > 0)
    {
        ret = get_range_pairs(ptr_in, ranges);
        if (ret > 0)
        {
            printf("1st - from %d to %d - 2nd - from %d to %d ret=%d\t", ranges[0], ranges[1], ranges[2], ranges[3], ret);

            if (isWrapped(ranges))
            {
                wrap_count++;
                printf("WRAPPED %d times!\n", wrap_count);
            }
            else
                printf("\n");
        }
        //usleep(10 ms);
    }
}

int get_range_pairs(FILE *ptr_in, int *ranges)
{
    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return 0;
    }
    char in_char;
    int test = fscanf(ptr_in, "%u-%u,%u-%u\n", ranges, ranges + 1, ranges + 2, ranges + 3);
    return test;
}

bool isWrapped(int *ranges)
{
    return (ranges[0] <= ranges[2]) && (ranges[1] >= ranges[3]) || (ranges[2] <= ranges[0]) && (ranges[3] >= ranges[1]);
}
