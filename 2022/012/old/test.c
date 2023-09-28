#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
FILE *ptr_in;

char in_str[128];

uint x = 0;
uint y = 0;
bool loop = true;
bool foundS = false;

struct node
{
    char U;
    char D;
    char R;
    char L;
    uint x;
    uint y;
    char h;
};

struct node *nodes;
struct node *Snode;
struct node *Enode;

void mapParser(char c);
void findWay(struct node nod);

void mapParser(char c)
{
    x++;
    printf("Search S ! at x=%2lu y=%2lu == %c", x, y, c);
    if (!foundS)
    {
        if (c == 'S')
        {
            foundS = true;
            nodes = malloc(sizeof(struct node));
            printf("Found S !\n");
            Snode = nodes;
            nodes[0].x = x;
            nodes[0].y = y;
            nodes[0].h = (char)fscanf(ptr_in, "%c");
        }
    }
    if (c == '\n')
    {
        y++;
        printf(" - nl!\n");
    }
}

void findS()
{
    for ()
        uint x = 0, y = 0;
    if (c == 'S')
    {
        foundS = true;
        nodes = malloc(sizeof(struct node));
        printf("Found S !\n");
        Snode = nodes[]

                nodes[0]
                    .x = x;
        nodes[0].y = y;
        nodes[0].h = (char)fscanf(ptr_in, "%c");
    }
}

void findWay(struct node nod)
{
}

void main(int argc, char *argv[])
{
    ptr_in = fopen(argv[1], "r");

    char in_char;

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    char test = fscanf(ptr_in, "%c", &in_char);
    printf("%c\n", in_char);
    do
    {
        // printf("loop %s \n", test);
        if (test > 0)
        {
            mapParser(in_char);
            test = fscanf(ptr_in, "%c", &in_char);
        }
        else
        {
            loop = false;
            printf("Finished !\n");
        }
    } while (/* (*test != EOF) && */ (loop == true));
}
