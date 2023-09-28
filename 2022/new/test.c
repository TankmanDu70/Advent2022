#include <stdio.h>

int main(int argc, char *argv[])
{
    char test = 0;
//    scanf("%d",&test);

    test = getchar();
    printf("%u\r\n", test);
    return 0;
}