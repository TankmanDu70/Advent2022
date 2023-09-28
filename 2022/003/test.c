#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

int line_count = 0;
int char_count = 0;
int max_char_count = 0;

char in_char = ' ';
char *char_buffer;
int prio_sum = 0;

void main(int argc, char *argv[])
{
    FILE *file_ptr = fopen(argv[1], "r");

    char alphabet[] = {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'};
    for (size_t i = 0; i < sizeof(alphabet); i++)
        printf("%c=%d ", alphabet[i], alphabet[i]);
    printf("\n");
    if (file_ptr == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    int test = fscanf(file_ptr, "%c", &in_char);
    char_buffer = malloc(sizeof(char));

    while (test > 0) // end of file
    {
        while ((in_char != '\n') && (test > 0))
        {
            char_count++;
            // printf(" %d ", char_count);
            if (char_count > max_char_count)
            {
                // printf("\nmx %d cc %d\n", max_char_count, char_count);
                max_char_count = char_count;
                char_buffer = realloc(char_buffer, max_char_count * sizeof(char));
            }
            if (char_buffer == NULL)
                printf("\naaaaa\n");
            char_buffer[char_count - 1] = in_char;
            // printf("%c", in_char);
            test = fscanf(file_ptr, "%c", &in_char);
        }
        line_count++;
        // char_buffer[char_count] = '\0';
        // printf("\tline %d, char count %d\n", line_count, char_count);

        if (char_count > 0)
        {
            size_t char_count_hlf = char_count >> 1;

            char *first = malloc(char_count_hlf + 1);
            char *second = malloc(char_count_hlf + 1);

            memcpy(first, char_buffer, char_count_hlf);
            memcpy(second, char_buffer + char_count_hlf, char_count_hlf);

            first[char_count_hlf + 1] = '\0';
            second[char_count_hlf + 1] = '\0';

            // printf("1- %s \t 2- %s", first, second);
            for (size_t i = 0; i < char_count_hlf; i++)
            {
                for (size_t j = 0; j < char_count_hlf; j++)
                {
                    if (first[i] == second[j])
                    {
                        int prio = first[i] >= 'a' ? first[i] - 'a' + 1 : first[i] - 'A' + 'z' - 'a' + 2;
                        prio_sum += prio;
                        printf("dupplicate = %c prio %2d \t sum = %d\n", first[i], prio, prio_sum);
                        i = char_count_hlf;
                        j = char_count_hlf;
                    }
                }
            }
        }
        // printf("\nline buffer = %s ", char_buffer);
        char_count = 0;
        test = fscanf(file_ptr, "%c", &in_char);
        // printf("   test %d cc %d \n", test, char_count);
        //sleep(1);
    }

    free(char_buffer);
}
