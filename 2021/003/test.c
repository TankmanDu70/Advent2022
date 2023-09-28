#include <stdio.h>
#include <string.h>
#include <stdint.h>

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen("/home/thomas/Documents/Advent/2021/003", "r");
    // FILE *ptr_out = fopen("/home/thomas/Documents/Advent/2021/003_out", "w");

    int depth = 0;
    int horiz = 0;

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    const char *strf = "%s%d\n";

    char in_dir[] = "froward";
    int in_cnt = 0;
    int test = 2;//fscanf(ptr_in, strf, &in_dir, &in_cnt);

    int time = 10;
    while (test == 2 && (time > 0))
    {
        // printf("read %d \n", num);
        test = fscanf(ptr_in, "%s%d", &in_dir, &in_cnt);
        if (test == 2)
        {
            //time--;
            if (strcmp(in_dir, "up") == 0)
            {
                depth += in_cnt;
            }
            if (strcmp(in_dir, "down") == 0)
            {
                depth -= in_cnt;
            }
            if (strcmp(in_dir, "forward") == 0)
            {
                horiz += in_cnt;
            }
        printf("test = %1d dir=%8s cnt=%2d, d=%2d, h=%2d\n",test, in_dir, in_cnt, depth, horiz);
        }
    }
    printf("final d=%3d h=%3d x=%10d\n", depth, horiz, depth*horiz);

    fclose(ptr_in);
}