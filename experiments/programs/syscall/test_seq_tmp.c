#include <unistd.h>
#include <fcntl.h>

int main() {
    int fd = open("/etc/passwd", O_RDONLY);
    if (fd != -1) close(fd);

    execl("/tmp/tmp_exec", "tmp_exec", NULL);
    return 0;
}
