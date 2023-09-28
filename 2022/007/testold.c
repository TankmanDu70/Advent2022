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
void parseFile(FILE *ptrIn, char *inStr);

void main(int argc, char *argv[])
{
    FILE *ptrIn = fopen(argv[1], "r");
    printf("?\n");

    char in_char;
    char inStr[128];

    struct dir_t root = {.name = '/', .subdir_count = 0, .file_count = 0};
    cdir = &root;

    if (ptrIn == NULL)
    {
        printf("Error opening !\n");
        return;
    }
    got_string = getString(ptrIn, inStr);
    parseFile(ptrIn, inStr);
    // printf("%s\n%s\n", inStr, cdStr);

    // sleep(1);
}

int getString(FILE *ptrIn, char *str)
{
    if (ptrIn == NULL)
    {
        printf("Error opening !\n");
        return 0;
    }
    char in_char;
    int got_char = fscanf(ptrIn, "%c", &in_char);
    if ((in_char == '$'))
        isCmd = true;
    else
        isCmd = false;

    int strlen = 0;
    do
    {
        // printf("%c", in_char);
        str[strlen++] = in_char;
        got_char = fscanf(ptrIn, "%c", &in_char);

    } while ((got_char > 0) && (in_char != '\n'));

    if (got_char < 0)
    {
        isCmd = false;
        printf("f%%\n");
        return 0;
    }
    str[strlen] = '\0';
    // if (str[strlen] == '\0')
    //     printf("\nchar strlen %c \n", str[strlen]);
    return strlen;
}

void parseFile(FILE *ptrIn, char *inStr)
{
    while (got_string > 0)
    {
        printf("%s\t\t-isCmd=%d", inStr, isCmd);
        if (CMP(inStr, cdStr)) //(memcmp(inStr, cdStr, sizeof(cdStr)) ==)
        {
            //
            // printf("there is cd %s, %s\n", inStr, cdir->name);
            if (memcmp(cdir->name, inStr + sizeof(cdStr) - 1, 1) == 0)
            {
                printf("stayed in %s\n", cdir->name);
            }
            for (size_t i = 0; i < 16; i++)
            {
                // if (cdir->subdirs[i]->name[0] == inStr[sizeof(cdStr) - 1])
                //{
                //     cdir = &cdir->subdirs[i];
                //     printf("changed dir to %s\n", cdir->name);
                // }
                /* code */
            }
        }
        else if (CMP(inStr, lsStr)) //(memcmp(inStr, lsStr, sizeof(lsStr)) ==)
        {
            parseLs(ptrIn, inStr);
        }
        else
            printf("\n");
        got_string = getString(ptrIn, inStr);
    }
}

void mkdir(struct dir_t *cd, char *name)
{
}

void parseLs(FILE *ptrIn, char *inStr)
{
    // printf("ici\n");
    do
    {
        // printf("ici\n");
        got_string = getString(ptrIn, inStr); // get the first result of ls command
        printf("\nin:%s\t-isCmd=%d\t", inStr, isCmd);

        int type_end = 0;
        int str_end = 0;
        int i = 0;
        do
        {
            if (inStr[i] == ' ')
                type_end = i; // end of the file/dir typesize;
            else if (inStr[i] == '\0')
            {
                str_end = i;
                break;
            } // end of the command line;
            printf("-i=%d",i++);
        } while ((i < sizeof(inStr)));

        char *type = malloc(type_end); // size of type/memory
        char *name = malloc(str_end - type_end);

        memcpy(type, inStr, type_end);
        memcpy(name, inStr + type_end, str_end - type_end);

        int size = atoi(type);
        printf("type_end %d str_end %d ", type_end, str_end);
        printf("type: %s name: %s size= %d ", type, name, size);

        if (CMP(inStr, dirStr))
        {
            // printf("ici\n");
            //  printf("new dir %d\n", cdir->subdir_count);

            cdir->subdirs[cdir->subdir_count] = malloc(sizeof(struct dir_t));
            cdir->subdirs[cdir->subdir_count]->name[0] = inStr[type_end + 1];
            cdir->subdirs[cdir->subdir_count]->file_count = 0;
            cdir->subdirs[cdir->subdir_count]->subdir_count = 0;
            cdir->subdirs[cdir->subdir_count]->parent = cdir;
            cdir->subdir_count++;
            memset(cdir->files, 0, 15);
            memset(cdir->subdirs, 0, 15);
            for (size_t c = 0; c < 16; c++)
                cdir->files[c] = malloc(sizeof(struct file_t));
            printf(" -> new dir %c, dircount of %s is %d\n", inStr[type_end + 1], cdir->name, cdir->subdir_count);
        }
        else if (size > 0)
        {
            printf("new file %d\n", cdir->file_count);
            cdir->files[cdir->file_count] = malloc(sizeof(struct file_t));
            memcpy(cdir->files[cdir->file_count]->name, inStr + type_end + 1, str_end - type_end);
            cdir->files[cdir->file_count]->size = size;
            printf("%dth file %s:%d bytes\n", cdir->file_count, cdir->files[cdir->file_count]->name, size);
            cdir->file_count++;
        }
        else
            printf("failed to parse %s, size is %d - isCmd=%d\n", inStr, size, isCmd);
        free(type);
        free(name);
    } while (!isCmd);
}