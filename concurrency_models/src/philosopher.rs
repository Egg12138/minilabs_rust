use crate::dinning::Table;
use std::fmt::write;
use std::time::Duration;
use std::thread;
use rand::random;

#[macro_export]
macro_rules! s{
    ($st:expr) => {
        String::from($st) 
    };
    () => { String::new() }
}


#[derive(Debug, Clone)]
enum Action {
   TakingLeft,
   PutDownLeft,
   TakingRight,
   PutDownRight,
   Thinking,
   Eating, 
}

impl std::fmt::Display for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let emoji = match self {
        Self::Eating => {"🍗"},
        Self::PutDownLeft => {"🪓"},
        Self::PutDownRight => {"🔧"},
        Self::TakingLeft => {"🙋‍♂️"},
        Self::TakingRight => {"💁‍♀️"},
        Self::Thinking => {"🔞"},
      };
      let mut output = s!();
      write(&mut output, format_args!("{}\n",  emoji))
  }
}

pub(crate) struct Philosopher {
   name: String,
   left: usize,
   right: usize,
   doing: Action,
}

impl Philosopher {
    pub fn new(name: &str, left: usize, right: usize) -> Self {
        Philosopher { name: s!(name), 
                      left,
                      right, 
                      doing: Action::Thinking,
                    }
    }

    pub fn run(&mut self, chops: &mut Table,  optimes: Option<usize>) {
      if let Some(times) = optimes {
         for _ in 0..times { self.protocal(chops); } 
      } else { loop { self.protocal(chops);} } 
    }
  

    fn protocal(&mut self, chops: &mut Table) {
        self.todoact(Action::Thinking, chops);
        self.todoact(Action::TakingLeft, chops);
        self.todoact(Action::TakingRight, chops);
        self.todoact(Action::Eating, chops);
        self.todoact(Action::PutDownRight, chops);
        self.todoact(Action::PutDownLeft, chops);
    }

    fn print_act(&self) {
      println!("{} is {}", self.name, self.doing);
    }

    fn todoact(&mut self, act: Action, chops: &mut Table) {
      match act {
        Action::TakingLeft => { 
          chops.chopsticks[self.left].lock().unwrap();
          thread::sleep(Duration::from_millis(random::<u64>()));
        },
        Action::TakingRight => {
          chops.chopsticks[self.right].lock().unwrap();
          thread::sleep(Duration::from_millis(random::<u64>()));
        },
        _ => {},
      }
      self.doing = act;
      self.print_act();
    }

  
}


