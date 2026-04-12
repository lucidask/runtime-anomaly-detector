#include <memory.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void backdoor()
{
    printf("\n\nFlag 2 !\n");
    sleep(1);
    exit(46);
}

int f(char* arg1)
{
    int  anti_malware_word1 = 0x44454647;
    char buffer[4]          = {0};
    char car_name[20]       = "";
    int  anti_malware_word2 = 0x17181920;

    printf("Pointer: %p\n", &anti_malware_word2);

    if (strlen(arg1) <= 10 || strlen(arg1) > 40) {
        printf("The name of the car to sell must not be too small or too big\n");
        exit(43);
    }
    strcpy(car_name, arg1);

    if (anti_malware_word1 != 0x44454647) {
        printf("Someone tries to pirate the program !\n");
        exit(44);
    }
    if (anti_malware_word2 != 0x17181920) {
        printf("Someone tries to pirate the program !\n");
        exit(45);
    }

    if (strcmp(buffer, "4009GFED") == 0) {
        printf("\n\nFlag 1 !\n");
        sleep(1);
    }
}

// Sell a vehicle to someone with a proper name.
// Goal : See TP1 description.
//
// Usage:
//      $ python t1_input.py > f
//      $ just gdb-f t1 f
//   gdb$ start
int main(int argc, char* argv[])
{
    if (argc != 2) {
        printf("There must be exactly 2 parameters\n");
        exit(42);
    }

    f(argv[1]);

    return 0;
}
