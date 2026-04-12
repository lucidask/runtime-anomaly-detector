#include <memory.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void backdoor(int secret_code)
{
    if (secret_code == 0x50148412) {
        printf("\n\nFlag 1 !\n");
        sleep(1);
    } else {
        printf("Psst! There is a secret nearby !\n");
    }

    exit(46);
}

int f()
{
    int   anti_malware_word1   = 0x18573054;
    char  favorite_letter      = 'R';
    char* favorite_subpassword = "Rust";
    char  password[23]         = "Default password";
    int   unused2              = 4312;

    printf("Pointer: %p\n", &password);

    printf("Please change the unsecure default password: ");
    gets(password);

    if (favorite_letter != 'R') {
        printf("Someone tries to pirate the program !\n");
        exit(43);
    }
    if (favorite_subpassword != "Rust") {
        printf("Someone tries to pirate the program !\n");
        exit(43);
    }
    if (strstr(password, "Rust") == NULL) {
        printf("Password invalid: It must contain the 'Rust' keyword.\n");
        exit(44);
    }
    if (anti_malware_word1 != 0x18573054) {
        printf("Someone tries to pirate the program !\n");
        exit(45);
    }
}

// Check if a new password is secure with a favorite subpassword.
// Goal : See TP1 description.
//
// Usage:
//      $ python t2_input.py > f
//      $ just gdb t2
//   gdb$ start < f
int main(int argc, char* argv[])
{
    if (argc != 1) {
        printf("There must be exactly 1 parameters\n");
        exit(42);
    }

    f();

    return 0;
}
