#include "mapping.h"
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

mapping readingList;

void mappingCtor(mapping *m)
{
    m->readingCount = 1;
    m->beaconCount = 1;
    m->beacons = malloc(sizeof(beacon));
    m->readings = malloc(sizeof(reading));
}

void mappingParser(FILE *f, mapping *m)
{
    bool cont = true;
    size_t lr = m->readingCount - 1;
    size_t lb = m->beaconCount - 1;
    while (cont)
    {
        cont = parseSensors(f, &m->readings[lr]);
        if (cont)
        {
            parseBeaconAbsPos(&m->readings[lr], &m->beacons[lb]);
            parseManDist(&m->readings[lr]);
            sensorPrint(&m->readings[lr]);
            m->readingCount++;
            lr++;
            m->readings = realloc(m->readings, m->readingCount * sizeof(reading));
            if (isLastBeaconUnique(m))
            {
                beaconPrint(&m->beacons[lb]);
                m->beaconCount++;
                lb++;
                m->beacons = realloc(m->beacons, m->beaconCount * sizeof(beacon));
            }
        }
    }
}

bool isLastBeaconUnique(mapping *m)
{
    size_t lb = m->beaconCount - 1;
    if (lb == 0)
    {
        return true;
    }
    long int lastX = m->beacons[lb].x;
    long int lastY = m->beacons[lb].y;
    for (int ii = 0; ii < lb; ii++)
    {
        if ((m->beacons[ii].x == lastX) && (m->beacons[ii].y == lastY))
        {
            return false;
        }
    }
    return true;
}

unsigned int freeCellCount(mapping *m, int y)
{
    unsigned int ret = 0;

    size_t lr = m->readingCount - 1;
    for (int ii = 0; ii < lr; ii++)
    {
        int x = m->readings[ii].x;
        int manDist_diff = manDistFromReading(&m->readings[ii], x, y) - m->readings[ii].manDist;

        if (manDist_diff != 0)
        {
            m->readings[ii].x_max = x + abs(manDist_diff);
            m->readings[ii].x_min = x - abs(manDist_diff);

            printf("reading ");
            printRange(&m->readings[ii]);

            int jj = ii;
            while (jj > 0)
            {
                jj--;

                if (isInRange(&m->readings[jj], m->readings[ii].x_max))
                    m->readings[ii].x_max = m->readings[jj].x_min - 1;

                if (isInRange(&m->readings[jj], m->readings[ii].x_min))
                    m->readings[ii].x_min = m->readings[jj].x_max + 1;

                printRange(&m->readings[ii]);
            }

            int addendum = (m->readings[ii].x_max - m->readings[ii].x_min) + 1;

            printf("-> %d \r\n", addendum > 0 ? addendum : 0);

            if (addendum > 0)
                ret += addendum;
        }
    }

    removeBeacons(m, &ret, y);
    removeSensors(m, &ret, y);

    printMap(m);

    return ret;
}

bool isInRange(reading *r, int x)
{
    return (r->x_max >= x && r->x_min <= x);
}

void removeBeacons(mapping *m, int *ret, int y)
{
    size_t lb = m->beaconCount - 1;
    for (int jj = 0; jj < lb; jj++)
    {
        size_t lr = m->readingCount - 1;
        for (int ii = 0; ii < lr; ii++)
        {
            if ((m->beacons[jj].y == y) && isInRange(&m->readings[ii], m->beacons[jj].x))
            {
                *ret = *ret - 1;
                printf("removed one B\r\n");
            }
        }
    }
}

void removeSensors(mapping *m, int *ret, int y)
{
    size_t lr = m->readingCount - 1;
    for (int ii = 0; ii < lr; ii++)
    {
        if ((m->readings[ii].y == y))
        {
            *ret = *ret - 1;
            printf("removed one S\r\n");
        }
    }
}

void removeOverlappings(mapping *m, int *ret, int y)
{
    int x_array[m->readingCount];
    memset(x_array, 0, m->readingCount * sizeof(int));

    size_t lr = m->readingCount - 1;
    for (int ii = 0; ii < lr; ii++)
    {
        if ((m->beacons[ii].y == y) && isInRange(&m->readings[ii], m->beacons[ii].x))
        {
            *ret--;
        }
    }
}

void printRange(reading *r)
{
    printf(" [%d,%d] ", r->x_min, r->x_max);
}

void printMap(mapping *m)
{
    int x_min = 0;
    int x_max = 0;
    int y_min = 0;
    int y_max = 0;

    size_t lr = m->readingCount - 1;
    size_t lb = m->beaconCount - 1;

    for (int ii = 0; ii < lr; ii++)
    {
        if ((m->readings[ii].x > x_max))
            x_max = m->readings[ii].x;
        if ((m->readings[ii].x < x_min))
            x_min = m->readings[ii].x;
        if ((m->readings[ii].y > y_max))
            y_max = m->readings[ii].y;
        if ((m->readings[ii].y < y_min))
            y_min = m->readings[ii].y;
    }

    int size_x = x_max - x_min + 2;
    int size_y = y_max - y_min;

    char *map = malloc(sizeof(char) * (size_x) * (size_y));
    memset(map, '.', sizeof(char) * (size_x) * (size_y));

    for (int ii = 0; ii < size_y; ii++)
    {
        map[ii * size_x] = '\r';
        map[ii * size_x + 1] = '\n';
    }

    for (int ii = 0; ii < lr; ii++)
    {
        map[(m->readings[ii].x + abs(x_min)) + ((m->readings[ii].y + abs(y_min)) * size_x)] = 'S';
    }

    for (int ii = 0; ii < lb; ii++)
    {
        map[(m->beacons[ii].x + abs(x_min)) + ((m->beacons[ii].y + abs(y_min)) * size_x)] = 'B';
    }

    printf(map);
    printf("\r\n");
    free(map);
}