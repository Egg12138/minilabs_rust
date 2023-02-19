# Minilabs

## pstreers

依赖:
`clap.rs`,解析命令行参数

### 细节

先学习`pstree`过程的系统调用，下面是zsh和gitstatus等的子进程的读取思路。
秉着一切都是文件的观念操作。马上能找到`/proc/PID`.

```shell
// 上一个进程349的最后操作，
stat("/proc/343/ns/cgroup", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/343/ns/ipc", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/343/ns/mnt", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/343/ns/net", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/343/ns/pid", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/343/ns/user", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/343/ns/uts", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
close(4)                                = 0
// 349 handle start:
openat(AT_FDCWD, "/proc/349/stat", O_RDONLY) = 4
stat("/proc/349", {st_mode=S_IFDIR|0555, st_size=0, ...}) = 0
fstat(4, {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
// read all from /proc/349/stat
read(4, "349 (zsh) S 12 348 343 34819 144"..., 8192) = 314
// why read twice ?
read(4, "", 7168)                       = 0
// read cur time(how long the system has been running).
openat(AT_FDCWD, "/proc/uptime", O_RDONLY) = 5
// fstat of uptime, (fd = 5)
fstat(5, {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
read(5, "19197.92 305533.23\n", 1024)   = 19
close(5)                                = 0
// go back... read the task of proc 349
openat(AT_FDCWD, "/proc/349/task", O_RDONLY|O_NONBLOCK|O_CLOEXEC|O_DIRECTORY) = 5
fstat(5, {st_mode=S_IFDIR|0555, st_size=0, ...}) = 0
// getdents 是 bare kernel interface... 手册建议我们去看readdir(3).
getdents64(5, /* 3 entries */, 32768)   = 72
getdents64(5, /* 0 entries */, 32768)   = 0
close(5)                                = 0
stat("/proc/349/ns/cgroup", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/349/ns/ipc", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/349/ns/mnt", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/349/ns/net", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/349/ns/pid", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/349/ns/user", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
stat("/proc/349/ns/uts", {st_mode=S_IFREG|0444, st_size=0, ...}) = 0
close(4)    
```

* 在C中，我们使用`<dirent.h>`, 其中，`readdir(DIR *dirstream)`每一次调用`readdir`返回值都是一个buffer,这个buffer会不断移动游标到下一个文件.但他不会递归下去

* 使用到`strtol`, `atol`等libc函数, 前者的鲁棒性更好一点。需要注意的是前者我们需要传入`tail_ptr`用来标志**Parse成功**后的一个字符。如果啥都没解析出来，那就依旧是NULL.

1. readdir: /proc/
2. choose_proc_dirs: /proc/123..
> 这里表示子项递归我觉得不必要，因为子进程还是会在根目录下的 
> 如何完成这个树数据结构很重要
> 子进程总是小于后进程，所以我们可以标记父项
> 