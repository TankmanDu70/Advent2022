#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

struct file_t
{
    char name[16];
    unsigned int size;
};

struct dir_t
{
    struct dir_t *parent;
    struct dir_t *subdirs[16];
    struct file_t *files[16];
    int file_count;
    int subdir_count;
    char *name[16];
};

enum state
{
    getCommand,
    result
};

const char cdStr[] = "$ cd ";
const char lsStr[] = "$ ls";
const char dirStr[] = "dir ";

bool isCmd;
int got_string;
struct dir_t *cdir = NULL;

#define CMP(str1, str2) memcmp(str1, str2, sizeof(str2) - 1) == 0

int getString(FILE *ptrIn, char *str);
void parseFile(FILE *ptrIn, char *in_str);

void main(int argc, char *argv[])
{
    FILE *ptrIn = fopen(argv[1], "r");
    printf("?\n");

    char in_char;
    char in_str[128];

    struct dir_t root = {.name = '/', .subdir_count = 0, .file_count = 0};
    cdir = &root;

    if (ptrIn == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    char *test = fgets(in_str, 64, ptrIn);
    parseFile(ptrIn, in_str);
    // printf("%s\n%s\n", in_str, cdStr);

    // sleep(1);
}

void parseString() {}

void parseFile(FILE *ptrIn, char *in_str)
{
    while (got_string > 0)
    {
        printf("%s\t\t-isCmd=%d", in_str, isCmd);
        if (CMP(in_str, cdStr)) //(memcmp(in_str, cdStr, sizeof(cdStr)) ==)
        {
            //
            // printf("there is cd %s, %s\n", in_str, cdir->name);
            if (memcmp(cdir->name, in_str + sizeof(cdStr) - 1, 1) == 0)
            {
                printf("stayed in %s\n", cdir->name);
            }
            for (size_t i = 0; i < 16; i++)
            {
                // if (cdir->subdirs[i]->name[0] == in_str[sizeof(cdStr) - 1])
                //{
                //     cdir = &cdir->subdirs[i];
                //     printf("changed dir to %s\n", cdir->name);
                // }
                /* code */
            }
        }
        else if (CMP(in_str, lsStr)) //(memcmp(in_str, lsStr, sizeof(lsStr)) ==)
        {
            parseLs(ptrIn, in_str);
        }
        else
            printf("\n");
        got_string = getString(ptrIn, in_str);
    }
}

void mkdir(struct dir_t *cd, char *name)
{
}

void parseLs(FILE *ptrIn, char *in_str)
{
    // printf("ici\n");
    do
    {
        // printf("ici\n");
        got_string = getString(ptrIn, in_str); // get the first result of ls command
        printf("\nin:%s\t-isCmd=%d\t", in_str, isCmd);

        int type_end = 0;
        int str_end = 0;
        int i = 0;
        do
        {
            if (in_str[i] == ' ')
                type_end = i; // end of the file/dir typesize;
            else if (in_str[i] == '\0')
            {
                str_end = i;
                break;
            } // end of the command line;
            printf("-i=%d", i++);
        } while ((i < sizeof(in_str)));

        char *type = malloc(type_end); // size of type/memory
        char *name = malloc(str_end - type_end);

        memcpy(type, in_str, type_end);
        memcpy(name, in_str + type_end, str_end - type_end);

        int size = atoi(type);
        printf("type_end %d str_end %d ", type_end, str_end);
        printf("type: %s name: %s size= %d ", type, name, size);

        if (CMP(in_str, dirStr))
        {
            // printf("ici\n");
            //  printf("new dir %d\n", cdir->subdir_count);

            cdir->subdirs[cdir->subdir_count] = malloc(sizeof(struct dir_t));
            cdir->subdirs[cdir->subdir_count]->name[0] = in_str[type_end + 1];
            cdir->subdirs[cdir->subdir_count]->file_count = 0;
            cdir->subdirs[cdir->subdir_count]->subdir_count = 0;
            cdir->subdirs[cdir->subdir_count]->parent = cdir;
            cdir->subdir_count++;
            memset(cdir->files, 0, 15);
            memset(cdir->subdirs, 0, 15);
            for (size_t c = 0; c < 16; c++)
                cdir->files[c] = malloc(sizeof(struct file_t));
            printf(" -> new dir %c, dircount of %s is %d\n", in_str[type_end + 1], cdir->name, cdir->subdir_count);
        }
        else if (size > 0)
        {
            printf("new file %d\n", cdir->file_count);
            cdir->files[cdir->file_count] = malloc(sizeof(struct file_t));
            memcpy(cdir->files[cdir->file_count]->name, in_str + type_end + 1, str_end - type_end);
            cdir->files[cdir->file_count]->size = size;
            printf("%dth file %s:%d bytes\n", cdir->file_count, cdir->files[cdir->file_count]->name, size);
            cdir->file_count++;
        }
        else
            printf("failed to parse %s, size is %d - isCmd=%d\n", in_str, size, isCmd);
        free(type);
        free(name);
    } while (!isCmd);
}