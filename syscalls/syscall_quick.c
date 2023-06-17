#include <stdio.h>
#include <syscall.h>
#include <sys/types.h>
#include <pthread.h>
#include <assert.h>


#ifndef gettid
#define gettid() syscall(SYS_gettid)
#endif

typedef struct myarg_t {
    int first;
} myarg_t;

void *threadtask(void *arg) {
        myarg_t *m = (myarg_t *)arg;
        printf("thread '%d' ", m->first);
        int tid = gettid(); 
        int pid = syscall(SYS_getpid);

        printf("[%u]: process<%d>\n", tid, pid);
        return NULL;

}

int
main()
{
        
        int s;

        pthread_attr_t attr;
        s = pthread_attr_init(&attr);
        assert(s >= 0);
        
        for (int i = 0; i < 5; i++) {
            pthread_t p;
            myarg_t arg;
            arg.first = i;
            s = pthread_create(&p,  &attr, threadtask, &arg);
            pthread_join(p, NULL);
        }

        int tid = gettid(); 
        int pid = syscall(SYS_getpid);

        printf("parent: [thread %u]: process<%d>\n", tid, pid);

        return 0;
}
