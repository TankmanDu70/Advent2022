#include "team.h"
#include "main.h"
#include <string.h>

int roundChecks[] = {1, 20, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000};

void printTeam(m_team *team)
{
    for (int ii = 0; ii < team->m_count; ii++)
    {
        monkeyPrint(team->monkeys[ii]);
    }
}

void printTeamBusiness(m_team *team)
{
    for (int ii = 0; ii < team->m_count; ii++)
    {
        monkeyBusyPrint(team->monkeys[ii]);
    }
}

void teamPrintFinalScore(m_team *team)
{
    long long unsigned int highest = 0;
    long long unsigned int secondHighest = 0;
    for (int ii = 0; ii < team->m_count; ii++)
    {
        if (team->monkeys[ii]->inspect_count > highest)
        {
            secondHighest = highest;
            highest = team->monkeys[ii]->inspect_count;
        }
        else if (team->monkeys[ii]->inspect_count > secondHighest)
        {
            secondHighest = team->monkeys[ii]->inspect_count;
        }
    }
    printf(" \r\nSCORES\t %llu \t %llu\r\n %llu\r\n",
           highest, secondHighest, (long long unsigned int)(secondHighest * highest));
}

void printTeamStash(m_team *team)
{
    for (int ii = 0; ii < team->m_count; ii++)
    {
        monkeyStashPrint(team->monkeys[ii]);
    }
}

int parseFile(const char *const path, m_team *team)
{
    FILE *file = fopen(path, "r");
    monkeyObj m = malloc(sizeof(struct monkey));
    int m_cnt = 0;
    bool test = true;
    while (test)
    {
        test = monkeyParse(file, &m[m_cnt]);
        monkeyCtor(&m[m_cnt]);
        m_cnt++;
        m = realloc(m, sizeof(struct monkey) * (m_cnt + 1));
    }

    for (int ii = 0; ii < m_cnt; ii++)
    {
        team->monkeys[ii] = &m[ii];
    }

    team->m_count = m_cnt;
    return m_cnt;
}

void teamCompute(m_team *team)
{
    for (int ii = 0; ii < team->m_count; ii++)
    {
        while (team->monkeys[ii]->stash.count)
        {
            if (monkeyCompute(team->monkeys[ii]))
            {
                monkeyPass(team->monkeys[ii], team->monkeys[team->monkeys[ii]->monKeyTrueIndex]);
            }
            else
            {
                monkeyPass(team->monkeys[ii], team->monkeys[team->monkeys[ii]->monKeyFalseIndex]);
            }
        }
    }
    if (team->round_count >= roundChecks[team->checkIndex])
    {
        printf(" \r\nROUND %5d\r\n", team->round_count);
        printTeamBusiness(team);
        team->checkIndex++;
    }
    team->round_count++;
}
