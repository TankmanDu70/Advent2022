#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct
{
    int value;
    struct divisible
    {
        bool by_2;
        bool by_3;
        bool by_5;
        bool by_7;
        bool by_11;
        bool by_13;
        bool by_17;
        bool by_19;
    } is_div;
} item;

struct monkey
{
    int index;
    struct stash_t
    {
        item *items;
        int items_count;
    } stash;
    bool (*isDivisible)(item *it);
    struct monkey *monKeyFalse;
    struct monkey *monKeyTrue;
    int inspect_counter;
};

struct monkey *monkeys[8];

bool parseMonkey(FILE *file, struct monkey *m);
void parseFile(char *path);
void monkeyCtor(struct monkey *m);