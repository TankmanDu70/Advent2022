#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include "item.h"

struct monkey
{
    int index;
    struct stash_t
    {
        item *items;
        int count;
        int size;
    } stash;
    int operand;
    char operator;
    int div;
    uint8_t monKeyFalseIndex;
    uint8_t monKeyTrueIndex;
    int inspect_count;
};

typedef struct monkey *monkeyObj;

bool monkeyParse(FILE *file, monkeyObj m);
void monkeyCtor(monkeyObj m);
void monkeyPrint(monkeyObj m);
void monkeyPass(monkeyObj me, monkeyObj dest);

bool monkeyCompute(monkeyObj m);
void monkeyBore(monkeyObj me);
void monkeyStashPrint(monkeyObj m);
void monkeyBusyPrint(monkeyObj m);
