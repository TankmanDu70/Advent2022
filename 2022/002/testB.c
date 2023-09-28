#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

int confrontA(char opponent);
int confrontB(char opponent);
int confrontC(char opponent);

int me_scr = 0;
int en_scr = 0;

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    char en_cr = '0';
    char me_cr = '0';

    int test = fscanf(ptr_in, "%c ", &en_cr);
    test = fscanf(ptr_in, "%c\n", &me_cr);

    while (test > 0)
    {
        int match = 0;
        switch (en_cr)
        {
        case 'A':
            match = confrontA(me_cr);
            break;
        case 'B':
            match = confrontB(me_cr);
            break;
        case 'C':
            match = confrontC(me_cr);
            break;
        default:
            break;
        }

        printf("him: %c me:%c ", en_cr, me_cr);

        switch (match)
        {
        case 1:
            printf("Win  ");
            me_scr += 6;
            break;
        case 0:
            printf("Draw ");
            me_scr += 3;
            break;
        case -1:
            printf("Lose ");
            break;
        }
        printf("en->%3d, me->%3d\n", en_scr, me_scr);
        test = fscanf(ptr_in, "%c\n", &en_cr);
        test = fscanf(ptr_in, "%c ", &me_cr);
    }
}

int confrontA(char opponent)
{
    // me_scr += 1;
    switch (opponent)
    {
    case 'X': // need to loose
        me_scr += 3;
        return -1;
    case 'Y': // draw
        me_scr += 1;
        return 0;
    case 'Z': // win
        me_scr += 2;
        return 1;
    }
}

int confrontB(char opponent)
{
    // me_scr += 2;
    switch (opponent)
    {
    case 'X':
        me_scr += 1;
        return -1;
    case 'Y':
        me_scr += 2;
        return 0;
    case 'Z':
        me_scr += 3;
        return 1;
    }
}

int confrontC(char opponent)
{
    // me_scr += 3;
    switch (opponent)
    {
    case 'X':
        me_scr += 2;
        return -1;
    case 'Y':
        me_scr += 3;
        return 0;
    case 'Z':
        me_scr += 1;
        return 1;
    }
}
