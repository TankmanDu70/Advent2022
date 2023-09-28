#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    int test = 1;

    unsigned int elf_count = 0;
    unsigned int *elf = malloc(sizeof(unsigned int));
    printf("allocated %ld \n", sizeof(elf));

    unsigned int nr_1_fattest = 0;
    unsigned int nr_2_fattest = 0;
    unsigned int nr_3_fattest = 0;

    char in_str[12];
    char in_cr;
    int te;

    unsigned int fatestElfCalories = 0;
    unsigned int currentElfCalories = 0;

    memset(in_str, 0, 12);

    while (test > 0)
    {
        unsigned int cal = 0;
        // test = fscanf(ptr_in, "%d", &cal);
        size_t dec_count = 0;
        bool is_eof = false;

        while (!is_eof)
        {
            test = fscanf(ptr_in, "%c", &in_cr);
            if (test != 1)
            {
                // printf("elf = %d - calories  %d - total %d - decCe %ld\n", elf_count, 0, elf[elf_count], dec_count);
                break;
            }
            // printf("%c", in_cr);
            if (in_cr == '\n')
            {
                is_eof = true;
                if (dec_count == 0)
                {
                    //  printf("elf = %d - calories  %d - total %d - decCe %ld\n", elf_count, 0, elf[elf_count], dec_count);
                    elf_count++;
                    if (currentElfCalories > fatestElfCalories)
                        fatestElfCalories = currentElfCalories;
                    currentElfCalories = 0;
                    // elf = realloc(elf, elf_count * sizeof(unsigned int));
                    // elf[elf_count] = 0;
                    // printf("reallocated %ld\n", sizeof(elf));
                }
                in_str[dec_count] = '\0';
                dec_count = 0;
            }
            else
            {
                in_str[dec_count] = in_cr;
                dec_count++;
            }
            //  sleep(1);
            // printf("  %ld\n", dec_count);
        }
        // printf("%s \n", in_str);
        unsigned int t = atoi(in_str);
        // printf("elf = %d - calories  %d - total %d \n", elf_count, t, currentElfCalories); // elf[elf_count]);
        // sleep(1);
        currentElfCalories += t;
        printf("current %d fattest %d\n", currentElfCalories, fatestElfCalories);
        // elf[elf_count] += t;
    }
    fclose(ptr_in);
}