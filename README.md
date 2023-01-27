This repository demonstrates a regression during upgrade rustc 1.65 -> 1.67

UPDATE: I tested with 1.66.1 and the issue is not seen there.
The segv is present on arm and x86 targets.

```

$ rustup show
Default host: aarch64-unknown-linux-gnu
rustup home:  /home/ubuntu/.rustup

installed toolchains
--------------------

stable-aarch64-unknown-linux-gnu (default)
1.65-aarch64-unknown-linux-gnu

active toolchain
----------------

stable-aarch64-unknown-linux-gnu (default)
rustc 1.67.0 (fc594f156 2023-01-24)


$ cargo run
warning: unused import: `prettytable::format`
 --> src/main.rs:6:9
  |
6 |     use prettytable::format;
  |         ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `test-table` (bin "test-table") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/test-table`
Segmentation fault (core dumped)
$
$ gdb --args ./target/debug/test-table 
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04.1) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "aarch64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from ./target/debug/test-table...
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/ubuntu/test-table/target/debug/test-table.
Use `info auto-load python-scripts [REGEXP]' to list them.
(gdb) r
Starting program: /home/ubuntu/test-table/target/debug/test-table 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/aarch64-linux-gnu/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
alloc::vec::Vec<T,A>::len (self=0xaaaaaab84000)
    at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs:2057
2057	/rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs: No such file or directory.
(gdb) bt
#0  alloc::vec::Vec<T,A>::len (self=0xaaaaaab84000)
    at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs:2057
#1  0x0000aaaaaaab1df8 in prettytable::row::Row::len (self=0xaaaaaab84000) at src/row.rs:32
#2  0x0000aaaaaaab46fc in prettytable::TableSlice::get_column_num (self=0xffffffffeb70) at src/lib.rs:77
#3  0x0000aaaaaaab4844 in prettytable::TableSlice::get_all_column_width (self=0xffffffffeb70) at src/lib.rs:119
#4  0x0000aaaaaaab497c in prettytable::TableSlice::__print (self=0xffffffffeb70, out=0xffffffffe998, 
    f=0xaaaaaab647e000) at src/lib.rs:142
#5  0x0000aaaaaaab5534 in prettytable::TableSlice::print (self=0xffffffffeb70, out=0xffffffffe998)
    at src/lib.rs:166
#6  0x0000aaaaaaab574c in prettytable::TableSlice::print_tty (self=0xffffffffeb70, force_colorize=false)
    at src/lib.rs:185
#7  0x0000aaaaaaab58d0 in prettytable::TableSlice::printstd (self=0xffffffffeb70) at src/lib.rs:200
#8  0x0000aaaaaaab5bf8 in prettytable::Table::printstd (self=0xffffffffeb70) at src/lib.rs:414
#9  0x0000aaaaaaab1ba4 in test_table::main () at src/main.rs:29
(gdb) 

$ rustup default 1.65-aarch64-unknown-linux-gnu
info: using existing install for '1.65-aarch64-unknown-linux-gnu'
info: default toolchain set to '1.65-aarch64-unknown-linux-gnu'

  1.65-aarch64-unknown-linux-gnu unchanged - rustc 1.65.0 (897e37553 2022-11-02)

$ cargo run
   Compiling libc v0.2.139
   Compiling rustc-serialize v0.3.24
   Compiling byteorder v1.4.3
   Compiling encode_unicode v0.3.6
   Compiling lazy_static v0.2.11
   Compiling term v0.4.6
   Compiling unicode-width v0.1.10
   Compiling memchr v1.0.2
   Compiling atty v0.2.14
   Compiling csv v0.15.0
   Compiling prettytable-rs v0.6.7
   Compiling test-table v0.1.0 (/home/ubuntu/test-table)
warning: unused import: `prettytable::format`
 --> src/main.rs:6:9
  |
6 |     use prettytable::format;
  |         ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `test-table` (bin "test-table") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 2.48s
     Running `target/debug/test-table`
+-----------+-------------+---------+--------------+----------+------------+--------------+
| ServiceID | ServiceName | Vlan ID | Total Ports# | Locked # | Unlocked # | Inconsistent |
+===========+=============+=========+==============+==========+============+==============+
| 1         | Service #1  | 42      | 10000        | 10000    | 9999       |              |
+-----------+-------------+---------+--------------+----------+------------+--------------+
$ 



===== switched over to a dedicated debug host - an lxd container with a fresh ubuntu 20.04 install,
with rust installed and "apt-get install build-essential" done, for cc.

Starting the executable under gdb, I can see the "rows" seems completely out of whack:

ubuntu@rust-test:~/test-rustc-1.65-1.67-segv$ gdb ./target/debug/test-table 
GNU gdb (Ubuntu 9.2-0ubuntu1~20.04.1) 9.2
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from ./target/debug/test-table...
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/ubuntu/test-rustc-1.65-1.67-segv/target/debug/test-table.
Use `info auto-load python-scripts [REGEXP]' to list them.
(gdb) b prettytable::TableSlice::get_all_column_width
Breakpoint 1 at 0x14ca6: file src/lib.rs, line 119.
(gdb) r
Starting program: /home/ubuntu/test-rustc-1.65-1.67-segv/target/debug/test-table 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, prettytable::TableSlice::get_all_column_width (self=0x7fffffffe260) at src/lib.rs:119
119	        let colnum = self.get_column_num();
(gdb) p self
$1 = (*mut prettytable::TableSlice) 0x7fffffffe260
(gdb) p *self
$2 = prettytable::TableSlice {format: 0x0, titles: 0x8, rows: &[prettytable::row::Row] {data_ptr: 0x555555619ba0, length: 93824993041104}}
(gdb) s
prettytable::TableSlice::get_column_num (self=0x7fffffffe260) at src/lib.rs:75
75	        let mut cnum = 0;
(gdb) 
76	        for r in self.rows {
(gdb) 
core::slice::iter::<impl core::iter::traits::collect::IntoIterator for &[T]>::into_iter (self=...)
    at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/slice/iter.rs:24
24	/rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/slice/iter.rs: No such file or directory.
(gdb) n
25	in /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/slice/iter.rs
(gdb) n
prettytable::TableSlice::get_column_num (self=0x7fffffffe260) at src/lib.rs:77
77	            let l = r.len();
(gdb) p r
$3 = (*mut prettytable::row::Row) 0x555555619ba0
(gdb) p r.len()
$4 = 184683593789
(gdb) n
