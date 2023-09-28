#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

#define FILE_NAME_LEN 16
#define DIR_NAME_LEN 16
#define MAX(a, b) a > b ? a : b

struct file_t
{
    char name[FILE_NAME_LEN];
    unsigned int size;
};

struct dir_t
{
    struct dir_t *parent;
    struct dir_t *subdirs;
    struct file_t *files;
    uint file_count;
    uint subdir_count;
    uint recursion;
    char name[DIR_NAME_LEN];
};

enum state
{
    getCommand,
    result
};

bool isCmd;
int got_string;
struct dir_t *cdir = NULL;

uint big_sum = 0;

int getString(FILE *ptrIn, char *str);
void parseFile(FILE *ptrIn, char *in_str, struct dir_t *dir_ptr);
void parse_ls(FILE *ptrIn, char *in_str, struct dir_t *dir_ptr);
struct dir_t *parse_cd(char *in_str, struct dir_t *dir_ptr);
void print_dir(struct dir_t *dir_ptr, bool rec);
uint getSize(struct dir_t *dir_ptr, bool rec);
uint printSizes(struct dir_t *dir_ptr, bool rec);

void print_dir(struct dir_t *dir_ptr, bool rec)
{
    for (size_t i = 0; i < dir_ptr->recursion; i++)
    {
        printf("\t");
    }
    printf("-%s: %d files, %d dir\n", dir_ptr->name, dir_ptr->file_count, dir_ptr->subdir_count);
    if (dir_ptr->file_count > 0)
    {
        for (size_t i = 0; i < dir_ptr->file_count; i++)
        {
            for (size_t i = 0; i < dir_ptr->recursion; i++)
            {
                printf("\t");
            }
            printf("file %ld %s/%s (%d)\n", i, dir_ptr->name, dir_ptr->files[i].name, dir_ptr->files[i].size);
        }
    }
    if (dir_ptr->subdir_count > 0)
    {
        for (size_t i = 0; i < dir_ptr->subdir_count; i++)
        {
            for (size_t i = 0; i < dir_ptr->recursion; i++)
            {
                printf("\t");
            }
            printf("dir %ld %s/%s\n", i, dir_ptr->name, dir_ptr->subdirs[i].name);
            if (rec)
                print_dir(&dir_ptr->subdirs[i], true);
        }
    }
}

uint getSize(struct dir_t *dir_ptr, bool rec)
{
    uint result = 0;
    if (dir_ptr->file_count > 0)
    {
        for (size_t i = 0; i < dir_ptr->file_count; i++)
        {
            result += dir_ptr->files[i].size;
        }
    }
    if (dir_ptr->subdir_count > 0)
    {
        for (size_t i = 0; i < dir_ptr->subdir_count; i++)
        {
            if (rec)
                result += getSize(&dir_ptr->subdirs[i], true);
        }
    }
    return result;
}

uint printSizes(struct dir_t *dir_ptr, bool rec)
{
    uint result = 0;
    if (dir_ptr->file_count > 0)
    {
        for (size_t i = 0; i < dir_ptr->file_count; i++)
        {
            result += dir_ptr->files[i].size;
        }
    }
    if (dir_ptr->subdir_count > 0)
    {
        for (size_t i = 0; i < dir_ptr->subdir_count; i++)
        {
            if (rec)
                result += printSizes(&dir_ptr->subdirs[i], true);
        }
    }
    for (size_t i = 0; i < dir_ptr->recursion; i++)
    {
        printf("\t");
    }
    printf("-%s -(%u) \n", dir_ptr->name, result);
    if (result <= 100000)
    {
        big_sum += result;
    }
    return result;
}

struct dir_t *parse_cd(char *in_str, struct dir_t *dir_ptr)
{
    if (strcmp(in_str, "..") == 0)
    {
        dir_ptr = dir_ptr->parent;
    }
    else
    {
        for (size_t i = 0; i < dir_ptr->subdir_count; i++)
        {
            if (strcmp(dir_ptr->subdirs[i].name, in_str) == 0)
            {
                dir_ptr->subdirs[i].subdir_count = 0;
                dir_ptr->subdirs[i].file_count = 0;
                dir_ptr = &(dir_ptr->subdirs[i]);
                break;
            }
            else if (strcmp(dir_ptr->subdirs[i].name, in_str) == 0)
            {
                dir_ptr->subdirs[i].subdir_count = 0;
                dir_ptr->subdirs[i].file_count = 0;
                dir_ptr = &(dir_ptr->subdirs[i]);
                break;
            }
        }
    }
    return dir_ptr;
}

void parseFile(FILE *ptrIn, char *in_str, struct dir_t *dir_ptr)
{
    char *test = fgets(in_str, 64, ptrIn);
    while (test != NULL)
    {
        // printf("-- parsing %s", in_str);
        char name[DIR_NAME_LEN];
        memset(name, 'x', DIR_NAME_LEN);
        if (sscanf(in_str, "$ cd %s\n", name) == 1)
        {
            dir_ptr = parse_cd(name, dir_ptr);
            test = fgets(in_str, 64, ptrIn);
        }
        else if (strcmp(in_str, "$ ls\n") == 0)
        {
            // printf("-- performing ls in %s\n", dir_ptr->name);
            parse_ls(ptrIn, in_str, dir_ptr);
        }
        else
        {
            // printf("-- no comprende \n");
            test = fgets(in_str, 64, ptrIn);
        }
    }
}

void parse_ls(FILE *ptrIn, char *in_str, struct dir_t *dir_ptr)
{
    in_str = fgets(in_str, 64, ptrIn);
    bool parsing = true;
    while (parsing)
    {
        if (in_str == NULL)
            break;
        char name[MAX(FILE_NAME_LEN, DIR_NAME_LEN)];
        uint size = 0;
        if (sscanf(in_str, "%d %s", &size, name) == 2) //! new file
        {
            // printf("found file %s/%s of size %d\n", dir_ptr->name, name, size);
            if (dir_ptr->file_count++ == 0)
                dir_ptr->files = malloc(sizeof(struct file_t));
            else
                dir_ptr->files = realloc(dir_ptr->files, sizeof(struct file_t) * dir_ptr->file_count);
            memcpy(dir_ptr->files[dir_ptr->file_count - 1].name, name, FILE_NAME_LEN);
            dir_ptr->files[dir_ptr->file_count - 1].size = size;
            in_str = fgets(in_str, 64, ptrIn);
        }
        else if (sscanf(in_str, "dir %s", name) == 1) //! new dir
        {
            // printf("found subdir %s/%s \n", dir_ptr->name, name);
            if (dir_ptr->subdir_count++ == 0)
                dir_ptr->subdirs = malloc(sizeof(struct dir_t));
            else
                dir_ptr->subdirs = realloc(dir_ptr->subdirs, sizeof(struct dir_t) * dir_ptr->subdir_count);
            memcpy(dir_ptr->subdirs[dir_ptr->subdir_count - 1].name, name, DIR_NAME_LEN);
            dir_ptr->subdirs[dir_ptr->subdir_count - 1].parent = dir_ptr;
            in_str = fgets(in_str, 64, ptrIn);
            dir_ptr->subdirs[dir_ptr->subdir_count - 1].recursion = dir_ptr->recursion + 1;
        }
        else
            parsing = false;
    }
    // print_dir(dir_ptr, false);
}

void main(int argc, char *argv[])
{
    FILE *ptrIn = fopen(argv[1], "r");

    char in_char;
    char in_str[128];

    struct dir_t root = {.name = "root", .subdir_count = 0, .file_count = 0, .recursion = 0};
    struct dir_t *cdir = &root;

    if (ptrIn == NULL)
    {
        printf("Error opening !\n");
        return;
    }

    parseFile(ptrIn, in_str, cdir);
    // printf("%s\n%s\n", in_str, cdStr);
    printf("\033[0;32m");
    printf("\nAll directories are :\n");
    printf("\033[0m");
    print_dir(&root, true);
    printf("\033[0;31m");
    printf("\nAll directories size are :\n");
    printf("\033[0m");
    printSizes(&root, true);

    printf("\033[0;31m");
    printf("\nResult is %d:\n", big_sum);
    printf("\033[0m");

    // sleep(1);
}
