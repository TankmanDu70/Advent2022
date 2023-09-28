#include <stdio.h>
#include <string.h>
#include <stdint.h>

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen("/home/thomas/src/Advent/2021/004/004", "r");
    // FILE *ptr_out = fopen("/home/thomas/Documents/Advent/2021/003_out", "w");

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    const char *strf = "%c\n";
    int test = 1;
    int count = 0;

    int in_bin0[5] = {
        0,
        0,
        0,
        0,
        0,
    };

    int in_bin_ones_cnt[5] = {0,
                              0,
                              0,
                              0,
                              0};

    int gamma[5] = {0,
                    0,
                    0,
                    0,
                    0};
    int epsilon[5] = {0,
                      0,
                      0,
                      0,
                      0};
    while (test > 0)
    {
        // printf("read %d \n", num);
        //       test = fscanf(ptr_in, "%5[0-1]\n", in_bin0);

        test = fscanf(ptr_in, "%1d%1d%1d%1d%1d\n", in_bin0, in_bin0 + 1, in_bin0 + 2, in_bin0 + 3, in_bin0 + 4);
        if (test > 0)
        {
            count++;
            // printf("%1d %1d %1d %1d %1d\n", in_bin0[0], in_bin0[1], in_bin0[2], in_bin0[3], in_bin0[4]);
            for (size_t i = 0; i < sizeof(in_bin0) / sizeof(in_bin0[0]); i++)
            {
                in_bin_ones_cnt[i] += in_bin0[i];
            }
            printf("%3d, got %1d %1d %1d %1d %1d -> count %1d %1d %1d %1d %1d \n", count, in_bin0[0], in_bin0[1], in_bin0[2], in_bin0[3], in_bin0[4], in_bin_ones_cnt[0], in_bin_ones_cnt[1], in_bin_ones_cnt[2], in_bin_ones_cnt[3], in_bin_ones_cnt[4]);
        }
        else
        {
            for (size_t i = 0; i < sizeof(in_bin0) / sizeof(in_bin0[0]); i++)
            {
                if (in_bin_ones_cnt[i] > (count - in_bin_ones_cnt[i]))
                {
                    gamma[i] = 1;
                    epsilon[i] = 0;
                }
                else
                {
                    gamma[i] = 0;
                    epsilon[i] = 1;
                }
            }
            printf("gamma %d%d%d%d%d epsilon %d%d%d%d%d\n", gamma[0], gamma[1], gamma[2], gamma[3], gamma[4], epsilon[0], epsilon[1], epsilon[2], epsilon[3], epsilon[4]);
            int dec_gamma = toInt(gamma);
            int dec_eps = toInt(epsilon);
            printf("gamma %d, epsilon %d cons %d\n", dec_gamma, dec_eps, dec_eps * dec_gamma);
        }
    }
    fclose(ptr_in);
}

int toInt(int arr[]);
int toInt(int arr[])
{
    int val = 0;
    for (size_t i = 0; i < 5; i++)
    {
        printf("%d %d %d\n", i, val, arr[i]);
        val += arr[i] << 4 - i;
    }
    return val;
}