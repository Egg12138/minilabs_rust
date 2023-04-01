#include <pthread.h>
#include <stdlib.h>
#include <stdio.h>
#include <semaphore.h>
#define X -1

sem_t s;


void *child(void *arg);
int
main(int argc, char *argv[])
{
       int initval;
       if (argc == 2) {
           initval = atoi(argv[1]); 
       } else {
           initval = X;
       }
       sem_init(&s, 0, initval);
       printf("parent:begin\n");
       pthread_t c;
       int sem_value;
       pthread_create(&c, NULL, child, NULL);
       /*
        * parent: begin
        * child
        * parent: end
        * thread0 -> running
        * thread1 -> ready
        * thread0 -> waiting
        * thread1 -> running
        * thread0 -> ready
        * thread1 -> waiting
        */
       sem_getvalue(&s, &sem_value);
       printf("[Parent]before wait: s -> %d\n", sem_value);
       sem_wait(&s);
       sem_getvalue(&s, &sem_value);
       printf("[Parent]before wait: s -> %d\n", sem_value);
       printf("parent: end\n");        
       return 0;
}


void *
child(void *arg)
{
    int stmp;
    sem_getvalue(&s, &stmp);
    printf("[TC]before post: s -> %d\n", stmp);
    printf("child[]\n");
    sem_post(&s);
    sem_getvalue(&s, &stmp);
    printf("[TC]after post: s -> %d\n", stmp);
    return NULL;
}
