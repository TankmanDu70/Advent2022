#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <math.h>

// #define printEVENTS
#define MIN(a, b) a < b ? a : b
#define MAX(a, b) a < b ? b : a

struct sensorReading
{
    int x;
    int y;
    int b_x;
    int b_y;
    int distance;
    int x_range[2];
    int y_range[2];
};

struct cell
{
    int x;
    int y;
    char val;
};

struct cell **grid;

int x_min = __INT32_MAX__;
int y_min = __INT32_MAX__;
int x_max = 0;
int y_max = 0;

struct sensorReading *sensorsReadings;
size_t sensorReadingCount;

void gridPopulate(struct sensorReading *sr);
void gridPrint();
void sensorParser(char *in_str, FILE *ptr_in, char *test);
bool cannotHaveAbeacon(int x, int y);

bool cannotHaveAbeacon(int x, int y)
{
    bool can = false;
    for (size_t sensor = 0; sensor < sensorReadingCount; sensor++)
    {
        if ((sensorsReadings[sensor].b_x == (x - abs(x_min))) && (sensorsReadings[sensor].b_y == (y - abs(y_min))))
            return false;
    }
    for (size_t sensor = 0; sensor < sensorReadingCount; sensor++)
    {
        if (sensorsReadings[sensor].distance >= (abs(x - sensorsReadings[sensor].x - abs(x_min)) + abs(y - sensorsReadings[sensor].y + -abs(y_min))))
        {
            can = true;
            break;
        }
    }
    return can;
}

void gridPopulate(struct sensorReading *sr)
{
    int x_m = abs(x_min);
    int y_m = abs(y_min);

    if (grid[sr->y + y_m][sr->x + x_m].val != '.')
        printf("sensor dupplicate\n");
    grid[sr->y + y_m][sr->x + x_m].val = 'S';

    if (grid[sr->b_y + y_m][sr->b_x + x_m].val != '.')
        printf("dupplicate/overlap\n");
    grid[sr->b_y + y_m][sr->b_x + x_m].val = 'b';
}

void gridPrint()
{
    printf("x_min %2d \ny_min %2d\n", x_min, y_min);
    printf("   ");
    if ((x_max - x_min + 1 < 100) && (y_max - y_min + 1 < 100))
    {
        for (int col = 0; col < x_max - x_min + 1; col++)
            printf("%3d", col - abs(x_min));
        printf("\n");
        for (size_t line = 0; line < y_max - y_min + 1; line++)
        {
            printf("%2ld ", line - abs(y_min));
            for (size_t col = 0; col < x_max - x_min + 1; col++)
            {
                if (grid[line][col].val == '.')
                {
                    if (cannotHaveAbeacon(col, line))
                    {
                        printf("\033[0;31m");
                        printf("%3c", '#');
                        printf("\033[0m");
                    }
                    else
                    {
                        printf("\033[0;32m");
                        printf("%3c", grid[line][col].val);
                        printf("\033[0m");
                    }
                }
                else
                {
                    printf("\033[0;32m");
                    printf("%3c", grid[line][col].val);
                    printf("\033[0m");
                }
            }
            printf("\n");
        }
    }
}

void sensorParser(char *in_str, FILE *ptr_in, char *test)
{
    // struct sensorReading s = {.x = 0, .y = 0, .b_x = 0, .b_y = 0};
    sensorReadingCount++;
    if (sensorReadingCount == 1)
    {
        sensorsReadings = malloc(sizeof(struct sensorReading));
        // printf("malloced\n");
    }
    else
    {
        // printf("reallocing %ld\n", sensorReadistancedingCount * sizeof(struct sensorReading));
        sensorsReadings = realloc(sensorsReadings, sizeof(struct sensorReading) * sensorReadingCount);
        // printf("realloced\n");
    }
    // printf("\n%s\n", in_str);
    struct sensorReading s;
    int res = sscanf(in_str, "Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d\n", &s.x, &s.y, &s.b_x, &s.b_y);
    if (res != 4)
        printf("\t !! ouch %d \n", res);
    else
    {
        sensorsReadings[sensorReadingCount - 1].x = s.x;
        sensorsReadings[sensorReadingCount - 1].y = s.y;
        sensorsReadings[sensorReadingCount - 1].b_x = s.b_x;
        sensorsReadings[sensorReadingCount - 1].b_y = s.b_y;
        sensorsReadings[sensorReadingCount - 1].distance = abs(s.x - s.b_x) + abs(s.y - s.b_y);
        printf("Sensor at x=%3d, y=%3d: closest beacon is at x=%3d, y=%3d count=%ld -distance=%d\n", sensorsReadings[sensorReadingCount - 1].x, sensorsReadings[sensorReadingCount - 1].y, sensorsReadings[sensorReadingCount - 1].b_x, sensorsReadings[sensorReadingCount - 1].b_y, sensorReadingCount, sensorsReadings[sensorReadingCount - 1].distance);
        // printf("\n");
        int nw_x_min = MIN(s.b_x, s.x);
        int nw_y_min = MIN(s.b_y, s.y);
        int nw_x_max = MAX(s.b_x, s.x);
        int nw_y_max = MAX(s.b_y, s.y);
        if (nw_x_min < x_min)
            x_min = nw_x_min;
        if (nw_y_min < y_min)
            y_min = nw_y_min;
        if (nw_x_max > x_max)
            x_max = nw_x_max;
        if (nw_y_max > y_max)
            y_max = nw_y_max;
    }
}

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    char in_str[128];

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    sensorReadingCount = 0;
    sensorsReadings = NULL;
    char *test;
    bool loop = true;

    do
    {

        test = fgets(in_str, 128, ptr_in);
        // printf("loop %s \n", test);
        if (test != NULL)
        {
            sensorParser(in_str, ptr_in, test);
        }
        else
        {
            loop = false;
            printf("Finished !\n");
        }
    } while (/* (*test != EOF) && */ (loop == true));

    x_min -= 10;
    y_min -= 10;
    x_max += 10;
    y_max += 10;

    printf("x_ (%d->%d) \ny_(%d->%d)\n", x_min, x_max, y_min, y_max);
    printf("%s \n", argv[3]);
    if (argv[3][0] == 'y')
    {
        grid = (struct cell **)malloc(sizeof(struct cell) * (y_max - y_min + 1));
        for (size_t i = 0; i < y_max - y_min + 1; i++)
        {
            grid[i] = (struct cell *)malloc(sizeof(struct cell) * (x_max - x_min + 1));
        }
        for (size_t i = 0; i < y_max - y_min + 1; i++)
        {
            for (size_t j = 0; j < x_max - x_min + 1; j++)
            {
                grid[i][j].val = '.';
            }
        }
        for (size_t i = 0; i < sensorReadingCount; i++)
        {
            gridPopulate(sensorsReadings + i);
        }
        gridPrint();
    }
    uint counter = 0;
    for (size_t col = 0; col < x_max - x_min + 1; col++)
    {
        if (cannotHaveAbeacon(col, atoi(argv[2]) + abs(y_min)))
            counter++;
    }
    printf("places that cannot have a beacon %d\n", counter);
}