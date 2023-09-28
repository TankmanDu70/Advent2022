#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <math.h>

struct number
{
    int position;
    int value;
};

struct number *nu;
uint nu_count = 1;

struct number *getNuByPosition(int pos);
struct number *getNuByPosition(int pos)
{
    for (size_t i = 0; i <= nu_count; i++)
    {
        if (nu[nu_count].position == i)
            return &nu[nu_count];
    }
}

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    char *in_char;
    char in_str[128];

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    in_char = fgets(in_str, 128, ptr_in);

    while (in_char != NULL)
    {
        int val = 0;
        if (sscanf(in_str, "%d\n", &val) == 1)
        {
            if (nu_count == 1)
                nu = malloc(sizeof(struct number));
            else
                nu = realloc(nu, sizeof(struct number) * nu_count);
            nu[nu_count - 1].position = nu_count;
            nu[nu_count - 1].value = val;

            in_char = fgets(in_str, 128, ptr_in);
            if (in_char != NULL)
                nu_count++;
        }
    }

    for (size_t i = 0; i < nu_count; i++)
    {
        printf("(%u:%d)\t", nu[i].position, nu[i].value);
    }
    printf("\n");

    for (size_t i = 0; i < nu_count; i++)
    {
        uint mt_position = nu[i].position;
        if (nu[i].value > 0)
        {
            for (size_t j = 0; j < abs(nu[i].value); j++)
            {
                if (++nu[i].position > nu_count)
                    nu[i].position = 1;
            }
        }
        else
        {
            for (size_t j = 0; j < abs(nu[i].value); j++)
            {
                if (--nu[i].position == 0)
                    nu[i].position = nu_count;
            }
        }

        for (size_t i = 0; i <= nu_count; i++)
        {
            for (size_t j = 0; j < nu_count; j++)
            {
                if (nu[j].position == i)
                    printf("(%u:%d)\t", nu[j].position, nu[j].value);
            }
        }
        printf("\n");
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
    printf("\n");

    for (size_t i = 0; i < nu_count; i++)
    {
        printf("(%u:%d)\t", nu[i].position, nu[i].value);
    }
}
