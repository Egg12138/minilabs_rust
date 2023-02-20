// TODO : 优先级进程调度
// TODO: 协程
#![allow(unused_variables)]
#![feature(box_syntax)]
use std::time::Duration;
use chronon::prelude::*;

const MAX_JOBS: usize = 512;
type Cid = u64;
type Buf = Vec<Cid>;
// type Buf = Vec<Task>;

struct Task {
    pub id: Cid,
    //sttime: DataTime<UTC>,
    lifetime: Duration,
    pub priority: u8,
}
/// actually, we wanna fill the job deque with Task struct.
/// work style:
/// * push a new job
/// * examine priority 
/// * if higher than current
///     -> new jobs deque, with higher priority
///     -> puhs new job to new deque,
///     -> start 
/// * if lower,
///     -> push to the deque at the layor of the same priority
/// * when a job has done, check the deque:
/// * if empty
///     -> lower-priority deque start working
/// * else
///     -> the next 
///
/// 还需要考虑：任务长期被挤压低优先级
/// 事实上我们可以使用静态的任务优先级分层
/// 因为我们不会给太多的优先级……
/// 而且我们要实现协程
// #[derive(Parser, )]
#[derive(Debug, Clone)]
pub(crate) struct Agent {
    pub priority1: TaskDeque,
    pub priority2: TaskDeque,
    pub priority3: TaskDeque,
    pub priority4: TaskDeque,
    pub priority5: TaskDeque,
} 

impl for Agent {
    pub new() -> Self {
        let deque = TaskDeque::default();
        Agent {
            priority1: deque.clone(),
            priority2: deque.clone(),
            priority3: deque.clone(),
            priority4: deque.clone(),
            priority5: deque,
        }
    }

    pub join(&mut self, job: Task) {
        match job.priority {
            1 => { self.priority1.push() },
            2=> {  self.priority2.push()  },
            3 => {   self.priority3.push() },
            4 => {  self.priority4.push()  },
            5 => {  self.priority5.push()  },
            _ => {  },
        }
    }
    pub join_sample(&mut self, p: u8) {
        assert!(p<=5 && p>=1);
        let new_sample = Task {
            id: // random id 
            lifetime: Duration::from_seconds(1),
            priority: p,
        };
        
        printf()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TaskDeque {
    jobs: <Vec<Cid>>
    priority: u8,
//  jobs: <Vec<Task>>
}


impl Default for TaskDeque {
    fn default() -> Self {
        TaskDeque { jobs: Vec::new(), priority: 0 }
    }
}

impl<T, const N: usize> From[T; N] for TaskDeque 
where 
    T: Cid
{
    #[cfg(test)]
    fn from(slice: [T; N]) -> TaskDeque {
        TaskDeque { 
            jobs: <[T]>::into_vec(box slice),
            priority:  0
    }
}

impl for TaskDeque 
{
    pub fn new() -> Self {
        TaskDeque {
            Vec::new(),
            priority: 0,
        } 
    }

    /// 不仅需要检查Vec, 还需要检查Buff
    pub fn all_done(self) -> bool {
        self.jobs.len() == 0;
        // self.jobs.len() == 0 && buffer.is_empty()
    }
    
    pub fn len(self) -> usize { self.jobs.len()    } 


    pub fn promote(&mut self) {
        if self.priority <= 5 { self.priority += 1; }
    }

    pub fn pop(&mut self) -> Some(Cid){
        self.jobs.pop()
    }

    pub fn join(&mut self, job: Cid)  {
        self.jobs.push(job);
    }

    pub fn set_priority(&mut self, new_pri: u8) -> Self {
        self.priority = new_pri;
        self
    }  

    pub fn push(&mut self, job: Cid) -> Self {
        self.jobs.push(job);  
        self
    }
    
    pub fn  
    
}
