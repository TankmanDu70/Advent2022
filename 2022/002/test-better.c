#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

typedef enum
{
    rock = 1,
    paper = 2,
    scissors = 3
} choice;

typedef enum
{
    loose = -1,
    draw = 0,
    win = 1,
} game;

char rock_name[] = "rock";
char paper_name[] = "paper";
char scissors_name[] = "scissors";
char *toName[3] = {rock_name, paper_name, scissors_name};

int confront(choice his, choice mine);

//  I picked  rock  paper  scissors
int confrontLUT[3][3] = {{0, 1, -1},  // he picked rock
                         {-1, 0, 1},  // he picked paper
                         {1, -1, 0}}; // he picked scissors

int my_scr = 0;
int his_scr = 0;

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    char his = '0';
    char mine = '0';

    int test = fscanf(ptr_in, "%c %c\n", &his, &mine);
    // his -= 'A';
    // mine -= 'Y';

    while (test > 0)
    {
        // printf("him: %c me:%c \n", his, mine);
        his -= 'A';
        mine -= 'X';
        int match = confront(his, mine);
        // printf("him: %u me:%u \n", his, mine);
        printf("him: %s me:%s \t", toName[his], toName[mine]);

        switch (match)
        {
        case win:
            printf("Win  ");
            my_scr += 6;
            break;
        case draw:
            printf("Draw ");
            my_scr += 3;
            break;
        case loose:
            printf("Lose ");
            break;
        }
        printf("score en->%3d, me->%3d\n", his_scr, my_scr);
        test = fscanf(ptr_in, "%c %c\n", &his, &mine);
    }
}

int confront(choice his, choice mine)
{
    my_scr += mine + 1;
    return confrontLUT[his][mine];
}