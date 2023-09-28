#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <math.h>

int *number;
uint nu_count = 1;

void parseFile(char *in_char, char *in_str, FILE *ptr_in);
void printList(void);


void printList(void)
{
    for (size_t i = 0; i < nu_count; i++)
    {
        printf("(%u:%d)\t", i + 1, number[i]);
    }
    printf("\n");
}

void parseFile(char *in_char, char *in_str, FILE *ptr_in)
{
    in_char = fgets(in_str, 128, ptr_in);

    while (in_char != NULL)
    {
        int val = 0;
        if (sscanf(in_str, "%d\n", &val) == 1)
        {
            if (nu_count == 1)
                number = malloc(sizeof(int));
            else
                number = realloc(number, sizeof(int) * nu_count);
            number[nu_count - 1] = val;

            in_char = fgets(in_str, 128, ptr_in);
            if (in_char != NULL)
                nu_count++;
        }
    }
}

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen("./Readme", "r");

    char *in_char;
    char in_str[128];

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    parseFile(in_char, in_str, ptr_in);
    printList();

    for (size_t i = 0; i < nu_count; i++) // all numbers must be processed
    {
        uint mt_position = i;
        if (number[i] > 0)
        {
            for (size_t j = 0; j < abs(number[i]);)
            {
                if (++j > nu_count)
                    j = 1;
                int buff = number[i + j + 1];
                number[i + j + 1] = number[i + j];
                number[j] = buff;
                printList();
            }
        }
        else
        {
            for (size_t j = 0; j < abs(number[j]); j++)
            {
                if (--j < 1)
                    j = nu_count - 1;
                int buff = number[j];
                number[j] = number[j + 1];
                number[j + 1] = buff;
            }
        }

        // for (size_t j = nu[i].position; j < mt_position; j++)
        //{
        //     nu[j].position = (nu[j].position - 1) % nu_count;
        // }
        /* for (size_t j = 0; j < nu_count - 1; j++)
        {
            if()
        } */
        // printf("(%u/%u:%d)\t", nu[i].position,  nu[i].value);
    }

    printList();
}
