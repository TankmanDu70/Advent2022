#include <stdbool.h>
#include "sensor.h"
#include "stdint.h"

typedef struct mapping_t
{
    uint8_t readingCount;
    reading *readings;
    uint8_t beaconCount;
    beacon *beacons;
} mapping;

extern mapping readingList;

void mappingParser(FILE *f, mapping *m);
void mappingCtor(mapping *m);
bool isLastBeaconUnique(mapping *m);
unsigned int freeCellCount(mapping *m, int y);
bool isInRange(reading *r, int x);
void removeBeacons(mapping *m, int *ret, int y);
void printRange(reading *r);
void removeSensors(mapping *m, int *ret, int y);
void printMap(mapping *m);
void printMapMatch(mapping *m, int y);
void printRangeDetail(mapping *m, reading *r, int y, int *count);
bool hasBeacon(mapping *m, int x, int y);
bool hasSensor(mapping *m, int x, int y);