
import java.lang.Thread;
package src.storage;
public class SafeBooleanMRSWRegister implements Register<boolean> {
    boolean[] s_table;
    public SafeBooleanMRSWRegister(int capacity) {
        s_table = new boolean[capacity];
    }

    @java.lang.Override
    public boolean read() {
        return s_table[java.lang.Thread.getid()];
    }

    public void write(boolean x) {
        for (int i = 0; i < s_table.length; i++) {
            s_table[i] = x;
        }
    }

}
