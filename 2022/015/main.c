#include "main.h"
#include "mapping.h"

static FILE *file;

int main(int argc, int *argv[])
{
    file = fopen("./Readme", "r");

    if (!file)
        EXIT_WITH_ERROR("caca\r\n");

    mapping readingList;
    mappingCtor(&readingList);
    mappingParser(file, &readingList);
    printf("%d free cells on line %d\r\n", freeCellCount(&readingList, 7), 7);

    printf("%d free cells on line %d\r\n", freeCellCount(&readingList, 9), 9);
    printf("%d free cells on line %d\r\n", freeCellCount(&readingList, 10), 10);
    printf("%d free cells on line %d\r\n", freeCellCount(&readingList, 11), 11);

    printf("%d free cells on line %d\r\n", freeCellCount(&readingList, 20), 20);
    //printf("%d free cells on line %d\r\n", freeCellCount(&readingList, 2000000), 2000000);
}