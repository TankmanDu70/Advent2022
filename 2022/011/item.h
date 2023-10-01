#include "main.h"
#define PRIME_RANGE 9

typedef struct prime
{
    unsigned int value;
    int remainder;
} prime;

typedef struct ref_div
{
    prime pr_2;
    prime pr_3;
    prime pr_5;
    prime pr_7;
    prime pr_11;
    prime pr_13;
    prime pr_17;
    prime pr_19;
    prime pr_23;
} ref_div;

typedef union factors_t
{
    prime primes[PRIME_RANGE];
    ref_div pr_str;
} factors_t;

typedef struct
{
    int value;
    int summed;
    factors_t factors;
} item;

void itemCtor(item *it);
void itemCalc(item *it, const char operator, const unsigned int operand);
bool isItemDiv(const item *it, const unsigned int operand);
void addAllItemDiv(item *const it, const unsigned int val);
void multAllItemDiv(item *const it, const unsigned int val);
void sqrAllItemDiv(item *const it);
