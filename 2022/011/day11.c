#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <stdint.h>
#include <math.h>
#include "team.h"

#define printEVENTS

char getPromptC(void);

void main(int argc, char *argv[])
{
    m_team team = {.round_count = 1,
                   .m_count = 0,
                   .checkIndex = 0};

    int m_count = parseFile("./Readme", &team);

    printTeam(&team);
    getPromptC();

    for (int ii = 0; ii < 10000; ii++)
    {
        teamCompute(&team);
    }

    teamPrintFinalScore(&team);
}

char getPromptC(void)
{
    char input;
    printf("Ready?");
    scanf("%c", &input);
    return input;
}
