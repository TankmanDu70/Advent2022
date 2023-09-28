#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <stdint.h>

#define stack_max 72
#define dock_count 9

typedef struct stack_t
{
    char list[stack_max];
    uint8_t top;
} stack;

typedef struct dock_t
{
    stack docks[dock_count];
    uint8_t highestStack;
} dock;

int getCommand(FILE *ptr_in, uint8_t *cnt, uint8_t *org, uint8_t *dst);
void fillStack(dock *dock, uint8_t dock_index, char *str);
void updateHighestStack(dock *dock, uint8_t newMax);
void permutate(dock *dock, uint8_t count, uint8_t origin, uint8_t destination);

void printDock(dock *dock)
{
    //system("clear");
    
    //  for (size_t dck = 0; dck < dock_count; dck++)
    //{
    //      printf("%s\n", dock->docks[dck].list);
    //  }
    uint8_t highest_stack = dock->highestStack;
    if (highest_stack > 0)
    {
        printf("highest_stack %d\n", highest_stack);
        // return;
    }
    for (uint8_t pos = 0; pos <= highest_stack; pos++)
    {
        if (highest_stack - pos >= 0)
            printf("%d-->\t", highest_stack - pos + 1);
        // printf("%d ", pos);
        for (size_t dck = 0; dck < dock_count; dck++)
        {
            if (dock->docks[dck].list[highest_stack - pos] != 0)
                printf("%c\t", dock->docks[dck].list[highest_stack - pos]);
            else
                printf("\t");
        }
        printf("\n");
        if (highest_stack - pos == 0)
        {
            printf("0-->\t1\t2\t3\t4\t5\t6\t7\t8\t9\n--->\t");
            for (size_t i = 0; i < dock_count; i++)
            {
                printf("%d\t", dock->docks[i].top);
                /* code */
            }
            printf("\n");
        }
        // usleep(10000);
    }
}

void setDock(dock *dock)
{
    dock->highestStack = 0;
    for (size_t i = 0; i < dock_count; i++)
    {
        memset(dock->docks[i].list, 0, stack_max);
        /* code */
    }
    // printDock(dock);
    fillStack(dock, 0, "ZTFRWJG");
    fillStack(dock, 1, "GWM");
    fillStack(dock, 2, "JNHG");
    fillStack(dock, 3, "JRCNW");
    fillStack(dock, 4, "WFSBGQVM");
    fillStack(dock, 5, "SRTDVWC");
    fillStack(dock, 6, "HBNCDZGV");
    fillStack(dock, 7, "SJNMGC");
    fillStack(dock, 8, "GPNWCJDL");

    // uint8_t highest_stack = stack_max;
    // for (size_t pos = highest_stack; pos > 0; pos--)
    //     for (size_t dck = 0; dck < dock_count; dck++)
    //     {
    //         dock.docks[dck].list[pos] = pos;
    //         // printf("%hhd\t", dock.docks[dck].list[pos]);
    //     }
}

void permutate(dock *dock, uint8_t count, uint8_t origin, uint8_t destination)
{
    for (size_t i = 0; i < count; i++)
    {
        //printf("move %dth(%c)-%d to %d from %d(%c)-%d\n", origin + 1, dock->docks[origin].list[dock->docks[origin].top - 1], dock->docks[origin].top - 1, dock->docks[destination].top, destination + 1, dock->docks[destination].list[dock->docks[destination].top - 1], dock->docks[destination].top);

        dock->docks[destination].top++;
        dock->docks[destination].list[dock->docks[destination].top - 1] = dock->docks[origin].list[dock->docks[origin].top - 1];
        dock->docks[origin].list[dock->docks[origin].top - 1] = 0;

        dock->docks[origin].top--;
        updateHighestStack(dock, dock->docks[destination].top);
        // system("clear");
        // printDock(&dock);
        //usleep(10000);
    }
}

void fillStack(dock *dock, uint8_t dock_index, char *str)
{
    uint8_t len = strlen(str);
    for (size_t i = 0; i < len; i++)
    {
        dock->docks[dock_index].list[i] = str[i];
        // printf("%c\t", str[i]);
        /* code */
    }
    dock->docks[dock_index].top = len;
    updateHighestStack(dock, len);
    // printf("len %d \0\n", strlen(str));
    // printf("\t\t%s\n", dock->docks[dock_index].list);
}

void updateHighestStack(dock *dock, uint8_t newMax)
{
    if (newMax > dock->highestStack)
    {
        dock->highestStack = newMax;
        // printf("newMax %d", newMax);
    }
}

int getCommand(FILE *ptr_in, uint8_t *cnt, uint8_t *org, uint8_t *dst)
{
    return fscanf(ptr_in, "move %hhd from %hhd to %hhd \n", cnt, org, dst);
}

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    dock dock;

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    uint8_t count = 0;
    uint8_t origin = 0;
    uint8_t destination = 0;
    int test = 0;

    setDock(&dock);
    printDock(&dock);

    test = getCommand(ptr_in, &count, &origin, &destination);
    printf("test = %d\n", test);
    while (test >= 0)
    {

        printf("commands is to take %d from %d to %d\n\n", count, origin, destination);
        usleep(500000);
        permutate(&dock, count, origin - 1, destination - 1);
        printDock(&dock);
        //return;
        test = getCommand(ptr_in, &count, &origin, &destination);

        // system("clear");
        if (test == -1)
            break;
        // test = -1;
    }
}
