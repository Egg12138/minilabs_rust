package src.sync;

import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class LockedQueue<T> {
    final Lock lock = new ReentrantLock();
    final Condition cond_notFull = lock.newCondition();
    final Condition cond_notEmpty = lock.newCondition();
    final T[] items;
    int tail, head, cnt;
    public LockedQueue(int capacity) {
       items = (T[]) new Object[capacity];
    }

    public void enq(T x) throws InterruptedException {
        lock.lock();
        try {
            while (cnt == items.length) cond_notFull.await();
            // after the condition was satisfied:
            items[tail] = x;
            if (++tail == items.length) tail = 0;
            ++cnt;
            cond_notEmpty.signal();
        } finally {
           lock.unlock();
        }
    }

    public T deq() throws InterruptedException {
       lock.lock();
       try {
           while (cnt == 0) cond_notEmpty.await();
           // read first
           T x = items[head];
           if (++head == items.length) head = 0;
           --cnt;
           cond_notFull.signal();
           return x;
       } finally {
          lock.unlock();
       }
    }
}
