#include "sensor.h"
#include <stdio.h>
#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>

void sensorCtor(reading *r)
{
    r->x = 0;
    r->y = 0;
    r->r_y = 0;
    r->r_x = 0;
}

bool parseSensors(FILE *file, reading *r)
{
    assert(file != NULL);
    bool stop = false;
    bool ret = fscanf(file, "Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d\r\n", &r->x, &r->y, &r->r_x, &r->r_y) > 0;
    return ret;
}

void parseManDist(reading *r)
{
    r->manDist = abs(r->x - r->r_x) + abs(r->y - r->r_y);
}

void parseBeaconAbsPos(reading *r, beacon *b)
{
    b->x = r->r_x;
    b->y = r->r_y;
}

void sensorPrint(reading *r)
{
    printf("Sensor at x=%d, y=%d:\r\n", r->x, r->y);
    printf("closest beacon is at x=%d, y=%d \r\n", r->r_x, r->r_y);
    printf("Manhatan d=%d\r\n\r\n", r->manDist);
}

void beaconPrint(beacon *b)
{
    printf("--> Beacon at x=%d, y=%d:\r\n", b->x, b->y);
}

unsigned int manDistFromReading(reading *r, int x, int y)
{
    return abs(r->x - x) + abs(r->y - y);
}