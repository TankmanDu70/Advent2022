#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

typedef struct beacon_t
{
    int x;
    int y;
    uint8_t index;
} beacon;

typedef struct reading_t
{
    int x;
    int y;
    int r_x;
    int r_y;
    int x_max;
    int x_min;
    int manDist;
} reading;

bool parseSensors(FILE *file, reading *r);
void parseManDist(reading *r);
void parseBeaconAbsPos(reading *r, beacon *b);

unsigned int manDistFromReading(reading *r, int x, int y);

void sensorCtor(reading *r);
void sensorPrint(reading *r);
void beaconPrint(beacon *r);