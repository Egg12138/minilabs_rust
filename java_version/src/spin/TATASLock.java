package src.spin;

import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.locks.Lock;

public class TATASLock extends AtomicBoolean {
    AtomicBoolean state = new AtomicBoolean(false);
    public void lock() {
       while (true) {
           while(state.get()) {} // Locked. keeping trying
           if (!state.getAndSet(true)) return; // Previous state: unlocked(false) , and now set locked(true)
       }
    }
    public  void unlock() {
       state.set(false);
    }
}
