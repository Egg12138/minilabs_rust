package src.spin;

import java.util.concurrent.atomic.AtomicBoolean;

public class TASLock extends AtomicBoolean {
    AtomicBoolean state = new AtomicBoolean(false);
    public void lock() {
        while (state.getAndSet(true)) { //waiting...
        }
    }
    public  void unlock() {
        state.set(false);
    }
}
