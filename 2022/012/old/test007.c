#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <stdint.h>

FILE *ptr_in;

char in_str[128];

bool loop = true;
bool foundS = false;

struct node
{
    uint x;
    uint y;
    struct node *prox[4];
    bool can[4];
    bool tried[4]; // Left Top Right Bottom
    char height;
    uint options;
    uint id;
    uint cost;
    bool checked;
    char pref[4];
};

enum dir
{
    left = 0,
    top,
    right,
    down
};

struct dirXY
{
    char x;
    char y;
};

struct dirXY dirMod[4] = {{-1, 0}, {0, -1}, {1, 0}, {0, 1}};
struct node *Snode;
struct node *Enode;
struct node *Cnode;

uint cX = 0;
uint cY = 0;
uint node_cnt = 0;

char oppDir[4] = {right, down, left, top};
char charDir[4] = {'<', '^', '>', 'v'};

uint score = 0;
uint options = 0;

#define INPUT // INPUT README

#ifdef README
#define mapH 5
#define mapL 8
#define DELAY 100000
#define inFile "./Readme"
#elif defined(INPUT)
#define DELAY 10000
#define mapH 41
#define mapL 93
#define inFile "./input"
char grid[mapH][mapL];
#endif

char grid[mapH][mapL];
char path[mapH][mapL];
struct node nodes[mapH][mapL];

#define TOP 0, -1
#define DOWN 0, 1
#define RIGHT 1, 0
#define LEFT -1, 0

struct step
{
    uint8_t x;
    uint8_t y;
    struct step *next;
    struct step *prev;
};

struct step *controller;
uint step_count;

void mapParser(FILE *ptr_in);
int findWay();
void checkOptions(struct node *nod);
bool canGo(struct node *nod, char dir);
void favorDir(struct node *nod);
void printLink(struct node *nod);
bool isInOptList(uint x, uint y);
bool worthGoingThere(struct node *nod, char dir);
void printPath(struct node *nod, uint len);
void printController();

void mapParser(FILE *ptr_in)
{
    uint x = 0;
    uint y = 0;
    char test = fgetc(ptr_in);
    do
    {
        // printf("loop %s \n", test);
        if (test >= 0)
        {
            if (test == '\n')
            {
                x = 0;
                y++;
            }
            else
            {
                grid[y][x] = test;
                if (grid[y][x] == 'S')
                {
                    path[y][x] = 'S';
                    nodes[y][x].x = x;
                    nodes[y][x].y = y;
                    nodes[y][x].id = 0;
                    nodes[y][x].cost = 0;
                    nodes[y][x].height = 1;
                    Snode = &nodes[y][x];
                }
                else if (grid[y][x] == 'E')
                {
                    grid[y][x] = '{';
                    path[y][x] = 'E';
                    nodes[y][x].x = x;
                    nodes[y][x].y = y;
                    nodes[y][x].id = 0;
                    nodes[y][x].cost = 0;
                    nodes[y][x].height = 26;
                    Enode = &nodes[y][x];
                }
                else
                {
                    path[y][x] = '.';
                    nodes[y][x].y = y;
                    nodes[y][x].x = x;
                    nodes[y][x].height = grid[y][x] - 'a' + 1;
                }
                x++;
            }
            // test = fscanf(ptr_in, "%c", &in_char);
            test = fgetc(ptr_in);
        }
        else
        {
            for (size_t i = 0; i < mapH; i++)
            {
                for (size_t j = 0; j < mapL; j++)
                {
                    checkOptions(&nodes[i][j]);
                }
            }
            loop = false;
            printf("Finished Parsing !\n\n");
        }
    } while (/* (*test != EOF) && */ (loop == true));
}

void printController()
{
    for (size_t i = 0; i < step_count; i++)
    {
        printf("(%d:%d) ", controller[i].x, controller[i].y);
    }

    printf("\n");
}

void print2DArray(char f[mapH][mapL])
{
    printf("  ");
    for (size_t col = 0; col < mapL; col++)
        printf("%ld", col / 10);
    printf("\n  ");
    for (size_t col = 0; col < mapL; col++)
        printf("%ld", col % 9);
    printf("\n");
    for (size_t lin = 0; lin < mapH; lin++)
    {
        printf("%2ld", lin);
        for (size_t col = 0; col < mapL; col++)
        {
            // if (isInList(col, lin))
            //     printf("\033[0;32m");
            if ((lin == cY) && (col == cX))
                printf("\033[0;31m");
            printf("%c", f[lin][col]);
            printf("\033[0m");
        }
        printf("\n");
    }
    printf("\n");
}

void printNode(struct node *nod)
{
    printf("%3dth node %p (%2d:%2d) h=%2d (%2c) --options %d (%d=%c)(%d=%c)(%d=%c)(%d=%c) -tried(l%d)(t%d)(r%d)(d%d)\n",
           node_cnt, nod, nod->x + 1, nod->y + 1, nod->height, nod->height + 'a' - 1, nod->options, nod->can[left], nod->prox[left] != NULL ? nod->prox[left]->height + 'a' - 1 : '~', nod->can[top], nod->prox[top] != NULL ? nod->prox[top]->height + 'a' - 1 : '~', nod->can[right], nod->prox[right] != NULL ? nod->prox[right]->height + 'a' - 1 : '~', nod->can[down], nod->prox[down] != NULL ? nod->prox[down]->height + 'a' - 1 : '~', nod->tried[left], nod->tried[top], nod->tried[right], nod->tried[down]);
}

void favorDir(struct node *nod)
{
    int d_X = Enode->x - nod->x;
    int d_Y = Enode->y - nod->y;
    if (abs(d_X) > abs(d_Y))
    {
        nod->pref[0] = d_X > 0 ? right : left;
        nod->pref[1] = d_Y > 0 ? top : down;
        nod->pref[2] = nod->pref[1] == down ? top : down;
        nod->pref[3] = nod->pref[0] == left ? right : left;
    }
    else
    {
        nod->pref[0] = d_Y > 0 ? down : top;
        nod->pref[1] = d_X > 0 ? left : right;
        nod->pref[2] = nod->pref[1] == left ? right : left;
        nod->pref[3] = nod->pref[0] == down ? top : down;
    }
}

int findWay()
{
    int propagated = 0;
    // uint stop = step_count - 1;
    for (size_t i = 0; i < step_count; i++)
    {
        struct node *nod = &nodes[controller->y][controller->x];
        if (nod->options >= 1) //! time for a decision of where we go...
        {
            for (size_t dir = 0; dir < 4; dir++)
            {
                // printf("%de option\n", dir);
                if (nod->can[dir] && (!nod->tried[dir] && (path[nod->x + dirMod[dir].x][nod->y + dirMod[dir].y] == '.')) /* && (nod->options > 0) */)
                {
                    propagated++;
                    path[nod->y][nod->x] = charDir[nod->pref[dir]];
                    step_count++;
                    nod->tried[dir] = true;
                    struct step *st = malloc(sizeof(struct step));
                    st->prev = &controller[i];
                    st->x = nod->x + dirMod[dir].x;
                    st->y = nod->y + dirMod[dir].y;
                    controller=controller,
                    //controller[i].next = st;
                    //controller = realloc(controller, sizeof(struct step) * step_count);
                }
            }
            system("clear");
            print2DArray(path);
            printf("%d\n", step_count);
            usleep(10000);

            // printController();
        }
    }
    print2DArray(path);
    printf("next!!\n");
    // printController();
}

bool canGo(struct node *nod, char dir)
{
    return !nod->tried[dir];
    // return (path[nod->y + dirMod[dir].y][nod->x + dirMod[dir].x] == '.') |
    //        (path[nod->y + dirMod[dir].y][nod->x + dirMod[dir].x] == 'E');
}

bool beenThere(struct node *nod, char dir)
{
    return (path[nod->y + dirMod[dir].y][nod->x + dirMod[dir].x] != '.'); // (isInList(nod->x + dirMod[dir].x, nod->y + dirMod[dir].y)) ;//|| (path[nod->y + dirMod[dir].y][nod->x + dirMod[dir].x] != '.');
}

void checkOptions(struct node *nod)
{
    if (nod->y + 1 < mapH) //! down
    {
        nod->prox[down] = &nodes[nod->y + dirMod[down].y][nod->x + dirMod[down].x];
        if (((nod->prox[down]->height - nod->height) <= 1))
        {
            nod->can[down] = true;
            nod->options += 1;
        }
        else
            nod->can[down] = false;
    }
    else
        nod->prox[down] = NULL;

    if (nod->y > 0) //! top
    {
        nod->prox[top] = &nodes[nod->y + dirMod[top].y][nod->x + dirMod[top].x];
        if (((nod->prox[top]->height - nod->height) <= 1))
        {
            nod->can[top] = true;
            nod->options += 1;
        }
        else
            nod->can[top] = false;
    }
    else
        nod->prox[top] = NULL;

    if (nod->x > 0) //! left
    {
        nod->prox[left] = &nodes[nod->y + dirMod[left].y][nod->x + dirMod[left].x];
        if (((nod->prox[left]->height - nod->height) <= 1))
        {
            nod->can[left] = true;
            nod->options += 1;
        }
        else
            nod->can[left] = false;
    }
    else
        nod->prox[left] = NULL;

    if (nod->x + 1 < mapL) //! right
    {
        nod->prox[right] = &nodes[nod->y + dirMod[right].y][nod->x + dirMod[right].x];
        if (((nod->prox[right]->height - nod->height) <= 1))
        {
            nod->can[right] = true;
            nod->options += 1;
        }
        else
            nod->can[right] = false;
    }
    else
        nod->prox[right] = NULL;
    favorDir(nod);
    printf("%d options at (%d:%d)\n", nod->options, nod->x, nod->y);
}

void main(int argc, char *argv[])
{
    ptr_in = fopen(inFile, "r");

    char in_char;

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    // char test = fscanf(ptr_in, "%c", &in_char);
    Enode = malloc(sizeof(struct node));

    // printf("%c\n", in_char);
    // print2DArray(grid);
    // char c = getchar();
    char c;
    // printNode(Snode);
    // printf("%d\n", __LINE__);
    Snode = malloc(sizeof(struct node));
    Enode = malloc(sizeof(struct node));
    Cnode = Snode;
    mapParser(ptr_in);
    printNode(Cnode);
    controller = malloc(sizeof(struct step));
    step_count++;

    controller->x = Snode->x;
    controller->y = Snode->y;

    while (Cnode->height < 27)
    {
        // printf("Before? ");
        system("clear");
        // c = getchar();

        findWay(Cnode);
        // else if (Cnode->prev != NULL)
        // Cnode = Cnode->prev;
        c = getchar();
        // usleep(DELAY);
    }
}
