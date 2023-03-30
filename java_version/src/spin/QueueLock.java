package src.spin;

import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;

public class QueueLock implements Lock {
    private int[] flag;
    /* **ThreadLocal可以保证这是线程独有的独立副本， 不会有一致性通信问题**
     * tail 作为总索引，会超过capacity的， 我们要new_tail & size得到索引，这个返回值就是mySlotIndex, 保证局部性
     * 而用AtomicInteger，也是为了使用其原子的递增方法
     */
    ThreadLocal<Integer>mySlotIndex = new ThreadLocal<Integer> () {
        private Integer initaliValue() { return 0; }
    };
    public AtomicInteger tail_idx;
    volatile boolean[] shared_flag;
    int size;
    public QueueLock(int capacity) {
        size = capacity;
        tail_idx = new AtomicInteger(0);
        shared_flag = new boolean[capacity];
        shared_flag[0] = true; //init: thread0 holds the lock
    }

    @Override
    public void lock() {
        int slot = tail_idx.getAndIncrement() % size;
        mySlotIndex.set(slot);
        while (!shared_flag[slot]) {};
    }

    @Override
    public void unlock() {
        int slot = mySlotIndex.get();
        shared_flag[slot] = false;
        shared_flag[(slot + 1) % size]  = true; // wake up the next thread.
    }

    @Override
    public void lockInterruptibly() throws InterruptedException {

    }

    @Override
    public boolean tryLock() {
        return false;
    }

    @Override
    public boolean tryLock(long time, TimeUnit unit) throws InterruptedException {
        return false;
    }


    @Override
    public Condition newCondition() {
        return null;
    }
}
