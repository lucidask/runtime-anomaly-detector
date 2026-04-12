#include <memory.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

static const char* FLAG = "/usr/bin/date";

void backdoor()
{
    printf("\n\nFlag 1 !\n");
    sleep(1);
}

int f(char* arg1, bool is_shellcode)
{
    char  default_isbn[15];
    char* isbn_prefix = "ISBN_";
    char  isbn[20]    = "";

    printf("Pointer: %p\n", &isbn_prefix);
    strcpy(default_isbn, "ISBN_731-23425");

    strncpy(isbn, arg1, sizeof(isbn) * (sizeof(int) + 1));

    if (strstr(default_isbn, "ISBN_731-23425") == 0) {
        printf("Someone tries to pirate the program !\n");
        exit(44);
    }

    if (isbn_prefix != "ISBN_") {
        printf("Someone tries to pirate the program !\n");
        exit(45);
    }

    char* prefix_input = strstr(isbn, isbn_prefix);
    if (prefix_input != isbn) {
        printf("The new ISBN provided must be prefixed with 'ISBN ' !\n");
        exit(46);
    }

    int* ret = __builtin_return_address(0);
    if (((unsigned int) ret & 0xffff0000) == 0xffff0000 && !is_shellcode) {
        printf("Return address cannot be on the stack");
        exit(47);
    }
}

// Take a book's ISBN and store it for later.
// Goal : See TP description
//
// Usage:
//      $ python t1_input.py 0xffffffff > f
//      $ just gdb-f t1 f
//   gdb$ start
int main(int argc, char* argv[])
{
    if (argc != 3) {
        printf("There must be exactly 3 parameters\n");
        printf("./exec payload shellcode=true\n");
        printf("./exec payload shellcode=false\n");
        exit(42);
    }

    bool is_shellcode = false;
    if (strcmp(argv[2], "shellcode=true") == 0) {
        is_shellcode = true;
    } else if (strcmp(argv[2], "shellcode=false") == 0) {
        is_shellcode = false;
    } else {
        printf("Bad usage of the CLI");
        exit(43);
    }

    f(argv[1], is_shellcode);

    printf("\n\nFlag 2 !\n");
    sleep(1);

    return 0;
}
