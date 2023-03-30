package src.sync;

import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class SimpleRWLock implements ReadWriteLock{
    int readers_num;
    boolean holdwriter;
    Condition wait4cond;
    Lock lock;
    Lock readLock, writeLock;
    public SimpleRWLock() {
       holdwriter = false;
       readers_num = 0;
       lock = new ReentrantLock();
       readLock = new ReadLock();
       writeLock = new WriteLock();
       wait4cond = lock.newCondition();
    }

    public Lock readLock() {
        return readLock;
    }

    public Lock writeLock() {
        return writeLock;
    }

    class ReadLock implements Lock {
        public void lock() {
            lock.lock();
            try {
                while (hold_writer) wait4cond.await();
                readers_num++;
            } finally {
                lock.unlock();
            }
        }

        @Override
        public void unlock() {
            lock.lock();
            try {
                readers_num--;
                if (readers_num == 0) wait4cond.signalAll();
            } finally {
                lock.unlock();
            }

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


    class WriteLock implements  Lock {
        public void lock() {
            lock.lock();
            try {
                while (readers_num > 0 || holdwriter) wait4cond.await();
                holdwriter = true;
            } finally {
                lock.unlock();
            }
        }

        @Override
        public void unlock() {

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
}

