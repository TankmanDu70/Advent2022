#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

#define C_MAX 1000
#define L_MAX 1000

struct node
{
    bool block;
    bool grain;
};

struct node grid[L_MAX][C_MAX];

struct grain
{
    uint line;
    uint col;
};

uint deepest = 0;

uint leftest = 500;
uint rightest = 500;
uint count = 0;
char mode = 'b';

void printGrid(uint lMin, uint lMax, uint cMin, uint cMax);
bool blocked(uint lin, uint col);

void printGrid(uint lMin, uint lMax, uint cMin, uint cMax)
{
    system("clear");
    for (size_t lin = lMin; lin < lMax; lin++)
    {
        for (size_t col = cMin; col < cMax; col++)
        {
            if (grid[lin][col].grain)
                printf("o");
            else if (grid[lin][col].block == true)
                printf("#");
            else
                printf(".");
        }
        printf("\n");
    }
    printf("bottom! %d\n", count);
}

bool pour(struct grain *gr)
{
    grid[gr->line][gr->col].grain = false;
    if (gr->line < deepest)
    {
        if (!blocked(gr->line + 1, gr->col))
        {
            gr->line++;
            grid[gr->line][gr->col].grain = true;
            return false;
        }
        else if (!blocked(gr->line + 1, gr->col - 1))
        {
            gr->line++;
            gr->col--;
            grid[gr->line][gr->col].grain = true;
            return false;
        }
        else if (!blocked(gr->line + 1, gr->col + 1))
        {
            gr->line++;
            gr->col++;
            grid[gr->line][gr->col].grain = true;
            return false;
        }
        else
        {
            grid[gr->line][gr->col].grain = true;
            grid[gr->line][gr->col].block = true;
            return true;
        }
    }
    else
    {
        if (mode == 'b')
        {
            printf("enter mode ");
            mode = getchar();
        }
        grid[gr->line][gr->col].grain = true;
        printf("bottom! %d\n", count);
        printGrid(0, deepest + 2, leftest - 2, rightest + 5);
        exit(1);
        return true;
    }
}

bool blocked(uint lin, uint col)
{
    return (grid[lin][col].block || grid[lin][col].grain);
}
void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    char *in_char;
    char in_str[4096];

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    in_char = fgets(in_str, 4096, ptr_in);

    for (size_t i = 0; i < L_MAX; i++)
    {
        for (size_t j = 0; j < C_MAX; j++)
        {
            grid[i][j].block = false;
            grid[i][j].grain = false;
        }
    }

    while (in_char != NULL)
    {
        size_t i = 0;
        size_t j = 0;
        size_t prev_i = 0;
        size_t prev_j = 0;

        char *offset = in_str;
        while (sscanf(offset, "%ld,%ld", &j, &i) == 2)
        {
            offset = strstr(offset, " -> ");
            if (offset != NULL)
            {
                offset += 4;
            }
            // printf("got %ld %ld , %ld %ld\n", i, j, prev_i, prev_j);

            if ((prev_i != 0) && (prev_j != 0))
            {
                if (prev_j == j)
                {
                    for (size_t lin = prev_i; lin != i; prev_i < i ? lin++ : lin--)
                    {
                        grid[lin][j].block = true;
                        // printf("%ld %ld++\n", lin, j);
                    }
                    grid[i][j].block = true;
                }
                else
                {
                    for (size_t col = prev_j; col != j; prev_j < j ? col++ : col--)
                    {
                        grid[i][col].block = true;
                        // printf("%ld %ld++\n", i, col);
                    }
                    grid[i][j].block = true;
                }
                if (deepest < i)
                    deepest = i;
                if (leftest > j)
                    leftest = j;
                if (rightest < j)
                    rightest = j;
            }
            prev_i = i;
            prev_j = j;
            // sleep(1);
            if (offset == NULL)
                break;
        }
        in_char = fgets(in_str, 4096, ptr_in);
    };
    // printGrid(0, deepest, leftest, rightest);
    while (true)
    {
        struct grain Alan = {.col = 500, .line = 0};
        while (!pour(&Alan))
        {
            if (mode == 'b')
            {
                printGrid(0, deepest + 2, leftest - 2, rightest + 5);
                usleep(100000);
            }
        }
        if (count % 1000 == 0)
        {
            printGrid(0, deepest + 2, leftest - 2, rightest + 5);
            mode = getchar();
        }
        count++;
    }
}
