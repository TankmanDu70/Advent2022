#include "monkeys.h"
#include <stdlib.h>
#include "main.h"

typedef struct m_team
{
    int m_count;
    int round_count;
    int checkIndex;
    monkeyObj monkeys[TEAM_SIZE];
}m_team;


int parseFile(const char *const path, m_team *team);
void printTeam(m_team *team);
void teamCompute(m_team *team);
void teamPrintFinalScore(m_team *team);