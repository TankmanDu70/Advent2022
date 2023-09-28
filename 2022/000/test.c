#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    char in_char;

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    
    int test = fscanf(ptr_in, "%c ", &in_char);
    
    
    while (test > 0)
    {
       
    }
}
