#!/usr/local/bin/python3.11
import os
import sys
import os.path as op
from typing import List
ver = sys.version.split()
match ver[0]:
    case '3.11.2':
        print('running in Python3.11.2') 
    case _:
        print("> 3.10 but not 3.11.2")

"""------------------"""

SYSTEMD_PID: str = '1'
PREFIX_MID: str = '├─'
PREFIX_TOP: str = '─┬─'
PREFIX_BOT: str = '└─'

class Proc:
    def __init__(self, pid, parent = None):
      self.pid = pid
      self._pname = self.__get_pname()
      self.ch_pids = self._get_children_pids()
      self.children = []
      self.isleaf = True if len(self.ch_pids) == 0 else False
      self._parent = parent

    @property
    def parent(self):
      return self._parent

    @parent.setter
    def fetch_parent(self, parent):
      assert(self.pid > parent.pid)
      self._parent = parent

    @parent.deleter
    def beroot(self):
      self._parent = None

    def add_child(self, p):
      if p.pid > self.pid:
        self.children.append(p)
        p.fetch_parent = self 
        self.isleaf = False
        print("added, ", p.pid)

    def _get_children_pids(self):
      procdir = op.join('/proc', self.pid)
      ch = op.join(procdir, f'task/{self.pid}/children')
      with open(ch, 'r') as f:
        ch = f.read().split()[1:]
      threaddir = op.join(procdir, f'task')
      ch += os.listdir(threaddir)[1:]
      assert(not self.pid in ch)
      return ch



    def name(self):
      return self._pname


    def fmt(self):
      if self.isleaf:
        self._pname = self._pname.replace('(','{').replace(')','}')
      if self.parent:
        if self.pid == max(self.parent.ch_pids):
            prefix = PREFIX_BOT
        elif min(self.parent.ch_pids):
            prefix = PREFIX_TOP
        else:
            prefix = PREFIX_MID
      else: prefix = ''
      foramtted = f'{prefix}{self._pname}({self.pid})'
      return foramtted
    # ignore Thread, consider only pid/task/pid
    # return (children_file_path, proc_name)
    def __get_pname(self):
      procdir = op.join('/proc', self.pid)
      stat = op.join(procdir, 'stat')
      if int(self.pid) > 300000:
        print(f'{self.pid=}')
      with open(stat, 'r') as f:
        pname = f.read().split()[1]
      return pname

def all_pids():
  pids = [dname for dname in os.listdir('/proc') if dname.isdigit()]
  return pids


def build_pstree(ps):

  pass

def display_rec(root):
   
  pass

   
allprocs = [Proc(pid) for pid in all_pids() ]

def build_tree(node: Proc):
  global allprocs
  if node.isleaf: 
    node.children = [] 
    # return
  else:
    for p in allprocs:
      if p.pid in node.ch_pids: 
        print(p.fmt(), end='')
        node.add_child(p)  
        build_tree(p)

def build_from_root(root: Proc):
  global allproc
  [root.add_child(p) for p in allprocs if p.pid in root.ch_pids]
  build_tree(root)

def main():
  """
    argv will be used RD_ONLY
    this program is a demo, hence argparse is unnecessary
  """
  global allprocs
  print(allprocs[0])
  for p in allprocs:
    print(p.pid)
  pstree = build_from_root(allprocs[0])  
  display_rec(pstree)
  

  return True

if __name__ == "__main__":
  main()