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

    int elf_counter = 0;
    int elf_pack[3] = {0, 0, 0};
    int elf_counter_temp = 0;
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
        elf_pack[elf_counter] = char_count - elf_counter_temp; // - (elf_counter > 0 ? elf_pack[elf_counter - 1] : 0);
        elf_counter_temp = char_count;
        // printf("\nelfpack[%d]%d\n", elf_counter, char_count);
        //  char_count = 0;
        //   printf("\n");
        line_count++;
        elf_counter++;
        // char_buffer[char_count] = '\0';
        // printf("\tline %d, char count %d\n", line_count, char_count);

        if ((elf_counter == 3))
        {
            printf("\nNew grp: \n%s\ncounts\n%d\n%d\n%d\n", char_buffer, elf_pack[0], elf_pack[1], elf_pack[2]);
            char *first = malloc(elf_pack[0] + 1);
            char *second = malloc(elf_pack[1] + 1);
            char *third = malloc(elf_pack[2] + 1);

            printf("allocated\n");

            memset(first, 0, elf_pack[0] + 1);
            memset(second, 0, elf_pack[1] + 1);
            memset(third, 0, elf_pack[2] + 1);

            printf("set\n");

            memcpy(first, char_buffer, elf_pack[0]);
            memcpy(second, char_buffer + elf_pack[0], elf_pack[1]);
            memcpy(third, char_buffer + elf_pack[1] + elf_pack[0], elf_pack[2]);

            printf("copied\n");

            first[elf_pack[0]] = '\0';
            second[elf_pack[1]] = '\0';
            third[elf_pack[2]] = '\0';
            // printf("1- %s\n2- %s\n3- %s\n", first, second, third);
            printf("1- %s %d\n2- %s %d\n3- %s %d\n", first, elf_pack[0], second, elf_pack[1], third, elf_pack[2]);
            for (size_t i = 0; i < elf_pack[0]; i++)
            {
                for (size_t j = 0; j < elf_pack[1]; j++)
                {
                    for (size_t k = 0; k < elf_pack[2]; k++)
                    {
                        if ((first[i] == second[j]) && (second[j] == third[k]))
                        {
                            int prio = first[i] >= 'a' ? first[i] - 'a' + 1 : first[i] - 'A' + 'z' - 'a' + 2;
                            prio_sum += prio;
                            printf("\ndupplicate = %c prio %2d \t sum = %d\n", first[i], prio, prio_sum);
                            i = elf_pack[0];
                            j = elf_pack[1];
                            k = elf_pack[2];
                        }
                    }
                }
            }
            elf_counter = 0;
            char_count = 0;
            free(first);
            free(second);
            free(third);
            memset(char_buffer, 0, max_char_count);
            elf_counter_temp = 0;
        }
        //usleep(100000);
        // printf("\nline buffer = %s ", char_buffer);

        test = fscanf(file_ptr, "%c", &in_char);
        // printf("   test %d cc %d \n", test, char_count);
        // sleep(1);
    }

    free(char_buffer);
}
