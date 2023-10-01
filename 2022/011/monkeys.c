#include "monkeys.h"
#include "main.h"
#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <assert.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STR_LEN 64U

char inStr[MAX_STR_LEN];

bool monkeyParse(FILE *file, monkeyObj m)
{
    int stash[8] = {0};

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "Monkey %d:\r\n", &m->index);

    fgets(inStr, MAX_STR_LEN, file);
    m->stash.size = sscanf(inStr, "  Starting items: %d%*s%d%*s%d%*s%d%*s%d%*s%d%*s%d%*s%d", &stash[0], &stash[1], &stash[2], &stash[3], &stash[4], &stash[5], &stash[6], &stash[7]);
    m->stash.count = m->stash.size;
    m->stash.items = malloc(m->stash.size * sizeof(item));
    for (int ii = 0; ii < m->stash.count; ii++)
    {
        m->stash.items[ii].value = stash[ii];
        itemCtor(&m->stash.items[ii]);
    }

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "  Operation: new = old %c %d", &m->operator, & m->operand);

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "  Test: divisible by %d", &m->div);

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "    If true: throw to monkey %hhu", &m->monKeyTrueIndex);

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "    If false: throw to monkey %hhu", &m->monKeyFalseIndex);

    return (fgetc(file) != EOF);
}

void monkeyCtor(monkeyObj m)
{
    m->inspect_count = 0;
}

void monkeyPrint(monkeyObj m)
{
    printf("Monkey %u:\t", m->index);
    printf("%d items: ", m->stash.count);
    for (int ii = 0; ii < m->stash.count; ii++)
    {
        printf("%d ", m->stash.items[ii].value);
    }
    printf("\r\nopcode: %c %d\r\n", m->operator, m->operand);
    printf("test: %d\t", m->div);
    printf("true: %u\t", m->monKeyTrueIndex);
    printf("false: %u\r\n", m->monKeyFalseIndex);

    printf("business: %u\r\n\r\n", m->inspect_count);
}

void monkeyStashPrint(monkeyObj m)
{
    printf("Monkey %u:\t", m->index);
    printf("%d items: ", m->stash.count);
    for (int ii = m->stash.count - 1; ii >= 0; ii--)
    {
        printf("%u ", m->stash.items[ii].value);
    }
    printf("\r\n");
}

void monkeyBusyPrint(monkeyObj m)
{
    printf("Monkey %u:\t", m->index);
    printf("%5d\titems: ", m->inspect_count);
    printf("\r\n");
}

bool monkeyCompute(monkeyObj m)
{
    PASS_PRINTF("Monkey %d computes %d ", m->index, m->stash.items[m->stash.count - 1].value);
    itemCalc(&m->stash.items[m->stash.count - 1], m->operator, m->operand);
    PASS_PRINTF("->%5d ", m->stash.items[m->stash.count - 1].value);
    m->inspect_count++;
    monkeyBore(m);
    PASS_PRINTF("bored->%5d ", m->stash.items[m->stash.count - 1].value);
    return isItemDiv(&m->stash.items[m->stash.count - 1], m->div);
}

void monkeyPass(monkeyObj me, monkeyObj dest)
{
    PASS_PRINTF("pass to -> %d \r\n", dest->index);
    dest->stash.count++;
    if (dest->stash.count > dest->stash.size)
    {
        dest->stash.size++;
        dest->stash.items = realloc(dest->stash.items, sizeof(item) * dest->stash.size);
    }
    memcpy(&dest->stash.items[dest->stash.count - 1], &me->stash.items[me->stash.count - 1], sizeof(item));
    me->stash.count--;
}

void monkeyBore(monkeyObj me)
{
    (void)me;
}
