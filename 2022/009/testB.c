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
    uint prev_lin;
    uint prev_col;
    char name;
};

struct node
{
    char val;
    bool visited;
};

#define L_MAX 1000
#define C_MAX 1000

#define ROPE_LEN 10
#define DBG

struct node grid[L_MAX][C_MAX];
void print_grid(void);
bool headAndTailOnSameAxis(struct point *h, struct point *t);
bool isTail(struct point *p);
int point_distance(struct point *h, struct point *t);
void refresh_tail(char *dir, struct point *h, struct point *t);

void refresh_queue(char dir);
void move_point(char dir, struct point *p);
void tail_point(char *dir, struct point *h, struct point *t);
void update_grid(void);

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

struct point *rope[ROPE_LEN] = {
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

    for (size_t i = 0; i < L_MAX; i++)
    {
        for (size_t j = 0; j < C_MAX; j++)
        {
            grid[i][j].val = '.';
            grid[i][j].visited = false;
        }
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

        //getchar();
        // printf("%c - %d steps ", in_char, count);
        for (size_t i = 1; i <= count; i++)
        {
#ifdef DBG
            // system("clear");
#endif
            refresh_queue(in_char);
            printf("\niterate %ld \n", i);
            print_grid();
#ifdef DBG
            usleep(10 * rate);
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
    for (size_t i = 1; i < ROPE_LEN; i++)
    {
        refresh_tail(&dir, rope[i - 1], rope[i]);
    }
    for (size_t i = 0; i < ROPE_LEN; i++)
    {
        printf("%c(%d,%d) ", rope[i]->name, rope[i]->col, rope[i]->lin);
    }
    printf("\n");
    update_grid();
    // tail;
}

void move_point(char dir, struct point *p)
{ // head;
    // printf("%s - %c  \n", __func__, dir);

    p->prev_col = p->col;
    p->prev_lin = p->lin;

    switch (dir)
    {
    case 'R':
#ifdef DBG
        printf("%c+c ", p->name);
#endif
        p->col++;
        break;
    case 'L':
#ifdef DBG
        printf("%c-c ", p->name);
#endif
        p->col--;
        break;
    case 'U':
#ifdef DBG
        printf("%c+l ", p->name);
#endif
        p->lin--;
        break;
    case 'D':
#ifdef DBG
        printf("%c-l ", p->name);
#endif
        p->lin++;
        break;
    case 'I':
#ifdef DBG
        printf("init\n");
#endif
        grid[p->lin][p->col].val = p->name;
        return;
    default:
        printf("unforeseen case \n");
        exit(1);
        break;
    }
    if (p->name == '9')
        grid[p->lin][p->col].visited = true;
    update_grid();
}

void update_grid()
{
    for (size_t i = 0; i < L_MAX; i++)
    {
        for (size_t j = 0; j < C_MAX; j++)
        {
            if (!grid[i][j].visited)
                grid[i][j].val = '.';
            else
                grid[i][j].val = '#';
        }
    }

    for (size_t i = 0; i < ROPE_LEN; i++)
    {
        grid[rope[i]->lin][rope[i]->col].val = rope[i]->name;
    }
}

bool isTail(struct point *p)
{
    for (size_t i = 1; i < ROPE_LEN; i++)
    {
        if (rope[i] == p)
            return true;
    }
    return false;
}

void refresh_tail(char *dir, struct point *h, struct point *t)
{ // tail;
#ifdef DBG
    //printf("dist=%d ", point_distance(h, t));
#endif
    if ((point_distance(h, t) > 1))
        if ((headAndTailOnSameAxis(h, t) && (point_distance(h, t) > 1)) || (point_distance(h, t) > 2))
            tail_point(dir, h, t);
    update_grid();
}

void tail_point(char *dir, struct point *h, struct point *t)
{
    if (h->col > t->col)
        t->col++;
    else if (h->col < t->col)
        t->col--;
    if (h->lin > t->lin)
        t->lin++;
    else if (h->lin < t->lin)
        t->lin--;
    if (t->name == '9')
        grid[t->lin][t->col].visited = true;
    // t->prev_col = t->col;
    // t->prev_lin = t->lin;
    // t->col = h->prev_col;
    // t->lin = h->prev_lin;
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
    // system("clear");
    uint vis_counter = 0;
   // printf("\n   ");
    //for (size_t i = 0; i < C_MAX; i++)
        //printf("%ld", i / 10);
    //printf("\n   ");
   // for (size_t i = 0; i < C_MAX; i++)
        //printf("%ld", i % 10);
    //printf("\n");
    for (size_t i = 0; i < L_MAX; i++)
    {
        //printf("%2ld ", i);
        for (size_t j = 0; j < C_MAX; j++)
        {
            //printf("%c", grid[i][j].val);
            if (grid[i][j].visited)
                vis_counter++;
        }
        //printf("\n");
    }
    printf("Visited %d \n",vis_counter);
    // printf("H:(%d,%d) T:(%d,%d) #:%d\n", head.lin, head.col, tail.lin, tail.col, hashCount);
}
