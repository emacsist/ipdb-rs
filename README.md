# desc

unofficial rust version for https://github.com/ipipdotnet

# device

```bash
                          ./+o+-       
                  yyyyy- -yyyyyy+      OS: Ubuntu 14.04 LTS (Trusty Tahr)
               ://+//////-yyyyyyo      Kernel: x86_64 Linux 3.13.0-167-generic
           .++ .:/++++++/-.+sss/`      Uptime: 108d 21h 40m
         .:++o:  /++++++++/:--:/-      Packages: 670
        o:+o+:++.`..```.-/oo+++++/     Shell: bash 4.3.11
       .:+o:+o/.          `+sssoo+/    Disk: 7.0T / 7.6T (97%)
  .++/+:+oo+o:`             /sssooo.   CPU: Intel Xeon E5-2603 0 @ 8x 1.8GHz
 /+++//+:`oo+o               /::--:.   GPU: ASPEED Technology, Inc. ASPEED Graphics Family (rev 21)
 \+/+o+++`o++o               ++////.   RAM: 6593MiB / 32162MiB
  .++.o+++oo+:`             /dddhhh.
       .+.o+oo:.          `oddhhhh+
        \+.++o+o``-````.:ohdhhhhh+
         `:o+++ `ohhhhhhhhyo++os:
           .o:`.syhhhhhhh/.oo++o`
               /osyyyyyyo++ooo+++/
                   ````` +oo+++o\:
                          `oo++.


```
# bench

```bash
ipdbv4                  time:   [694.10 ns 694.31 ns 694.52 ns]
                        change: [-0.1452% -0.1046% -0.0615%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
```


# perf


```bash
perf stat --event task-clock,context-switches,page-faults,cycles,instructions,branches,branch-misses,cache-references,cache-misses target/release/ipdbv4-rust

 Performance counter stats for 'target/release/ipdbv4-rust':

        710.637275 task-clock (msec)         #    0.997 CPUs utilized
               130 context-switches          #    0.183 K/sec
             1,054 page-faults               #    0.001 M/sec
     1,268,990,768 cycles                    #    1.786 GHz
     3,327,191,701 instructions              #    2.62  insns per cycle
       674,373,546 branches                  #  948.970 M/sec
            28,649 branch-misses             #    0.00% of all branches
           288,362 cache-references          #    0.406 M/sec
           134,631 cache-misses              #   46.688 % of all cache refs

       0.712643771 seconds time elapsed
```
