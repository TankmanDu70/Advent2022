#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");
    printf("DAY006\n");
    char in_char;

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    char buffer[14];
    memset(buffer, 'x', 14);
    int ptr = 0;
    int test = 1;

    int incr = 0;

    while (test > 0)
    {
        test = fscanf(ptr_in, "%c", buffer + ptr); // buffer + ptr);
        // printf("%d-%c\n", test, buffer[ptr]);
        if (++ptr > 13)
            ptr = 0;
        incr++;
        bool redundant = false;
        printf("\n%3d\t%s", incr, buffer);
        if (incr > 13)
        {
            redundant = false;
            for (size_t i = 0; i < 13; i++)
            {
                if (redundant)
                    break;
                for (size_t j = i + 1; j < 14; j++)
                {
                    if (redundant)
                        break;
                    if (buffer[i] == buffer[j])
                    {
                        redundant = true;
                        printf("--X %d %d", i, j);
                        break;
                    }
                    /* code */
                }
                /* code */
            }

            if (!redundant)
            {
                printf("\n- start at index %d \n", incr);
                return;
                // incr = 0;
            }
        }
    }
}
