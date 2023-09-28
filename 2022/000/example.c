#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

void main(int argc, char *argv[])
{
    char *in_char = {"ta mere 010010 oooo * 999\n"};
    char i, j;

    int test = fscanf(in_char, "ta mere %d oooo * %d\n", &i, &j);

}
