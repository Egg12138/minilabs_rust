package src.sync;

public class DiningProblem {
   public  static  void main(String[] args) throws Exception {
       Philosopher[] ps = new Philosopher[5];
       Object[] forks = new Object[ps.length];

       for (int i = 0; i < forks.length; i++) forks[i] = new Object();
       for (int i = 0; i < ps.length; i++) {
           Object left = forks[i];
           Object right = forks[(i+1) % forks.length];
           if (i == ps.length - 1) ps[i] = new Philosopher(right, left);
           else ps[i] = new Philosopher(left, right);
           //Thread t = new Thread(ps[i], Integer.toString(i+1));
           //t.start();
       }
   }
}
