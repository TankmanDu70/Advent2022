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

    char buffer[4] = {'x', 'x', 'x', 'x'};
    int ptr = 0;
    int test = 1;

    int incr = 0;

    while (test > 0)
    {
        test = fscanf(ptr_in, "%c", buffer + ptr); // buffer + ptr);
        // printf("%d-%c\n", test, buffer[ptr]);
        if (++ptr > 3)
            ptr = 0;
        incr++;
        bool redundant = false;
        printf("\n%3d\t%c%c%c%c", incr, buffer[0], buffer[1], buffer[2], buffer[3]);
        if (incr > 3)
        {
            redundant = false;
            for (size_t i = 0; i < 3; i++)
            {
                if (redundant)
                    break;
                for (size_t j = i + 1; j < 4; j++)
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
