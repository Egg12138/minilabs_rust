package src.sync;

public class Philosopher implements  Runnable{
    private Object left;
    private  Object right;

    /*
     * <p> 实现Runnable是的他们可以作为不同线程运行 </p>
     */
    public Philosopher(Object leftfork, Object rightfork) {
        this.left = leftfork;
        this.right = rightfork;
    }

    @Override
    public void run() {
        try {
           int cnt = 10000;
           while (cnt-- >= 0) {
               act("Thinking");
               synchronized (left) {
                   act("take left");
                   synchronized (right) {
                       act("take right");
                       act("eating");
                       act("down right");
                   }
                   act("down left");
                   act("Thinking");
               }
           }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            return;
        }
    }

    private void act(String action) throws InterruptedException {
        if (action == "eating") {
            System.out.println(Thread.currentThread().getName());
        }
        Thread.sleep((int) (Math.random() * 10));
    }
}
