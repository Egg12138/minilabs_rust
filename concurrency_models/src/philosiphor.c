#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <samephore.h>

#define PNUM 5
#define USED 1
#define FREE 0

sem_t forks[PNUM];
int left(ps) { return ps; }
int right(ps) { return (ps - 1) % PNUM; }
void tryeat();
void eat();
void think();
void done();
void philosiphor();


int ps[PNUM];
int cur_p = 0;

int
main()
{ 
        for (int i = 0; i < PNUM; i++) sem_init(&(forks[i]), 0, 1);                     
        return 0;    
}

void tryeat() {
    sem_wait(forks[left(cur_p)]);
    sem_wait(forks[right(cur_p)]);
}

void done() {
    sem_post(forks[left(cur_p)]);
    sem_post(forks[right(cur_p)]);
}

void eat() {
    printf("%d is eating\n", cur_p);
}
void think() {
    printf("%d is thinking......\n", cur_p);
}

void philosiphor() {
     
    while (1) {
        think();
        tryeat();
        eat();
        done();
    }     
}
