#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

unsigned int nr_1_fattest = 0;
unsigned int nr_2_fattest = 0;
unsigned int nr_3_fattest = 0;

void sortFatElves(unsigned int currentFat);

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
    unsigned int *elf = malloc(sizeof(unsigned int) * 17);
    printf("allocated %ld \n", sizeof(elf));

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
                sortFatElves(currentElfCalories); // printf("elf = %d - calories  %d - total %d - decCe %ld\n", elf_count, 0, elf[elf_count], dec_count);
                printf("current %d nr1 %d nr2 %d nr3 %d\n", currentElfCalories, nr_1_fattest, nr_2_fattest, nr_3_fattest);

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
                    sortFatElves(currentElfCalories);
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
        if (test != 1)
            sortFatElves(currentElfCalories);
        printf("total %d nr1 %d nr2 %d nr3 %d\n", nr_1_fattest + nr_2_fattest + nr_3_fattest, nr_1_fattest, nr_2_fattest, nr_3_fattest);
        // elf[elf_count] += t;
    }
    fclose(ptr_in);
}

void sortFatElves(unsigned int currentFat)
{
    if (currentFat > nr_1_fattest)
    {
        nr_3_fattest = nr_2_fattest;
        nr_2_fattest = nr_1_fattest;
        nr_1_fattest = currentFat;
    }
    else if (currentFat > nr_2_fattest)
    {
        nr_3_fattest = nr_2_fattest;
        nr_2_fattest = currentFat;
    }
    else if (currentFat > nr_3_fattest)
        nr_3_fattest = currentFat;
}