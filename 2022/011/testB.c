#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>
#include <math.h>

#define printEVENTS

// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109
uint primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109};
#define prime_cnt sizeof(primes) / sizeof(primes[0])

struct item
{
    uint prm_dec[sizeof(primes) / sizeof(primes[0])];
    uint rst;
};

struct monkey
{
    uint index;
    struct item *items;
    uint items_count;
    char operator;
    uint factor;
    bool square;
    char throw_to_false;
    char throw_to_true;
    uint divide_by;
    uint inspect_counter;
};

struct monkey *monkeys;
int monkey_count;

void monkeyParser(char *str, FILE *ptr_in, char *test);

void monkeyPrint(struct monkey *monkey);
void monkeyRound(struct monkey *monkey);
bool monkeyTest(struct monkey *monkey, size_t obj_ind);
void monkeyInspect(struct monkey *monkey, size_t obj_ind);
void monkeyThrow(uint monkey_from, uint monkey_to, uint obj_index);
void monkeyListObjects(struct monkey *monkey);
uint monkeyGetBusiness();
void primedecGet(uint number, struct item *it);
void primedecPrint(struct item *it);
void monkeyExchange(uint monkey_to, uint monkey_from, uint obj_index);
uint primeFactor(struct item *it);
void itemCpy(struct item *it_dst, struct item *it_src);
void itemAdd(struct item *it_1, struct item *it_2);
void itemMult(struct item *it_dst, uint fac);

void main(int argc, char *argv[])
{
    FILE *ptr_in = fopen(argv[1], "r");

    char in_str[64];

    if (ptr_in == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    uint repeat = 0;
    if (argc > 2)
        repeat = atoi(argv[2]);
    else
    {
        printf("Error in argument 2\n");
        return;
    }

    if (repeat <= 0)
    {
        printf("Error in argument 2\n");
        return;
    }

    monkey_count = 0;
    bool loop = true;
    do
    {
        char *test;
        // printf("loop %d %s\n", ptr_in, test);
        // test = fscanf(ptr_in, "%[^\n]", in_str);
        test = fgets(in_str, 64, ptr_in);
        // printf("looped\n");
        //  printf("test= %d\n", test);
        if (test != NULL)
        {
            // printf("%s\n", test);
            // printf("%s ", test);
            monkeyParser(in_str, ptr_in, test);
        }
        else
            loop = false;
    } while (/* (*test != EOF) && */ (loop == true));

    while (repeat > 0)
    {
        printf("\n\nROUND %u \n\n", repeat);
        for (size_t i = 0; i < monkey_count; i++)
            monkeyRound(monkeys + i);
        for (size_t i = 0; i < monkey_count; i++)
            monkeyListObjects(monkeys + i);
        repeat--;
    }
    printf("total business = %u \n", monkeyGetBusiness());
    free(monkeys);
}

void monkeyParser(char *in_str, FILE *ptr_in, char *test)
{
    int count;
    // printf("__parsing__\n");

    if (sscanf(in_str, "Monkey %u:\n", &count) == 1)
    {
        // printf("monkey_++ count=%d\n", monkey_count);
        if (monkey_count == 0)
        {
            monkey_count++;
            monkeys = malloc(sizeof(struct monkey) * monkey_count);
            // printf("monkey_++ malloc \n");
        }
        else
        {
            monkey_count++;
            monkeys = realloc(monkeys, sizeof(struct monkey) * monkey_count);
            // printf("monkey_++ malloc \n");
        }
        monkeys[monkey_count - 1].index = count;
        // printf("count = %d\n", count);
    }
    else
    {
        printf("exit at %d", __LINE__);
        exit(1);
    }
    test = fgets(in_str, 64, ptr_in);
    if (test != NULL)
    {
        uint objects[10];
        int obj_cnt = sscanf(in_str, "  Starting items: %u, %u, %u, %u, %u, %u, %u, %u, %u, %u", objects, objects + 1, objects + 2, objects + 3, objects + 4, objects + 5, objects + 6, objects + 7, objects + 8, objects + 9);
        monkeys[monkey_count - 1].items = malloc(obj_cnt * sizeof(struct item));
        monkeys[monkey_count - 1].items_count = obj_cnt;
        if (obj_cnt >= 0)
        {
            printf("added objects :\n");
            for (size_t i = 0; i < obj_cnt; i++)
            {
                printf("%u dec= ", objects[i]);
                // monkeys[monkey_count - 1].items[i] = objects[i];
                primedecGet(objects[i], &(monkeys[monkey_count - 1].items[i]));
                primedecPrint(&(monkeys[monkey_count - 1].items[i]));
            }
            printf(" ");
        }
        else
        {
            printf("exit at %d", __LINE__);
            exit(1);
        }
    }
    else
    {
        printf("exit at %d", __LINE__);
        exit(1);
    }
    test = fgets(in_str, 64, ptr_in);
    if (test != NULL)
    {
        char operand_1[3];
        char operator[1];
        char operand_2[3];
        uint factor;
        char *substring = strstr(in_str, "new = ");
        int obj_cnt = 0;
        if (substring != NULL)
            obj_cnt = sscanf(substring, "new = %s %s %s", operand_1, operator, operand_2);
        else
            goto abort;
        // printf(strstr(in_str, "new = "));
        if (obj_cnt >= 2)
        {
            uint factor = (uint)atoi(operand_2);
            if (factor > 0)
            {
                printf("#  added operation %s %u\n", operator, factor);
                monkeys[monkey_count - 1].operator= operator[0];
                monkeys[monkey_count - 1].factor = factor;
                monkeys[monkey_count - 1].square = false;
            }
            else if (strcmp(operand_2, "old") == 0)
            {
                monkeys[monkey_count - 1].square = true;
                monkeys[monkey_count - 1].operator= '*'; // It just wants not to find it!!!!
                printf("#  added operation %s old\n", operator);
            }
        }
        else
        {
        abort:
            printf("exit at %d  %d\n", __LINE__, obj_cnt);
            exit(1);
        }
    }
    else
    {
        printf("exit at %d", __LINE__);
        exit(1);
    }

    test = fgets(in_str, 64, ptr_in);
    if (test != NULL)
    {
        int factor;
        int div_factor = sscanf(in_str, "  Test: divisible by %d", &factor);
        if (div_factor == 1)
        {
            for (size_t i = 0; i < prime_cnt; i++)
            {
                if (factor == primes[i])
                {
                    monkeys[monkey_count - 1].divide_by = i;
                    break;
                }
            }
        }
    }
    else
    {
        printf("exit at %d", __LINE__);
        exit(1);
    }

    test = fgets(in_str, 64, ptr_in);
    if (test != NULL)
    {
        int monkey_to;
        int div_factor = sscanf(in_str, "    If true: throw to monkey %d", &monkey_to);
        if (div_factor == 1)
            monkeys[monkey_count - 1].throw_to_true = monkey_to;
    }
    else
    {
        printf("exit at %d", __LINE__);
        exit(1);
    }

    test = fgets(in_str, 64, ptr_in);
    if (test != NULL)
    {
        int monkey_to;
        int div_factor = sscanf(in_str, "    If false: throw to monkey %d", &monkey_to);
        if (div_factor == 1)
            monkeys[monkey_count - 1].throw_to_false = monkey_to;
    }
    else
    {
        printf("exit at %d", __LINE__);
        exit(1);
    }

    test = fgets(in_str, 64, ptr_in);
    // printf("test = %d \n", test);
    monkeyPrint(monkeys + (monkey_count - 1));
}

void monkeyRound(struct monkey *monkey)
{
    for (size_t obj_ind = monkey->items_count; obj_ind--; obj_ind > 0)
    {
        monkeyInspect(monkey, obj_ind);
        if (monkeyTest(monkey, obj_ind) == true)
            monkeyThrow(monkey->index, monkey->throw_to_true, obj_ind);
        else
            monkeyThrow(monkey->index, monkey->throw_to_false, obj_ind);
    }
}

void monkeyInspect(struct monkey *monkey, size_t obj_index)
{
    monkey->inspect_counter++;
#ifdef printEVENTS
    printf("# obj %d:", primeFactor(&monkey->items[obj_index]));
    primedecPrint(&monkey->items[obj_index]);
#endif
    struct item add = {.prm_dec = {0, 0, 0, 0, 0, 0, 0}, .rst = 0};
    switch (monkey->operator)
    {
    case '+':
        monkey->items[obj_index].rst += monkey->factor;
        primedecGet(monkey->items[obj_index].rst + primeFactor(&monkey->items[obj_index]), &add);
        itemAdd(&monkey->items[obj_index], &add);
        break;
    case '*':
        if (monkey->square)
        {
            itemCpy(&add, &monkey->items[obj_index]);
            add.rst = 0;
            itemAdd(&monkey->items[obj_index], &add);

            itemMult(&add, (monkey->items[obj_index].rst));
            add.prm_dec[0] += 1;
            itemAdd(&monkey->items[obj_index], &add);
            monkey->items[obj_index].rst *= monkey->items[obj_index].rst;
        }
        else
            for (size_t i = 0; i < prime_cnt; i++)
                if (monkey->factor == primes[i])
                {
                    monkey->items[obj_index].prm_dec[i] += 1;
                    if (monkey->items[obj_index].rst > 0)
                    {
                        monkey->items[obj_index].rst *= primes[i];
                    }
                }

        break;
    default:
    {
        printf("exited %d\n", __LINE__);
        exit(1);
    }
    };
#ifdef printEVENTS
    printf(" becomes: ");
    primedecPrint(&monkey->items[obj_index]);
    // printf("%u \n", primeFactor(&monkey->items[obj_index]));
#endif
}

bool monkeyTest(struct monkey *monkey, size_t obj_ind)
{
    return (monkey->items[obj_ind].prm_dec[monkey->divide_by] && monkey->items[obj_ind].rst == 0);
}

void monkeyPrint(struct monkey *monkey)
{
    printf("I print monkey %d:\n  obj count = %d\n  Starting items: ", monkey->index, monkey->items_count);
    for (size_t i = 0; i < monkey->items_count; i++)
    {
        printf("%d ", primeFactor(&(monkey->items[i])));
    }
    printf("\n  Operation: new = old %c ", monkey->operator);
    if (monkey->square)
        printf("old\n");
    else
        printf("%d\n", monkey->factor);
    printf("  Test: divisible by %u (%uth)\n", primes[monkey->divide_by], monkey->divide_by);
    printf("    If true: throw to monkey %u\n", monkey->throw_to_true);
    printf("    If false: throw to monkey %u\n", monkey->throw_to_false);
    // printf("__printed monkey__\n");
}

void monkeyListObjects(struct monkey *monkey)
{
    printf("monkey %1d: obj count = %2d\t", monkey->index, monkey->items_count);
    printf("%3d inspections\t", monkey->inspect_counter);
    for (size_t i = 0; i < monkey->items_count; i++)
    {
        printf("%d ", primeFactor(&monkey->items[i]));
    }
    printf("\n");
}

void monkeyThrow(uint monkey_from, uint monkey_to, uint obj_index)
{
#ifdef printEVENTS
    printf("# monkey %d throws %d to monkey %d \n", monkey_from, primeFactor(&monkeys[monkey_from].items[obj_index]), monkey_to);
#endif
    monkeys[monkey_to].items = realloc(monkeys[monkey_to].items, ++monkeys[monkey_to].items_count * sizeof(struct item));
    monkeyExchange(monkey_to, monkey_from, obj_index);
    // if (obj_index != monkeys[monkey_from].items_count)
    //     monkeys[monkey_from].items[obj_index] = monkeys[monkey_from].items[monkeys[monkey_from].items_count];
    monkeys[monkey_from]
        .items = realloc(monkeys[monkey_from].items, --monkeys[monkey_from].items_count * sizeof(struct item));
#ifdef printEVENTS
    monkeyListObjects(monkeys + monkey_from);
    monkeyListObjects(monkeys + monkey_to);
#endif
}

uint monkeyGetBusiness()
{
    uint max_1 = 0;
    uint max_2 = 0;
    for (size_t i = 0; i < monkey_count; i++)
    {
        if (max_2 < monkeys[i].inspect_counter)
            if (max_1 < monkeys[i].inspect_counter)
            {
                max_2 = max_1;
                max_1 = monkeys[i].inspect_counter;
            }
            else
                max_2 = monkeys[i].inspect_counter;
    }
    printf("\nmax 1 = %u max_2 = %u \n", max_1, max_2);
    return max_1 * max_2;
}

void primedecGet(uint number, struct item *it)
{
    uint buff = number;
    // printf("# Decomposing %u into primes = ", number);
    for (size_t i = 0; i < prime_cnt; i++)
        it->prm_dec[i] = 0;
    it->rst = 0;
    for (size_t i = 0; i < prime_cnt;)
    {
        if (number % primes[i] == 0)
        {
            it->prm_dec[i] += 1;
            number /= primes[i];
        }
        else
        {
            i++;
        }
    }
    if (number < primes[0])
        if (number == 1)
            it->rst = 0;
        else
            it->rst = number;
    else
    {
        for (size_t i = 0; i < prime_cnt; i++)
            it->prm_dec[i] = 0;
        it->rst = buff;
    }
    if (primeFactor(it) != buff)
    {
        printf("exit at %d\n", __LINE__);
        exit(1);
    }
}

void primedecPrint(struct item *it)
{
    for (size_t i = 0; i < prime_cnt; i++)
    {
        if (it->prm_dec[i] != 0)
            printf("%u:%u ", primes[i], it->prm_dec[i]);
    }
    printf(" +%u = %u\n", it->rst, primeFactor(it));
}

void monkeyExchange(uint monkey_to, uint monkey_from, uint obj_index)
{
    for (size_t i = 0; i < prime_cnt; i++)
    {
        monkeys[monkey_to].items[monkeys[monkey_to].items_count - 1].prm_dec[i] = monkeys[monkey_from].items[obj_index].prm_dec[i];
    }
    monkeys[monkey_to].items[monkeys[monkey_to].items_count - 1].rst = monkeys[monkey_from].items[obj_index].rst;
}

uint primeFactor(struct item *it)
{
    uint ret = 0;
    // printf("= ");
    for (size_t i = 0; i < prime_cnt; i++)
    {
        if (it->prm_dec[i] != 0)
        {
            if (ret == 0)
                ret = 1;
            for (size_t j = 0; j < it->prm_dec[i]; j++)
            {
                ret *= primes[i];
            }
            // printf("%u:%u ", primes[i], it->prm_dec[i]);
        }
    }
    // printf("\n");
    return ret + it->rst;
}

void itemAdd(struct item *it_1, struct item *it_2)
{
    for (size_t i = 0; i < prime_cnt; i++)
    {
        it_1->prm_dec[i] += it_2->prm_dec[i];
    }
    it_1->rst += it_2->rst;
}

void itemCpy(struct item *it_dst, struct item *it_src)
{
    for (size_t i = 0; i < prime_cnt; i++)
    {
        it_dst->prm_dec[i] = it_src->prm_dec[i];
    }
    it_dst->rst = it_src->rst;
}

void itemMult(struct item *it_dst, uint fac)
{
    for (size_t i = 0; i < prime_cnt; i++)
    {
        it_dst->prm_dec[i] *= fac;
    }
}