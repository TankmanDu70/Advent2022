#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <math.h>

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

#define L_MAX 20
#define C_MAX 20

#define DBG

struct node grid[L_MAX][C_MAX];
void print_grid(void);
bool headAndTailOnSameAxis(void);
int point_distance(void);
void refresh_tail(char dir);

void refresh_queue(char dir);
void move_point(char dir, struct point *p);
void tail_point(struct point *p);
void update_grid(struct point *p);

struct point head = {.name = 'H', .lin = L_MAX >> 1, .col = C_MAX >> 1};
struct point tail = {.name = 'T', .lin = L_MAX >> 1, .col = C_MAX >> 1};

char dummy;
int hashCount = 0;

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

    move_point('I', &head);
    move_point('I', &tail);
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
    refresh_tail(dir);
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
    update_grid(p);
}

void update_grid(struct point *p)
{
    if ((p->lin < L_MAX) && (p->col < C_MAX))
    {
        if (grid[p->lin][p->col].val != 'H')
            grid[p->lin][p->col].val = p->name;
        if (grid[p->prev_lin][p->prev_col].visited)
            grid[p->prev_lin][p->prev_col].val = '#';
        else
            grid[p->prev_lin][p->prev_col].val = '.';

        if ((p->name == 'T') && !grid[p->lin][p->col].visited)
        {
            grid[p->lin][p->col].visited = true;
            hashCount++;
        }
    }
    else
    {
        printf("boundary error : (l=%dc=%d)\n", p->lin, p->col);
        exit(1);
    }
}

void refresh_tail(char dir)
{ // tail;
#ifdef DBG
    printf("dist=%d ", point_distance());
#endif
    if (headAndTailOnSameAxis() && (point_distance() > 1))
        move_point(dir, &tail);
    else if (point_distance() > 2)
        tail_point(&head);
    update_grid(&tail);
}

void tail_point(struct point *p)
{
    
    // tail.prev_col = tail.col;
    // tail.prev_lin = tail.lin;
    // tail.col = p->prev_col;
    // tail.lin = p->prev_lin;
}

int point_distance(void)
{
    return (abs(head.col - tail.col) + abs(head.lin - tail.lin));
}

bool headAndTailOnSameAxis(void)
{
    return ((tail.col == head.col) || (tail.lin == head.lin));
}

void print_grid(void)
{
    // system("clear");
    printf("\n   ");
    for (size_t i = 0; i < C_MAX; i++)
        printf("%ld", i / 10);
    printf("\n   ");
    for (size_t i = 0; i < C_MAX; i++)
        printf("%ld", i % 10);
    printf("\n");
    for (size_t i = 0; i < L_MAX; i++)
    {
        printf("%2ld ", i);
        for (size_t j = 0; j < C_MAX; j++)
        {
            printf("%c", grid[i][j].val);
        }
        printf("\n");
    }
    printf("H:(%d,%d) T:(%d,%d) #:%d\n", head.lin, head.col, tail.lin, tail.col, hashCount);
}
