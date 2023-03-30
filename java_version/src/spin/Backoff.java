package src.spin;

import java.awt.geom.AffineTransform;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;

public class Backoff {
    final int minDelay, maxDelay;
    int limit;

    public Backoff(int min, int max) {
        minDelay = min;
        maxDelay = max;
        limit = minDelay;
    }

    /*
     * 当遇到获取锁时机不对（也就是有锁是可获得的，但是有其他线程也在申请拿锁的）情况， 我们进行退避。
     * 退避的时间为minDelay, maxDelay之间的随机的整数毫秒。
     */
    public void backoff() throws InterruptedException {
        int delay = ThreadLocalRandom.current().nextInt(limit);
        limit = Math.min(maxDelay, 2 * limit);
        Thread.sleep(delay);
    }
}
