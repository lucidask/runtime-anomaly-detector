#include <memory.h>
#include <stdio.h>
#include <stdlib.h>

void extra_assembly()
{
    // Get the top of the stack
    // More info: https://gcc.gnu.org/onlinedocs/gcc/Basic-Asm.html
    __asm__ volatile(".intel_syntax;"
                     "mov %ebp, %esp; pop %ebx; ret;"
                     "pop %ebx; ret;"
                     "int 0x80; ret;"
                     "add %ebx, %eax; pop %ecx; ret;"
                     "xor %ecx, %ecx; ret;"
                     "sub %eax, 8; ret;"
                     "mov %edx, %eax; ret;"
                     "xchg %ebp, %ebx; push %ebp; ret;"
                     "dec %eax; ret;"
                     "mov %eax, [%ebx]; ret;"
                     ".att_syntax;");
}


int f(char* input)
{
    char buffer[8] = {0};
    printf("Pointer: %p\n", &buffer);

    strcpy(buffer, input);
}

// Copy a string into a small buffer.
// Goal : Inject a ROP chain to syscall execve("/bin/sh", NULL, NULL).
int main(int argc, char* argv[])
{
    if (argc != 2) {
        printf("There must be exactly 2 parameters\n");
        exit(42);
    }

    f(argv[1]);

    return 0;
}
