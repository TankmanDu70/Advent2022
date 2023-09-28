#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <stdint.h>

#define FOREST_SIZE 99

uint visibles = FOREST_SIZE * 2 + (FOREST_SIZE - 2) * 2;

void printTheWholeFuckingForest(uint8_t trees[][FOREST_SIZE]);
void countingTheFuckingVisibleTrees(uint8_t trees[][FOREST_SIZE]);
int countingTheFuckingSenicView(uint8_t trees[][FOREST_SIZE]);

bool visibleFromSouth(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j);

bool visibleFromLeft(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j);
bool visibleFromRight(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j);
bool visibleFromNorth(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j);
bool visibleFromSouth(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j);

int countingFromLeft(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j);
int countingFromRight(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j);
int countingFromNorth(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j);
int countingFromSouth(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j);

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    char in_char;

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    int test = fscanf(ptr_in, "%c", &in_char);

    printf("created the fucking forest of %d x %d\n", FOREST_SIZE, FOREST_SIZE);
    printf("started of with %d visibles\n", visibles);

    uint8_t trees[FOREST_SIZE][FOREST_SIZE];

    size_t x = 0;
    size_t y = 0;

    printf("counting the fucking trees \n");
    while (test > 0)
    {
        if (in_char != '\n')
        {
            trees[x][y] = in_char;
            // printf("%c", trees[x][y]);
            x++;
        }
        else
        {
            // printf("\n");
            y++;
            x = 0;
        }
        test = fscanf(ptr_in, "%c", &in_char);
    }
    printf("counted the fucking trees \n");
    printTheWholeFuckingForest(trees);
    scanf("%c");
    printf("MAX=%d\n", countingTheFuckingSenicView(trees));
}

void printTheWholeFuckingForest(uint8_t trees[][FOREST_SIZE])
{
    printf("printing fucking trees \n");
    for (size_t i = 0; i < sizeof(trees[0]); i++)
    {
        for (size_t j = 0; j < sizeof(trees[0]); j++)
        {
            printf("%c", trees[j][i]);
        }
        printf("\n");
    }
}

int countingTheFuckingSenicView(uint8_t trees[][FOREST_SIZE])
{
    uint localMax = 0;

    for (size_t i = 1; i < sizeof(trees[0]) - 1; i++)
    {
        for (size_t j = 1; j < sizeof(trees[0]) - 1; j++)
        {
            uint a = countingFromLeft(trees, i, j);
            uint b = countingFromRight(trees, i, j);
            uint c = countingFromNorth(trees, i, j);
            uint d = countingFromSouth(trees, i, j);
            uint newMax = a * b * c * d;

            printf("scn(%d,%d)=%ux%ux%ux%u=%d \n", i, j, a, b, c, d, newMax);
            if (newMax > localMax)
                localMax = newMax;
        }
        // scanf("%c");
    }
    return localMax;
}

void countingTheFuckingVisibleTrees(uint8_t trees[][FOREST_SIZE])
{
    printf("counting the fucking visible trees \n");
    for (size_t i = 1; i < sizeof(trees[0]) - 1; i++)
    {
        for (size_t j = 1; j < sizeof(trees[0]) - 1; j++)
        {

            if (visibleFromLeft(trees, i, j) || visibleFromRight(trees, i, j) || visibleFromNorth(trees, i, j) || visibleFromSouth(trees, i, j))
            {
                printf("(%2ld,%2ld)=%c\n", i, j, trees[i][j]);
                visibles++;
            }
        }
        printf("fucking subtotal %d\n", visibles);
        // scanf("%c");
    }
    printf("fucking total %d\n", visibles);
}

bool visibleFromLeft(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j)
{
    // printf("%s ? ", __func__);
    int k = i - 1;
    while (k >= 0)
    {
        // printf("%c>%c?\t", trees[k][j], trees[i][j]);
        if (trees[k][j] >= trees[i][j])
        {
            // printf("(0-");
            return false;
        }
        k--;
    }
    printf("(1-0-0-0)");
    return true;
    // size_t k = 0;
    // uint highest_from_left = trees[k][j];
    // while (trees[i][j] > highest_from_left)
    //     k++;
}

int countingFromLeft(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j)
{
    // printf("%s ? ", __func__);
    int k = i - 1;
    int ret = 1;
    while (k >= 0)
    {
        // printf("%c>%c?\t", trees[k][j], trees[i][j]);
        if ((trees[k][j] >= trees[i][j]) || (k == 0))
            break;
        ret++;
        k--;
    }
    // printf("left ret=%d\n", ret);
    return ret;
    // size_t k = 0;
    // uint highest_from_left = trees[k][j];
    // while (trees[i][j] > highest_from_left)
    //     k++;
}

bool visibleFromRight(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j)
{
    // printf("%s ? ", __func__);
    int k = i + 1;
    while (k < FOREST_SIZE)
    {
        // printf("%c>%c?\tt", trees[k][j], trees[i][j]);
        if ((trees[k][j] >= trees[i][j]))
        {
            // printf("0-");
            return false;
        }
        k++;
    }
    printf("(0-1-0-0)");
    return true;
    // size_t k = 0;
    // uint highest_from_left = trees[k][j];
    // while (trees[i][j] > highest_from_left)
    //     k++;
}

int countingFromRight(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j)
{
    // printf("%s ? ", __func__);
    int ret = 1;
    int k = i + 1;
    while (k < FOREST_SIZE)
    {
        // printf("loop\n");
        //  printf("%c>%c?\tt", trees[k][j], trees[i][j]);
        if ((trees[k][j] >= trees[i][j]) || (k == FOREST_SIZE - 1))
            break;
        k++;
        ret++;
    }
    // printf("right ret=%d\n", ret);
    return ret;
    // size_t k = 0;
    // uint highest_from_left = trees[k][j];
    // while (trees[i][j] > highest_from_left)
    //     k++;
}

bool visibleFromNorth(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j)
{
    // printf("%s ? ", __func__);
    int k = j - 1;
    while (k >= 0)
    {
        // printf("%c>%c?\t", trees[i][k], trees[i][j]);
        if (trees[i][k] >= trees[i][j] || (k == 0))
        {
            // printf("0-");
            return false;
        }
        k--;
    }
    printf("(0-0-1-0)");
    return true;
    // size_t k = 0;
    // uint highest_from_left = trees[k][j];
    // while (trees[i][j] > highest_from_left)
    //     k++;
}

int countingFromNorth(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j)
{
    // printf("%s ? ", __func__);
    int k = j - 1;
    int ret = 1;
    while (k >= 0)
    {
        // printf("%c>%c?\t", trees[i][k], trees[i][j]);
        if ((trees[i][k] >= trees[i][j] )|| (k == 0))
            break;
        k--;
        ret++;
    }
    // printf("North ret=%d\n", ret);
    return ret;
    // size_t k = 0;
    // uint highest_from_left = trees[k][j];
    // while (trees[i][j] > highest_from_left)
    //     k++;
}

bool visibleFromSouth(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j)
{
    // printf("%s ? ", __func__);
    int k = j + 1;
    while (k < FOREST_SIZE)
    {
        if ((trees[i][k] >= trees[i][j]) || (k == FOREST_SIZE - 1))
        {
            // printf("NOT SOUTH <(%ld,%ld)=%c bested by %c\n", i, k, trees[i][j],trees[i][k]);
            //  printf("0");
            return false;
        }
        k++;
    }
    printf("(0-0-0-1)");
    return true;
    // size_t k = 0;
    // uint highest_from_left = trees[k][j];
    // while (trees[i][j] > highest_from_left)
    //     k++;
}

int countingFromSouth(uint8_t trees[][FOREST_SIZE], uint8_t i, uint8_t j)
{
    // printf("%s ? ", __func__);
    int ret = 1;
    int k = j + 1;
    while (k < FOREST_SIZE)
    {
        if (trees[i][k] >= trees[i][j]|| (k == FOREST_SIZE - 1))
            break;
        k++;
        ret++;
    }
    // printf("South ret=%d\n", ret);
    return ret;
    // size_t k = 0;
    // uint highest_from_left = trees[k][j];
    // while (trees[i][j] > highest_from_left)
    //     k++;
}