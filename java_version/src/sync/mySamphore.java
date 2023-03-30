package src.sync;

import java.awt.image.renderable.ContextualRenderedImageFactory;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class mySamphore {
    final int capacity;
    int criticalThreadNums;
    Lock lock;
    Condition wait4cond;

    public mySamphore(int cap) {
        capacity = cap;
        lock = new ReentrantLock();
        wait4cond = lock.newCondition();
        criticalThreadNums = 0;
    }

    public void acquire() {
        lock.lock();
        try {
            while (criticalThreadNums == capacity) wait4cond.await();  // full
            criticalThreadNums++;
        }  catch (InterruptedException e) {
            System.out.println(e);
        }
        finally {
            lock.unlock();
        }
    }

    public void release() {
        lock.lock();
        try {
            criticalThreadNums--;
            wait4cond.signalAll();
        } finally {
            lock.unlock();
        }
    }
}


