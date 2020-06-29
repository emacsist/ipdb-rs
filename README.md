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
ipdbv4                  time:   [893.27 ns 893.40 ns 893.54 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
```


# perf

```bash
 Performance counter stats for 'target/release/ipdbv4-rust':

        909.653020 task-clock (msec)         #    0.998 CPUs utilized
               122 context-switches          #    0.134 K/sec
             1,942 page-faults               #    0.002 M/sec
     1,635,398,022 cycles                    #    1.798 GHz
     4,113,793,740 instructions              #    2.52  insns per cycle
       866,667,193 branches                  #  952.745 M/sec
           495,260 branch-misses             #    0.06% of all branches
           409,150 cache-references          #    0.450 M/sec
           211,979 cache-misses              #   51.810 % of all cache refs

       0.911265911 seconds time elapsed
```
