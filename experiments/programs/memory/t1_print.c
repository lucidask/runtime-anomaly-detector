#include <memory.h>
#include <stdio.h>
#include <stdlib.h>

void extra_assembly()
{
    // Get the top of the stack
    // More info: https://gcc.gnu.org/onlinedocs/gcc/Basic-Asm.html
    __asm__ volatile(".intel_syntax;"
                     "pop %eax; ret;"
                     "add %bl, %al; ret;"
                     "and %edx, 0xff; ret;"
                     "xchg %ebx, %eax; ret;"
                     "xchg %eax, %edx; ret;"
                     "mov %eax, %esp; ret;"
                     "push %eax; pop %ecx; ret;"
                     "int 0x80; ret;"
                     ".att_syntax;");
}


int f(char* input)
{
    char buffer[8] = {0};
    printf("Pointer: %p\n", &buffer);

    strcpy(buffer, input);
}

// Copy a string into a small buffer.
// Goal : Inject a ROP chain to syscall write("Flag 1!") and exit(0) safely.
int main(int argc, char* argv[])
{
    if (argc != 2) {
        printf("There must be exactly 2 parameters\n");
        exit(42);
    }

    f(argv[1]);

    return 0;
}
