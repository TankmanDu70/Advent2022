#include "monkeys.h"
#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <assert.h>
#include <stdlib.h>

#define MAX_STR_LEN 64U

char inStr[MAX_STR_LEN];

void parseFile(char *path)
{
    FILE *file = fopen(path, "r");
    struct monkey *m = malloc(sizeof(struct monkey));
    while (parseMonkey(file, m))
    {
        monkeyCtor(m);
    }
}

bool parseMonkey(FILE *file, struct monkey *m)
{
    int count = 0;
    int stash[4] = {0};
    int stashCount = 0;
    char opcode = ' ';
    int factor = 0;
    int div = 1;
    uint8_t trueMonkey;
    uint8_t falseMonkey;

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "Monkey %d:\r\n", &count);
    printf("Monkey %u\r\n", count);

    fgets(inStr, MAX_STR_LEN, file);
    stashCount = sscanf(inStr, "  Starting items: %d%*s%d%*s%d%*s%d", &stash[0], &stash[1], &stash[2], &stash[3]);
    printf("%d items: %d, %d, %d, %d\r\n", stashCount, stash[0], stash[1], stash[2], stash[3]);

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "  Operation: new = old %c %d", &opcode, &factor);
    printf("opcode: %c %d\r\n", opcode, factor);

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "  Test: divisible by %d", &div);
    printf("test: %d\r\n", div);

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "    If true: throw to monkey %hhu", &trueMonkey);
    printf("true: %u\r\n", trueMonkey);

    fgets(inStr, MAX_STR_LEN, file);
    sscanf(inStr, "    If false: throw to monkey %hhu", &falseMonkey);
    printf("false: %u\r\n\r\n", falseMonkey);

    m->index = count;
    m->stash.items_count = 0;

    return (fgetc(file) != EOF);
}

void monkeyCtor(struct monkey *m) { (void)m; };