This repository demonstrates a regression during upgrade rustc 1.65 -> 1.67

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


