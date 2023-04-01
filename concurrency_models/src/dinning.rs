use core::num;
use std::{sync::{self, Mutex}, vec};

#[derive(Clone, Copy)]
pub(crate) enum Chop {
  Free,
  Hold,
}


pub(crate) struct Table {
  pub chopsticks: Vec<Mutex<Chop>>,
}

impl Table {
  pub fn new(num_chops: usize) -> Self {
    let mut chops: Vec<Mutex<Chop>> = vec![];
    for _ in 0..num_chops {
      chops.push(Mutex::new(Chop::Free));
    }
    Table { chopsticks: chops}
  }
}