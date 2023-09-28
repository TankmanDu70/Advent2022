#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
FILE *ptr_in;

char in_str[128];

bool loop = true;
bool foundS = false;

struct node
{
    uint x;
    uint y;
    char prox[4];  // Left Top Right Bottom
    bool tried[4]; // Left Top Right Bottom
    bool can[4];
    char height;
    uint options;
    uint id;
    uint cost;
    bool checked;
    char pref[4];
    // left top right down
    struct node *next;
    struct node *prev;
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
struct node *nodes;
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

#define TOP 0, -1
#define DOWN 0, 1
#define RIGHT 1, 0
#define LEFT -1, 0

struct step
{
    uint x;
    uint y;
};

void mapParser(FILE *ptr_in);
int findWay(struct node *nod);
void addNode(struct node *nod, uint _x, uint _y, char from);
char mostLikelyDirection(struct node *nod);
void checkOptions(struct node *nod);
bool canGo(struct node *nod, char dir);
struct node *getNodeByXY(struct node *s_node, uint x, uint y);
bool isInList(uint x, uint y);
void favorDir(struct node *nod);
void printOptLink(struct node *nod);
void printLink(struct node *nod);
bool beenThere(struct node *nod, char dir);
bool isInOptList(uint x, uint y);
bool worthGoingThere(struct node *nod, char dir);
void printPath(struct node *nod, uint len);

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
                    path[y][x] = 'S';
                else if (grid[y][x] == 'E')
                {
                    grid[y][x] = '{';
                    path[y][x] = 'E';
                }
                else
                    path[y][x] = '.';
                x++;
            }
            // test = fscanf(ptr_in, "%c", &in_char);
            test = fgetc(ptr_in);
        }
        else
        {
            loop = false;
            printf("Finished Parsing !\n\n");
        }
    } while (/* (*test != EOF) && */ (loop == true));
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
            if (isInList(col, lin))
                printf("\033[0;32m");
            if ((lin == cY) && (col == cX))
                printf("\033[0;31m");
            printf("%c", f[lin][col]);
            printf("\033[0m");
        }
        printf("\n");
    }
    printf("\n");
}

bool isInList(uint x, uint y)
{
    struct node *n = Snode;
    while (n->next != NULL)
    {
        if ((n->x == x) && (n->y == y))
            return true;
        n = n->next;
    }
    return false;
}

void printNode(struct node *nod)
{
    printf("%3dth node %x (%2d:%2d) h=%2d (%2c) --options %d (%d=%c)(%d=%c)(%d=%c)(%d=%c) -tried(l%d)(t%d)(r%d)(d%d)\n",
           node_cnt, nod, nod->x + 1, nod->y + 1, nod->height, nod->height + 'a' - 1, nod->options, nod->can[left], nod->prox[left] + 'a' - 1, nod->can[top], nod->prox[top] + 'a' - 1, nod->can[right], nod->prox[right] + 'a' - 1, nod->can[down], nod->prox[down] + 'a' - 1, nod->tried[left], nod->tried[top], nod->tried[right], nod->tried[down]);
}

void findS()
{
    for (size_t lin = 0; lin < mapH; lin++)
    {
        for (size_t col = 0; col < mapL; col++)
        {
            if (grid[lin][col] == 'S')
            {
                nodes = malloc(sizeof(struct node));
                printf("Found S at %ld:%ld!\n", col + 1, lin + 1);
                // getchar();
                Snode = &nodes[0];
                Snode->id = 1;
                Snode->x = col;
                Snode->y = lin;
                Snode->height = 1;
                Snode->options = 0;
                Snode->cost = 1;
                memset(Snode->tried, false, 4);
                memset(Snode->can, false, 4);
            }
            if (grid[lin][col] == '{')
            {
                printf("Found E at %ld:%ld!\n", col + 1, lin + 1);
                // getchar();
                Enode->x = col;
                Enode->y = lin;
                Enode->height = 26;
            }
        }
    }
}

void printLink(struct node *nod)
{
    struct node *n = nod;
    while (n->next != NULL)
    {
        printNode(n);
        // printf("%c", n->height + 'a' - 1);
        n = n->next;
    }
    printf("\n");
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

int findWay(struct node *nod)
{
    if (nod->checked == false)
        checkOptions(nod);

    if (nod->options >= 1) //! time for a decision of where we go...
    {
        for (size_t dir = 0; dir < 4; dir++)
        {
            // printf("%de option\n", dir);
            if (nod->can[nod->pref[dir]] /* && (nod->options > 0) */)
                if ((nod->tried[nod->pref[dir]] == false))
                {
                    if (!beenThere(nod, nod->pref[dir]))
                    {
                        addNode(nod, dirMod[nod->pref[dir]].x, dirMod[nod->pref[dir]].y, oppDir[nod->pref[dir]]);
                        path[nod->y][nod->x] = charDir[nod->pref[dir]];
                        nod->next->id = nod->id + 1;
                        if (nod->options > 0)
                            nod->options--;
                        nod->tried[nod->pref[dir]] = true;
                        cX = nod->x;
                        cY = nod->y;
                        print2DArray(path);
                        printf("next!!\n");
                        printNode(nod);
                        // printLink(Snode);
                        return 1;
                    }
                }
        }
        goto deadend;
        // printf("%d \n", nod->id);
        // sleep(10);
    }
    else
    {
    deadend:
        cX = nod->x;
        cY = nod->y;
        print2DArray(path);
        printf("deadEnd\n");
        printNode(nod);
        // printOptLink(Snode);
        // path[nod->y][nod->x] = '#';
        return -1;
    }
}

bool canGo(struct node *nod, char dir)
{
    return !nod->tried[dir];
    // return (path[nod->y + dirMod[dir].y][nod->x + dirMod[dir].x] == '.') |
    //        (path[nod->y + dirMod[dir].y][nod->x + dirMod[dir].x] == 'E');
}

struct node *getNodeByXY(struct node *s_node, uint x, uint y)
{
    struct node *n = s_node;
    while (n->prev != NULL)
    {
        if ((n->x == x) && (n->y == y))
            return n;
        n = n->prev;
    }
    return NULL;
}

bool beenThere(struct node *nod, char dir)
{
    return (path[nod->y + dirMod[dir].y][nod->x + dirMod[dir].x] != '.');// (isInList(nod->x + dirMod[dir].x, nod->y + dirMod[dir].y)) ;//|| (path[nod->y + dirMod[dir].y][nod->x + dirMod[dir].x] != '.');
}

bool worthGoingThere(struct node *nod, char dir)
{
    struct node *n = getNodeByXY(nod, nod->x + dirMod[dir].x, nod->y + dirMod[dir].y);
    if (n != NULL)
        return n->options > 1;
    else
        return true; //(isInList(nod->x + dirMod[dir].x, nod->y + dirMod[dir].y)); //| (path[nod->y + dirMod[dir].y][nod->x + dirMod[dir].x] != '.');
}

void checkOptions(struct node *nod)
{
    if (nod->y + 1 < mapH) //! down
    {
        nod->prox[down] = grid[nod->y + 1][nod->x] - 'a' + 1;
        if (((nod->prox[down] - nod->height) <= 1))
        {
            nod->options += 1;
            nod->can[down] = true;
        }
    }
    else
        nod->prox[down] = '~';

    if (nod->y > 0) //! top
    {
        nod->prox[top] = grid[nod->y - 1][nod->x] - 'a' + 1;
        if (((nod->prox[top] - nod->height) <= 1))
        {
            nod->options += 1;
            nod->can[top] = true;
        }
    }
    else
        nod->prox[top] = '~';

    if (nod->x > 0) //! left
    {
        nod->prox[left] = grid[nod->y][nod->x - 1] - 'a' + 1;
        if (((nod->prox[left] - nod->height) <= 1))
        {
            nod->can[left] = true;
            nod->options += 1;
        }
    }
    else
        nod->prox[left] = '~';

    if (nod->x + 1 < mapL) //! right
    {
        nod->prox[right] = grid[nod->y][nod->x + 1] - 'a' + 1;
        if (((nod->prox[right] - nod->height) <= 1))
        {
            nod->can[right] = true;
            nod->options += 1;
        }
    }
    else
        nod->prox[right] = '~';
    favorDir(nod);
    printf("%d options\n", nod->options);
}

void addNode(struct node *nod, uint _x, uint _y, char from)
{
    // printf("%s", __FUNCTION__);
    nod->next = malloc(sizeof(struct node));
    node_cnt++;
    nod->next->prev = nod;
    nod->next->x = nod->x + _x;
    nod->next->y = nod->y + _y;
    nod->next->next = NULL;
    nod->next->height = grid[nod->next->y][nod->next->x] - 'a' + 1;
    memset(nod->next->tried, false, 4);
    memset(nod->next->can, false, 4);
    nod->next->cost = nod->cost + 1;
    nod->next->tried[from] = true;
    nod->options = 0;
    nod->checked = false;
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

    mapParser(ptr_in);
    // printf("%c\n", in_char);
    findS();
    // print2DArray(grid);
    // char c = getchar();
    char c;
    // printNode(Snode);
    // printf("%d\n", __LINE__);
    Cnode = Snode;
    printNode(Cnode);
    while (Cnode->height < 27)
    {
        // printf("Before? ");
        system("clear");
        // c = getchar();

        if (findWay(Cnode) == 1)
            Cnode = Cnode->next;
        else if (Cnode->prev != NULL)
            Cnode = Cnode->prev;
        else
        {
            printf("After? ");
            exit(1);
        }
        c = getchar();
        usleep(DELAY);
    }
}
