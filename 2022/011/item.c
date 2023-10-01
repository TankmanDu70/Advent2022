#include <stdbool.h>
#include <assert.h>
#include <stdio.h>
#include "item.h"
#include "main.h"

factors_t reference = {
    .pr_str.pr_2.value = 2,
    .pr_str.pr_3.value = 3,
    .pr_str.pr_5.value = 5,
    .pr_str.pr_7.value = 7,
    .pr_str.pr_11.value = 11,
    .pr_str.pr_13.value = 13,
    .pr_str.pr_17.value = 17,
    .pr_str.pr_19.value = 19,
    .pr_str.pr_23.value = 23};

void itemCtor(item *it)
{
    for (int ii = 0; ii < PRIME_RANGE; ii++)
    {
        it->factors.primes[ii].value = reference.primes[ii].value;
        it->factors.primes[ii].remainder += (it->value % it->factors.primes[ii].value);
    }
    it->summed = 0;
}

bool isItemDiv(const item *it, const unsigned int operand)
{
    for (int ii = 0; ii < PRIME_RANGE; ii++)
    {
        if (it->factors.primes[ii].value == operand)
        {
            if (it->factors.primes[ii].remainder == 0)
            {
                PASS_PRINTF("%d is_div/%d ", it->value, it->factors.primes[ii].value);
                return true;
            }
            PASS_PRINTF("%d has rem=%d ", it->value, it->factors.primes[ii].remainder);
            break;
        }
    }
    return false;
}

void addAllItemDiv(item *const it, const unsigned int val)
{
    for (int ii = 0; ii < PRIME_RANGE; ii++)
    {
        it->factors.primes[ii].remainder = (it->factors.primes[ii].remainder + val) % it->factors.primes[ii].value;
    }
}

void multAllItemDiv(item *const it, const unsigned int val)
{
    for (int ii = 0; ii < PRIME_RANGE; ii++)
    {
        it->factors.primes[ii].remainder *= val;
    }
}

void sqrAllItemDiv(item *const it)
{
    for (int ii = 0; ii < PRIME_RANGE; ii++)
    {
        it->factors.primes[ii].remainder *= it->factors.primes[ii].remainder;
    }
}

void itemCalc(item *it, const char operator, const unsigned int operand)
{
    switch (operator)
    {
    case '*':
    {
        if (operand)
        {
            multAllItemDiv(it, operand);
        }
        else
        {
            sqrAllItemDiv(it);
        }
        break;
    }
    case '+':
    {
        addAllItemDiv(it, operand);
    }
    }
}
