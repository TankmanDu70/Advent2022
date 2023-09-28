#include <stdio.h>

void main()
{
    FILE *ptr = fopen("/home/thomas/Documents/Advent/2021/002", "r");

    if (ptr == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    int num = 0;
    int prev_num = 0;
    int test = fscanf(ptr, "%d\n", &num);
    int iterator = 0;
    //printf("scan: %d ", test);
    while (test == 1)
    {
        //printf("read %d \n", num);
        prev_num = num;
        test = fscanf(ptr, "%d\n", &num);
        if (num > prev_num)
            iterator++;
    }
    printf("increased %d times", iterator);
    fclose(ptr);
}