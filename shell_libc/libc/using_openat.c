#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
//#include <pthread.h>

int
main(void)
{

        int fd = popen("/proc/6582/fd/3", O_WRONLY);
        if (fd < 0) {
            perror("open");
            return -1;
        } 

        write(fd, "hello", 5);
        return 0;

}
