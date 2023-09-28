#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

#define DBG
int sum = 0;

struct processor
{
    int tickCount;
    int signalStrength;
    int value;
    int promptRequest[6];
    size_t req_ptr;
};

void prompt(struct processor *uc)
{
#ifdef DBG
    printf("tick %3d - uc={.val=%3d .sig=%3d}\n", uc->tickCount, uc->value, uc->signalStrength);
#endif
    if (uc->promptRequest[uc->req_ptr] == uc->tickCount)
    {
        printf("tick %3d , val = %3d sig = %3d \n", uc->tickCount, uc->value, uc->signalStrength);
        sum += uc->signalStrength;
        uc->req_ptr++;
    }
}

void noop(struct processor *uc)
{
    uc->tickCount++;
    uc->signalStrength = uc->tickCount * uc->value;
    prompt(uc);
}
void addx(struct processor *uc, int val)
{
    uc->tickCount += 1;
    uc->signalStrength = uc->tickCount * uc->value;
    // #ifdef DBG
    //     printf(" ");
    // #endif
    prompt(uc);
    uc->tickCount += 1;
    uc->signalStrength = uc->tickCount * uc->value;
    prompt(uc);
    uc->value += val;
}

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    struct processor uc =
        {
            .tickCount = 0, .signalStrength = 1, .value = 1, .promptRequest = {20, 60, 100, 140, 180, 220}, .req_ptr = 0};

    // 20th, 60th, 100th, 140th, 180th, and 220th cycles
    char ins[5] = {
        ' ',
        ' ',
        ' ',
        ' ',
        '\0'};
    int value = 0;

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    int test = fscanf(ptr_in, "%s %d \n", ins, &value);

    while (test > 0)
    {
#ifdef DBG
        printf("%s %d\t", ins, value);
#endif
        if (strcmp(ins, "noop") == 0)
        {
            noop(&uc);
        }
        else if (strcmp(ins, "addx") == 0)
        {
            addx(&uc, value);
            // printf("%s is addx\n", ins);
        }
        test = fscanf(ptr_in, "%s %d \n", ins, &value);
        // printf("%d = %s ? %d - uc :%d \n", test, ins, value, uc.signalStrength);
    }
    printf("sum=%d\n", sum);
}
