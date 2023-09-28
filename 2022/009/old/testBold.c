#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <math.h>

struct dir
{
    int lin;
    int col;
};

struct point
{
    uint lin;
    uint col;
    uint _lin;
    uint _col;
    struct dir _dir;
    char name;
};

#define L_MAX 40
#define C_MAX 40

#define DBG

char grid[L_MAX][C_MAX];
void print_grid(void);
bool headAndTailOnSameAxis(struct point *h, struct point *t);
bool isTail(struct point *p);
int point_distance(struct point *h, struct point *t);
void refresh_tail(char *dir, struct point *h, struct point *t);

void refresh_queue(char dir);
void move_point(char dir, struct point *p);
void tail_point(char *dir, struct point *h, struct point *t);
void update_grid(struct point *p);

struct point head = {.name = 'H', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail1 = {.name = '1', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail2 = {.name = '2', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail3 = {.name = '3', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail4 = {.name = '4', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail5 = {.name = '5', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail6 = {.name = '6', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail7 = {.name = '7', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail8 = {.name = '8', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail9 = {.name = '9', .lin = L_MAX >> 1, .col = C_MAX >> 1};

struct point *rope[10] = {
    &head,
    &tail1,
    &tail2,
    &tail3,
    &tail4,
    &tail5,
    &tail6,
    &tail7,
    &tail8,
    &tail9,
};

char dummy;
int hashCount = 1;

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    char in_char;
    int count;

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    int rate = atoi(argv[2]);
    printf("Rate error %d\n", rate);
    sleep(1);
    if (rate < 0)
    {
        printf("Rate error %d\n", rate);
        return;
    }

    for (size_t i = 0; i < sizeof(grid) / sizeof(grid[0]); i++)
    {
        memset(grid[i], '.', sizeof(grid[0]));
    }

    for (size_t i = 0; i < sizeof(rope) / sizeof(rope[0]); i++)
    {
        move_point('I', rope[i]);
    }
#ifdef DBG
    print_grid();
#endif
    int test = fscanf(ptr_in, "%c %d \n", &in_char, &count);
    // printf("%c,%d steps\t", in_char, count);

    while (test > 0)
    {
        // printf("%c - %d steps ", in_char, count);
        for (size_t i = 0; i < count; i++)
        {
#ifdef DBG
            // system("clear");
            printf("iterate %ld \n", i);
            print_grid();
#endif
            refresh_queue(in_char);
#ifdef DBG
            usleep(1000 * rate);
            // scanf("%c\n", &dummy);
#else
#endif
            printf("%d\n", hashCount);
        }
        test = fscanf(ptr_in, "%c %d \n", &in_char, &count);
    }
    printf("%d", hashCount);
#ifdef DBG
    print_grid();
#endif
}

void refresh_queue(char dir)
{
    move_point(dir, &head);
    for (size_t i = 1; i < sizeof(rope) / sizeof(rope[0]); i++)
    {
        refresh_tail(&dir, rope[i - 1], rope[i]);
    }

    // tail;
}

void move_point(char dir, struct point *p)
{ // head;
    // printf("%s - %c  \n", __func__, dir);
    p->_col = p->col;
    p->_lin = p->lin;
    switch (dir)
    {
    case 'R':
#ifdef DBG
        printf("%c+c ", p->name);
#endif
        p->col++;
        p->_dir.col = 1;
        break;
    case 'L':
#ifdef DBG
        printf("%c-c ", p->name);
#endif
        p->col--;
        p->_dir.col = -1;
        break;
    case 'U':
#ifdef DBG
        printf("%c+l ", p->name);
#endif
        p->lin--;
        p->_dir.lin = -1;
        break;
    case 'D':
#ifdef DBG
        printf("%c-l ", p->name);
#endif
        p->lin++;
        p->_dir.lin = 1;
        break;
    case 'I':
#ifdef DBG
        printf("init\n");
#endif
        grid[p->lin][p->col] = p->name;
        return;
    default:
        printf("unforeseen case \n");
        exit(1);
        break;
    }
    update_grid(p);
}

void update_grid(struct point *p)
{
    if ((p->lin < L_MAX) && (p->col < C_MAX))
    {
        // if (grid[p->lin][p->col] != '#')
        grid[p->lin][p->col] = p->name;
        if (grid[p->_lin][p->_col] != '#')
            if (isTail(p))
            {
                grid[p->_lin][p->_col] = '#';
                hashCount++;
            }
            else
                grid[p->_lin][p->_col] = '.';
    }
    else
    {
        printf("boundary error : (l=%dc=%d)\n", p->lin, p->col);
        exit(1);
    }
}

bool isTail(struct point *p)
{
    for (size_t i = 1; i < sizeof(rope) / sizeof(rope[0]); i++)
    {
        if (rope[i] == p)
            return true;
    }
    return false;
}

void refresh_tail(char *dir, struct point *h, struct point *t)
{ // tail;
#ifdef DBG
    printf("dist=%d ", point_distance(h, t));
#endif
    if (headAndTailOnSameAxis(h, t) && (point_distance(h, t) > 1))
        move_point(*dir, t);
    else if (point_distance(h, t) > 2)
        tail_point(dir, h, t);
    update_grid(t);
}

void tail_point(char *dir, struct point *h, struct point *t)
{
    t->_col = t->col;
    t->_lin = t->lin;
    t->col = h->_col;
    t->lin = h->_lin;
}

int point_distance(struct point *h, struct point *t)
{
    return (abs(h->col - t->col) + abs(h->lin - t->lin));
}

bool headAndTailOnSameAxis(struct point *h, struct point *t)
{
    return ((t->col == h->col) || (t->lin == h->lin));
}

void print_grid(void)
{
    printf("\n   ");
    for (size_t i = 0; i < C_MAX; i++)
        printf("%2ld ", i);
    printf("\n");
    for (size_t i = 0; i < (sizeof(grid) / sizeof(grid[0])); i++)
    {
        printf("%2ld ", i);
        for (size_t j = 0; j < sizeof(grid[0]); j++)
        {
            printf("%2c ", grid[i][j]);
        }
        printf("\n");
    }
    // printf("H:(%d,%d) T:(%d,%d) #:%d\n", head.lin, head.col, t->lin, tail.col, hashCount);
}
