# desc

unofficial rust version for https://github.com/ipipdotnet

Any contributions are welcome ~

# test env device

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

## vs java version

> JDK java version "1.8.0_91"

```xml
        <dependency>
            <groupId>net.ipip</groupId>
            <artifactId>ipdb</artifactId>
            <version>1.1.3</version>
        </dependency>
```

```java
    @Test
    public void testJavaIPDB() {
        for (int i = 0; i < 100000; i++) {
            IPDB.find("113.67.126.164");
        }
        final long start = System.currentTimeMillis();
        long sum = 0;
        for (int i = 0; i < 1000000; i++) {
            final String[] v = IPDB.find("113.67.126.164");
            if (v != null) {
                sum += v.length;
            }
        }
        System.out.println("cost " + (System.currentTimeMillis() - start) + " ms, total " + (sum / 3));
    }
```

```rust
    use ipdb_rs as ipdb;
    let ipadd = ipdb::find("113.67.126.164", "CN");
    println!("ip addr {:?}", ipadd);
    let now = Instant::now();
    let mut sum: usize = 0;
    for i in 0..1000000 {
        if let Ok(v) = ipdb::find("113.67.126.164", "CN") {
            sum += v.len();
        }
    }
    println!("cost {} ms, total {}", now.elapsed().as_millis(), sum / 3);
```

```bash
Rust 
cost 657 ms, total 1000000

Java
cost 847 ms, total 1000000
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