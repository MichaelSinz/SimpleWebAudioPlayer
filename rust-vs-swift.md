# Rust vs. Swift & Intel vs Apple Silicon

## Background

This project began a long time ago as a personal fun exploration into audio visualization.  I needed a tool to process my extensive music library (over 50,000 audio files) and generate stereo waveform visualizations for each track.  These visualizations would later be integrated into a custom web-based music player I had developed.

### Initial Swift Implementation

I initially chose Swift for this project for several reasons:
- Swift is the primary language for Apple's ecosystem
- It has built-in access to Apple's powerful media processing libraries
- I wanted to see how Swift did as a more modern programming language

The Swift implementation performed reasonably well on my Intel MacBook Pro (2019, i9 processor), processing the entire library in approximately 29 minutes.  When I later upgraded to an M4 Max MacBook Pro, the same Swift code ran in about 16 minutes - a significant improvement due to the superior hardware performance of the Apple Silicon.

### The Rust Alternative

I became interested in exploring Rust as an alternative implementation:
- Rust could run on my Linux workstation
- Rust has been gaining popularity for performance-critical applications
- Rust's audio processing libraries have matured considerably
- I wanted to compare these languages in a real-world scenario
- It provided an opportunity to use Rust for a fun project

This led me to reimplement the same functionality in Rust, ensuring both versions produced effectively identical outputs for comparative testing.

People who know me, know what happened next.  I benchmarked these implementations.  I actually had done that with the Swift code and tuned its audio buffer sizes and processing loop to get the best performance I could.  So, that is what I did with the Rust version.

### The Benchmarking Process

Once both implementations were complete, I conducted thorough performance testing. For the Swift version, I had already optimized audio buffer sizes and processing loops to achieve optimal performance. I applied similar optimization techniques to the Rust implementation to ensure a fair comparison.

### The Unexpected Discovery (and dope-slap)

My benchmarking journey took an unexpected turn when I discovered something remarkable about the Rust implementation on my M4 Max MacBook Pro.

When I initially ran performance tests on the M4, the Rust version already showed impressive results, slightly outpacing the Swift implementation. But during my investigation, I realized that my Rust environment was still configured for Intel architecture. The Rust compiler had been migrated from my old Intel machine and was generating x86_64 binaries that were being executed through Rosetta 2, Apple's dynamic binary translator for running Intel applications on Apple Silicon.

In other words, **the Rust code was running under emulation and still outperforming the native Swift implementation**.

This had me giving myself a massive "dope-slap" and some additional work of now doing this correctly.  But it also brought home a realization of just how good Apple's hardware and Rosetta software are.

This discovery was genuinely surprising, as conventional wisdom suggests that native code should always outperform emulated code. The fact that x86_64 Rust code running through Rosetta's translation layer could match or exceed the performance of native ARM64 Swift code is a testament to both:

1. The efficiency of Rust as a systems programming language
2. The remarkable capabilities of Apple's Rosetta 2 translation technology

After reinstalling Rust with native Apple Silicon support, performance improved even further, with the native ARM64 Rust implementation now significantly outperforming both the emulated Rust code and the native Swift version on Apple Silicon.

These improvements held true for both small single-file operations and large multi-file, highly parallel test cases.

**Architecture Matters More Than Language**: The performance difference between Intel and Apple Silicon is generally larger than the difference between Rust and Swift on the same architecture.  The M4 is substantially faster than the Intel i9 regardless of programming language.  So much so that even the code compiled for the Intel processor ran faster on the Apple Silicon than it did running on its native Intel hardware!

> **Note:** All files were stored on the internal high-speed SSD.  The Rust version sustained ~650MB/s read rates, while the Swift version averaged around 320MB/s on the Apple M4 but noticeably lower rates when running on slower machines.  On systems with spinning media, both would perform *much* slower.  (Actually tested on my workstation but the results are not interesting as they are I/O dominated and the CPUs were relatively idle).

---

## Performance

The following sections present visualizations of various performance metrics collected during testing. Each graph shows relative performance across platforms, normalized so the largest value equals 100%.

<style>
  table {border-collapse:collapse;width:100%;}
  td {padding:1px 0.25em 1px 0.25em;text-wrap:nowrap;}
  th {text-align:left;}
  th span {font-size:90%;}
  th span span {float:right;font-size:80%;}
  td.bar {width:99%;}
  td.num {text-align:right;}
</style>

### Wall Clock Time (Real Time)
- **real**

This metric represents the actual elapsed time from start to finish. It includes all computation, I/O operations, and scheduling delays. Lower values indicate faster overall completion.

<table>
<tr><th>Platform</th><th colspan="2">Time <span>(seconds)<span>[shorter is faster]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">531.35</td><td class="bar"><div style="background:#808;width:22.99%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">952.81</td><td class="bar"><div style="background:#088;width:41.23%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">2,310.94</td><td class="bar"><div style="background:#800;width:100%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">1,763.86</td><td class="bar"><div style="background:#080;width:76.33%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">862.21</td><td class="bar"><div style="background:#333;width:37.31%;">|</div></td></tr>
<tr><td>i9 Linux Desktop Rust</td><td class="num">1,372.13</td><td class="bar"><div style="background:#008;width:59.38%;">|</div></td></tr>
</table>

### User Time (CPU Time)
- **user**

This metric shows the total CPU time spent executing user-level code across all cores. Lower values indicate more efficient processing. The M4 Rust implementation shows significantly less CPU usage than other configurations.

<table>
<tr><th>Platform</th><th colspan="2">User Time <span>(seconds)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">8,233.19</td><td class="bar"><div style="background:#808;width:23.64%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">13,871.20</td><td class="bar"><div style="background:#088;width:39.84%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">34,821.80</td><td class="bar"><div style="background:#800;width:100%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">25,992.10</td><td class="bar"><div style="background:#080;width:74.64%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">13,080.89</td><td class="bar"><div style="background:#333;width:37.57%;">|</div></td></tr>
<tr><td>i9 Linux Desktop Rust</td><td class="num">21,735.25</td><td class="bar"><div style="background:#008;width:62.42%;">|</div></td></tr>
</table>

### System Time
- **sys**

This graph displays the CPU time spent in the kernel on behalf of the process, including operations like file I/O and memory allocation. Lower values indicate less kernel overhead. Swift on Intel i9 shows notably high system time, while Rust implementations generally have less kernel overhead.

<table>
<tr><th>Platform</th><th colspan="2">System Time <span>(seconds)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">71.93</td><td class="bar"><div style="background:#808;width:10.34%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">357.92</td><td class="bar"><div style="background:#088;width:51.45%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">224.28</td><td class="bar"><div style="background:#800;width:32.24%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">695.70</td><td class="bar"><div style="background:#080;width:100%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">106.17</td><td class="bar"><div style="background:#333;width:15.26%;">|</div></td></tr>
<tr><td>i9 Linux Desktop Rust</td><td class="num">115.48</td><td class="bar"><div style="background:#008;width:16.60%;">|</div></td></tr>
</table>

### Maximum Resident Set Size
- **maximum resident set size**

This metric represents the peak amount of physical memory allocated during execution. The x86_64 binary running under Rosetta on the M4 uses the most memory, while the Linux implementation is the most memory efficient.

<table>
<tr><th>Platform</th><th colspan="2">Maximum Resident Set Size <span>(megabytes)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">310</td><td class="bar"><div style="background:#808;width:73.52%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">303</td><td class="bar"><div style="background:#088;width:71.88%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">230</td><td class="bar"><div style="background:#800;width:54.65%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">286</td><td class="bar"><div style="background:#080;width:67.80%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">421</td><td class="bar"><div style="background:#333;width:100%;">|</div></td></tr>
<tr><td>i9 Linux Desktop Rust</td><td class="num">108</td><td class="bar"><div style="background:#008;width:25.60%;">|</div></td></tr>
</table>

### Voluntary Context Switches
- **voluntary context switches**

Voluntary context switches occur when a process willingly gives up the CPU, typically when waiting for I/O. The Swift implementations show dramatically higher voluntary context switches, suggesting they use more asynchronous I/O operations than the Rust versions.

<table>
<tr><th>Platform</th><th colspan="2">Voluntary Context Switches <span>(thousands)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">163</td><td class="bar"><div style="background:#808;width:1.94%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">7,787</td><td class="bar"><div style="background:#088;width:93.24%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">166</td><td class="bar"><div style="background:#800;width:1.99%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">8,352</td><td class="bar"><div style="background:#080;width:100%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">160</td><td class="bar"><div style="background:#333;width:1.91%;">|</div></td></tr>
<tr><td>i9 Linux Desktop Rust</td><td class="num">214</td><td class="bar"><div style="background:#008;width:2.57%;">|</div></td></tr>
</table>

### Involuntary Context Switches
- **involuntary context switches**

These context switches happen when the OS preemptively switches out a process. Higher numbers can indicate CPU contention or scheduler pressure. The Swift implementation on M4 shows an extremely high number, suggesting significant preemption.

<table>
<tr><th>Platform</th><th colspan="2">Involuntary Context Switches <span>(thousands)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">3,562</td><td class="bar"><div style="background:#808;width:8.15%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">43,719</td><td class="bar"><div style="background:#088;width:100%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">8,297</td><td class="bar"><div style="background:#800;width:18.98%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">10,417</td><td class="bar"><div style="background:#080;width:23.83%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">7,048</td><td class="bar"><div style="background:#333;width:16.12%;">|</div></td></tr>
<tr><td>i9 Linux Desktop Rust</td><td class="num">505</td><td class="bar"><div style="background:#008;width:1.15%;">|</div></td></tr>
</table>

### Instructions Retired
- **instructions retired**

This metric counts the total number of CPU instructions executed to complete the task. Lower values can indicate less computational work. Interestingly, the x86_64 binary under Rosetta executes significantly more instructions, suggesting emulation overhead.

<table>
<tr><th>Platform</th><th colspan="2">Instructions <span>(trillions)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">126.72</td><td class="bar"><div style="background:#808;width:58.12%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">200.54</td><td class="bar"><div style="background:#088;width:92.00%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">144.87</td><td class="bar"><div style="background:#800;width:66.45%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">119.54</td><td class="bar"><div style="background:#080;width:54.83%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">218.01</td><td class="bar"><div style="background:#333;width:100%;">|</div></td></tr>
<tr><td>i9 Linux Desktop Rust</td><td class="num">147.02</td><td class="bar"><div style="background:#008;width:67.44%;">|</div></td></tr>
</table>

### Cycles Elapsed
- **cycles elapsed**

This shows the number of CPU cycles used during execution. The Apple Silicon implementations used substantially fewer cycles than their Intel counterparts, demonstrating the architecture's efficiency.

<table>
<tr><th>Platform</th><th colspan="2">Cycles <span>(trillions)<span>[shorter is better]</th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">28.98</td><td class="bar"><div style="background:#808;width:26.98%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">49.93</td><td class="bar"><div style="background:#088;width:46.49%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">107.39</td><td class="bar"><div style="background:#800;width:100%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">79.28</td><td class="bar"><div style="background:#080;width:73.83%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">44.33</td><td class="bar"><div style="background:#333;width:41.28%;">|</div></td></tr>
<tr><td>i9 Linux Desktop Rust</td><td class="num">103.17</td><td class="bar"><div style="background:#008;width:96.07%;">|</div></td></tr>
</table>

### Instructions Per Cycle
- **instructions per cycle**

IPC measures CPU efficiency - how many instructions are completed per clock cycle on average. Higher is better. Apple Silicon shows dramatically better IPC than Intel, with the Rosetta x86_64 implementation achieving the highest efficiency despite running emulated code.

<table>
<tr><th>Platform</th><th colspan="2">IPC <span><span>[longer is better]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">4.37</td><td class="bar"><div style="background:#808;width:88.82%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">4.02</td><td class="bar"><div style="background:#088;width:81.71%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">1.35</td><td class="bar"><div style="background:#800;width:27.44%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">1.51</td><td class="bar"><div style="background:#080;width:30.69%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">4.92</td><td class="bar"><div style="background:#333;width:100%;">|</div></td></tr>
<tr><td>i9 Linux Desktop Rust</td><td class="num">1.42</td><td class="bar"><div style="background:#008;width:28.86%;">|</div></td></tr>
</table>

### Peak Memory Footprint
- **peak memory footprint**

This represents the maximum memory actually touched (not just allocated) during execution. The Rosetta emulated code has the highest memory footprint, while native Rust implementations on both platforms show efficient memory usage.

<table>
<tr><th>Platform</th><th colspan="2">Peak Memory <span>(megabytes)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust</td><td class="num">118</td><td class="bar"><div style="background:#808;width:27.90%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift</td><td class="num">296</td><td class="bar"><div style="background:#088;width:70.23%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">104</td><td class="bar"><div style="background:#800;width:24.61%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">280</td><td class="bar"><div style="background:#080;width:66.38%;">|</div></td></tr>
<tr><td>M4 MacBookPro x86_64!</td><td class="num">421</td><td class="bar"><div style="background:#333;width:100%;">|</div></td></tr>
</table>

## Binary Sizes
```
MacOS Apple M4  Waver-rust:  2,075,176
MacOS Apple M4  Waver-swift:   760,728

MacOS Intel i9  Waver-rust:  2,145,472
MacOS Intel i9  Waver-swift:   770,512

Linux Intel i9  Waver-rust:  2,335,512
```

## Performance Details

The following performance metrics were collected during various runs of my `Waver-rust` and `Waver-swift` tools across my different machines, all processing the same dataset. I made sure the machines had cooled down between runs and chose the best test run from several test runs (albeit they were rather consistent in performance).

### Test runs:
Each test is grouped by CPU and platform to help compare relative performance between architectures and toolchains.

#### CPU: Apple M4 Max - MacBookPro 16-inch
```
>Waver-rust          >Waver-swift          files: 50056 total size: 362G
              531.35               952.81  real
            8,233.19            13,871.20  user
               71.93               357.92  sys
         324,927,488          317,652,992  maximum resident set size
             162,271            7,786,825  voluntary context switches
           3,562,056           43,719,084  involuntary context switches
 126,715,886,032,825  200,537,519,156,445  instructions retired
  28,978,240,976,295   49,925,846,249,479  cycles elapsed
                4.37                 4.02  instructions per cycle
         123,241,312          310,166,752  peak memory footprint
```

This is the correctly compiled version of the code as native Apple Silicon ARM code.

Note how the Rust version significantly beat the Swift version on the Apple Silicon machine!

Note also that the Rust version seems to allocate more memory than it actually touches.

#### CPU: Intel Core i9-9980HK - MacBookPro 16-inch
The same code was also compiled and tested on the Intel MacBookPro (i9, high-end configuration).  The Intel MacBook was plugged in and actively cooling with fans — though the M4 Max also ran fans, it did so for a shorter period and while unplugged.
```
>Waver-rust          >Waver-swift          files: 50056 total size: 362G
            2,310.94             1,763.86  real
           34,821.80            25,992.10  user
              224.28               695.70  sys
         241,524,736          299,642,880  maximum resident set size
             166,490            8,351,647  voluntary context switches
           8,296,980           10,416,933  involuntary context switches
 144,865,762,426,892  119,543,993,473,802  instructions retired
 107,392,710,469,137   79,283,453,250,177  cycles elapsed
                1.35                 1.51  instructions per cycle
         108,707,840          293,179,392  peak memory footprint
```

Swift wins on x86_64 for Intel even though it loses on Apple Silicon/ARM.  This is surprising, likely explained by Apple’s deeply optimized media libraries used by Swift while the Rust code is pure Rust and likely lacks the benefit of those same optimizations.  However, the fact that it was able to outdo Apple's optimized media libraries on Apple's own silicon is amazing.

The high context switch count in the Swift runs supports the fact that it is using OS- and library-level offloading.  Rust, on the other hand, doesn’t tap into these Apple-optimized frameworks and suffers from that on the Intel hardware.

#### CPU: Apple M4 Max - MacBookPro 16-inch - Rosetta
Here are the numbers for the x86_64 compiled code running on the M4!  Yes, wall clock time beat the native Swift code!  This just shows how good Apple's Rosetta is with Intel to Apple Silicon conversion in real time.  The M4 Max runs the Intel native binary nearly 3 times fast than the i9 MacBookPro and over 50% faster than the Intel i9 desktop!
```
>Waver-rust x86_64    files: 50056 total size: 362G
              862.21  real
            13080.89  user
              106.17  sys
         441,946,112  maximum resident set size
             159,579  voluntary context switches
           7,048,201  involuntary context switches
 218,012,619,278,690  instructions retired
  44,331,868,186,106  cycles elapsed
                4.92  instructions per cycle
         441,674,608  peak memory footprint
```

This is the same code but with the wrong tool chain!  Look at those numbers!  In fact, to validate this, I tried the actual binary as built on my Intel MacBookPro and ran the tests again and it produced the same results!

#### CPU: Intel i9-9900K – Linux Tower (Water-Cooled, ~4.72 GHz Average)
```
>Waver-rust           files: 50056 total size: 362G
            1,372.13  real
           21,735.25  user
              115.48  sys
         113,156,096  maximum resident set size
             214,256  voluntary context switches
             504,873  involuntary context switches
 147,020,034,428,541  instructions retired
 103,173,937,600,610  cycles elapsed
                1.42  instructions per cycle
                 n/a  peak memory footprint
```

The Linux desktop outperformed the Intel MacBookPro thanks to its higher clock speed and better cooling.  It even beat Swift on the Intel platform.  Still, it couldn’t keep up with the M4 Max on battery power — a remarkable testament to Apple Silicon.

While the Linux system’s SSD wasn’t quite as fast as Apple’s, it was more than fast enough (averaging 520MB/s, peaking at 600MB/s).  With water cooling maintaining ~81°C max CPU temp, it ran at full boost the entire time — unlike the thermally throttled MacBooks.

---

## Tool Chains:
All tests used the latest available toolchains for each system.

**Apple M4 Max MacBookPro:**
```
  swift-driver version: 1.120.5
  Apple Swift version 6.1 (swiftlang-6.1.0.110.21 clang-1700.0.13.3)
  rustc 1.86.0 (05f9846f8 2025-03-31) cargo 1.86.0 (adf9b6ad1 2025-02-28)
```

**Apple i9 MacBookPro:**
```
  swift-driver version: 1.120.5
  Apple Swift version 6.1 (swiftlang-6.1.0.110.21 clang-1700.0.13.3)
  rustc 1.86.0 (05f9846f8 2025-03-31) cargo 1.86.0 (adf9b6ad1 2025-02-28)
```

**Linux i9 Desktop:**
```
  rustc 1.86.0 (05f9846f8 2025-03-31) cargo 1.86.0 (adf9b6ad1 2025-02-28)
```

---

## Notable Differences

While both implementations produce visually comparable waveform images, they differ significantly in design philosophy, capabilities, and performance characteristics. The Rust version prioritizes cross-platform purity and memory safety at the cost of reduced feature breadth, whereas the Swift version leverages Apple’s deeply optimized frameworks for platform-specific efficiency but introduces trade-offs in abstraction and flexibility.

### Rust Trade-Offs/Benefits

1. **Format Support Limitations**:
   Reliance on the `symphonia` crate restricts primary support to MP3 files, contrasting with Swift’s broader format compatibility via Apple’s media libraries. This limitation stems from less mature ecosystem compared to Apple’s well-established frameworks.

2. **Memory Efficiency & Output Optimization**:
   Rust generates smaller PNG files by utilizing indexed color palettes (4-color), a feature not available in the default Core Graphics implementation used by Swift. This results in lower memory footprints during processing, with Rust showing 105MB peak residency on small datasets versus Swift’s 164MB.

3. **Native Streaming Performance**:
   Rust’s buffer management avoids memory spikes through optimized stream-based decoding, contrasting with Swift’s reliance on manual buffer size tuning to balance I/O and CPU efficiency (visible in larger max resident set sizes).

4. **Compile-Time Safety Guarantees**:
   Rust enforces strict memory safety at compile time via its ownership model, eliminating classes of runtime errors such as null pointer dereferences or data races inherent in frameworks using Objective-C or manual memory management.

---

### Swift Trade-Offs/Benefits

1. **Overhead from Advanced Frameworks**:
   Apple’s Core Audio and Core Graphics libraries include capabilities like HDR color profiling and real-time video effects—unnecessary for basic waveform generation—that contribute to increased computational overhead (evident in higher context switch counts and IPC inefficiency).

2. **Manual Buffer Optimization Dependency**:
   Swift requires explicit tuning of buffer sizes (e.g., `bufferSize = AVAudioFrameCount(65535)`) to maintain optimal performance, as seen in its sensitivity to I/O vs CPU workload balance across different file sizes.

3. **Framework Abstraction Layers**:
   Much of the Swift code delegates core processing to Apple’s lower-level Objective-C implementations (e.g., `AVAudioFile`, `NSBitmapImageRep`), which introduces indirection and reduces control over low-level optimizations compared to Rust’s direct, pure-Rust execution paths.

4. **Leverages Apple's Media Support**:
   This is hard to overstate, but Apple's advanced media support brings with it significant wins, including very robust support of not just MP3 but many other audio formats.  This means that if I were to need AAC support, the Swift version would already have it, unlike in "pure" Rust.  Swift applications can easily tap into hardware accelerations through Apple's frameworks and APIs, which are deeply integrated with the language and development environment.  This integration allows Swift code to efficiently leverage specialized hardware capabilities with minimal effort.  It just happens that in this case, that support was not a significant factor.

---

## Big Surprise!

Now, after some time of having written the above, I happened to wonder how well the Rosetta system really worked for other code.  The first thing I did was to take the same Swift code but compiled on my i9 MacBookPro and brought it to my M4Max MacBookPro and an M1Max MacBookPro.

The big surprise?  Swift compiler produces horrible Apple Silicon native code (or the Swift compiler produces amazing Intel CPU code).

The x86_64 compiled Swift code was actually slightly faster than the natively compiled code!  It was even more pronounced on the M1 Max CPU - where the x86_64 code was significantly faster than the native code!  How is that possible?  Maybe the Swift compiler is just really bad at generating ARM/Apple Silicon code.

Note that the M1Max CPU has only 10 cores (8P cores and 2E cores) while the M4Max has 16 cores (12P cores and 4E cores) which is why the wall clock time showed nearly 3x performance improvement for the native code on M4 compared to M1 but if you look at CPU time, it is not that much.  Still, the M4Max definitely retires more instructions per cycle in addition to running at higher clock speeds.

The i9 MacBookPro shows that the Swift code gen for the Intel processor is really good in comparison.  It performed better than the M1 MaxBookPro in wall clock (user time) but used more CPU time - the i9 MacBookPro has 8 real cores with 2 hardware threads per core so it acts close to a 16 core machine under the right conditions and this seems to have shown that!

**Note**  These tests were run over a different dataset than those earlier in the document.  I no longer had that exact same dataset available.

<table>
<tr><th>Platform</th><th colspan="2">Time <span>(seconds)<span>[shorter is faster]</span></span></th></tr>
<tr><td>M4 MacBookPro Swift Native</td><td class="num">57.88</td><td class="bar"><div style="background:#088;width:38.84%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift x86_64</td><td class="num">57.08</td><td class="bar"><div style="background:#808;width:38.30%;">|</div></td></tr>
<tr><td>M1 MacBookPro Swift Native</td><td class="num">149.02</td><td class="bar"><div style="background:#088;width:100.0%;">|</div></td></tr>
<tr><td>M1 MacBookPro Swift x86_64</td><td class="num">113.76</td><td class="bar"><div style="background:#808;width:76.34%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">106.81</td><td class="bar"><div style="background:#880;width:71.29%;">|</div></td></tr>
</table>

```
 M4 - Swift Native   M4 - Swift x86_64   M1 - Swift Native   M1 - Swift x86_64         i9 - Swift
             57.88               57.08              149.02              113.76             106.81  real
            901.07              886.71            1,440.82            1,098.64           1,657.28  user
              6.44                8.13               12.14                9.66              18.56  sys
     3,353,591,808       3,324,108,800       1,839,431,680       1,864,417,280      1,851,748,352  maximum resident set size
             3,120               3,470               3,193               3,186              3,146  voluntary context switches
           173,175             180,453             213,618             172,124            215,222  involuntary context switches
12,844,888,747,702  13,992,329,799,520  12,849,258,068,293  13,965,809,647,675  7,708,049,163,527  instructions retired
 3,173,763,286,259   3,113,048,935,885   4,151,163,543,513   3,164,590,363,249  5,343,710,243,119  cycles elapsed
              4.05                4.49                3.10                4.41               1.44  instructions per cycle
     3,317,648,480       3,329,125,512       1,810,191,360       1,865,813,696      1,795,973,120  peak memory footprint
        46,406,588          46,408,925          46,406,588          46,408,925         46,408,925  total bytes png files
```

<table>
<tr><th>Platform</th><th colspan="2">CPU Time <span>(seconds)<span>[shorter is faster]</span></span></th></tr>
<tr><td>M4 MacBookPro Swift Native</td><td class="num">901.07</td><td class="bar"><div style="background:#088;width:54.37%;">|</div></td></tr>
<tr><td>M4 MacBookPro Swift x86_64</td><td class="num">886.71</td><td class="bar"><div style="background:#808;width:50.50%;">|</div></td></tr>
<tr><td>M1 MacBookPro Swift Native</td><td class="num">1440.82</td><td class="bar"><div style="background:#088;width:86.94%;">|</div></td></tr>
<tr><td>M1 MacBookPro Swift x86_64</td><td class="num">1098.64</td><td class="bar"><div style="background:#808;width:66.29%;">|</div></td></tr>
<tr><td>i9 MacBookPro Swift</td><td class="num">1657.28</td><td class="bar"><div style="background:#880;width:100%;">|</div></td></tr>
</table>

### Rust produces better Apple Silicon code

The Rust compiled code performance is as you would expect.  It also shows that it produces exactly the same results on both processor architectures.  The subtle rounding/image differences do not show up.  They are bit-for-bit exactly the same results.  This is what you want from code that processes data.

It also shows how much better the M4 processor is over the M1 and how the M1 already is significantly better than the Intel i9.

Not only that, the Rust code also produces much more optimal PNG files which explains the significant size savings even though they have the same wave form images in them.

Note that in all cases the x86_64 binaries are the same (built on the i9 machine).

<table>
<tr><th>Platform</th><th colspan="2">Time <span>(seconds)<span>[shorter is faster]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust Native</td><td class="num">33.69</td><td class="bar"><div style="background:#088;width:28.02%;">|</div></td></tr>
<tr><td>M4 MacBookPro Rust x86_64</td><td class="num">48.29</td><td class="bar"><div style="background:#808;width:40.16%;">|</div></td></tr>
<tr><td>M1 MacBookPro Rust Native</td><td class="num">74.62</td><td class="bar"><div style="background:#088;width:62.06%;">|</div></td></tr>
<tr><td>M1 MacBookPro Rust x86_64</td><td class="num">94.08</td><td class="bar"><div style="background:#808;width:78.25%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">120.23</td><td class="bar"><div style="background:#880;width:100%;">|</div></td></tr>
</table>

```
 M4 - Rust Native    M4 - Rust x86_64   M1 - Rust Native    M1 - Rust x86_64          i9 - Rust
            33.69               48.29              74.62               94.08             120.23  real
           525.52              747.10             720.93              910.52           1,885.22  user
             3.68                4.85               6.78                7.31               7.21  sys
       35,618,816          36,241,408         23,789,568          24,494,080         32,022,528  maximum resident set size
                4                   1                  4                   3                 15  voluntary context switches
           86,282             124,135            104,329             127,973            231,475  involuntary context switches
7,850,819,714,697  12,221,211,323,564  7,850,597,715,658  12,244,086,240,882  8,330,060,241,836  instructions retired
1,865,337,341,480   2,590,336,001,001  2,077,492,279,600   2,620,509,208,004  6,118,377,513,298  cycles elapsed
             4.21                4.72               3.78                4.67               1.36  instructions per cycle
       32,850,688          35,223,304         21,022,400          22,689,472         29,450,240  peak memory footprint
       13,539,020          13,539,020         13,539,020          13,539,020         13,539,020  total bytes png files
```

<table>
<tr><th>Platform</th><th colspan="2">CPU Time <span>(seconds)<span>[shorter is faster]</span></span></th></tr>
<tr><td>M4 MacBookPro Rust Native</td><td class="num">525.52</td><td class="bar"><div style="background:#088;width:27.88%;">|</div></td></tr>
<tr><td>M4 MacBookPro Rust x86_64</td><td class="num">747.10</td><td class="bar"><div style="background:#808;width:39.63%;">|</div></td></tr>
<tr><td>M1 MacBookPro Rust Native</td><td class="num">720.93</td><td class="bar"><div style="background:#088;width:38.24%;">|</div></td></tr>
<tr><td>M1 MacBookPro Rust x86_64</td><td class="num">910.52</td><td class="bar"><div style="background:#808;width:48.30%;">|</div></td></tr>
<tr><td>i9 MacBookPro Rust</td><td class="num">1885.22</td><td class="bar"><div style="background:#880;width:100%;">|</div></td></tr>
</table>

What this all shows is that there is some significant benefit to using Rust from a performance standpoint in addition to the stricter correctness and safety.  It also shows that the Swift compiler toolchain, somewhere, is leaving a lot of performance on the floor.  The Intel code gen clearly is doing well while the Apple Silicon code gen is, to put it frankly, rather poor - especially when it can be beaten by Rosetta doing its magic on the Intel code from the same source code.

Rust clearly shows the performance difference that is expected when the compiler produces quality code.

This also, again, shows the amazing capabilities of Rosetta and the Apple Silicon CPU.  It is hard to express just how impressive they are given the performance gains *and* power savings compared to the Intel counterparts.

### Note on the data set

Since a significant amount of time had passed since I first did the Swift and Rust testing, I no longer had the exact same data set to test on so I had to make a benchmark dataset specifically designed to help test the code again.  It contains 4 different MP3 files of 4 very different sorts:  48kHz stereo, 44.1kHz joint stereo, 44.1kHz stereo, and 22kHz mono - all with different bit rates.  These 4 files were then duplicated (via hard links) to produce a total of 3,116 MP3 files (equal counts of each) distributed over many directories and sub-directories such that the tree was complex and mixed.

Why the hard links? Hard links allowed the operating system to cache just the 4 unique MP3 files in memory, completely eliminating disk I/O as a performance factor. While the test was processing over 3,000 MP3 files that represented more than 21 gigabytes of logical data, the actual unique file content was less than 30 megabytes - small enough to fit entirely in the OS buffer cache. This approach ensured that file reading time was effectively zero, allowing us to focus purely on processing performance. (Write operations for the PNG output files were still required, but these were minimal - totaling less than 46 megabytes across all files created.)

---

## Small Dataset Performance

### CPU: Apple M4 Max - MacBookPro 16-inch
```
>Waver-rust        >Waver-swift        files: 579 total size: 3.4G
              5.14               8.82  real
             78.84             137.04  user
              0.51               0.88  sys
       104,759,296        164,511,744  maximum resident set size
                 0                  0  voluntary context switches
            28,308             52,119  involuntary context switches
 1,221,008,526,268  1,932,847,466,808  instructions retired
   284,369,020,788    493,442,491,292  cycles elapsed
              4.93               3.92  instructions per cycle
        42,664,728        159,794,136  peak memory footprint
```

### CPU: Intel(R) Core(TM) i9-9980HK - MacBookPro 16-inch
```
>Waver-rust        >Waver-swift        files: 579 total size: 3.4G
             23.15              15.85  real
            320.66             226.38  user
              2.61               6.42  sys
        84,205,568        156,160,000  maximum resident set size
               571             81,332  voluntary context switches
           220,396            186,828  involuntary context switches
 1,397,457,239,759  1,163,196,148,748  instructions retired
 1,032,475,223,430    776,781,237,103  cycles elapsed
              1.35               1.50  instructions per cycle
        41,934,848        152,006,656  peak memory footprint
```

---

## M1 MacBookAir performance

This is a smaller, different dataset as the base model M1 MacBookAir
does not have the size of SSD or the performance of the newer M4.

You can definitely see the slower clock rate and lower IPC counts
but the performance difference is real between the Rust code and
the Swift code.  At 2 times slower, the Swift code definitely
loses in this specific scenario, even on Apple Silicon.

### CPU: Apple M1 - MacBookAir
```
>Waver-rust       >Waver-swift      files: 284 total size: 1.7G
            8.35             16.83  real
           60.64            115.39  user
            0.78              2.43  sys
      18,694,144       130,940,928  maximum resident set size
             885            38,124  voluntary context switches
          40,642           167,697  involuntary context switches
 564,405,682,270   887,674,571,101  instructions retired
 156,718,297,503   303,945,723,413  cycles elapsed
            3.60              2.92  instructions per cycle
      15,402,496       126,043,968  peak memory footprint
```

---

## The Code

Here are stripped down sources of the code that was used for the above tests.

### Swift
Built with: `swift build -c release`
```swift
// Simple hack program that, on MacOS with Swift, reads audio file (MP3/etc)
// and produces a PNG file that represents its waveform visually.

// This specifically uses streaming such that the whole audio does not
// need to be loaded into memory at once.  This allows the processing of
// some large/long audio files at the speed of decoding them without
// significant memory overhead.  Because of this we can then safely
// parallelize the operations such that multiple audio files can be
// processed at once.

// We use the swift argument parser library to have declarative arguments
// and argument validation handled in both a standard and simple way

import AVFoundation
import AppKit
import ArgumentParser
import Foundation


/// Waver Command Line Tool
/// The main command line tool struct for converting audio files to waveform
/// PNG images.
///
/// This tool accepts various command line arguments to customize the output of
/// the waveforms, such as image dimensions, colors for different channels and
/// background, buffer size for streaming, and more.
@main
struct Waver: ParsableCommand {
    @Option(help: "Image Width")
    var width: Int = 2048

    @Option(help: "Image Height (must be even)")
    var height: Int = 128

    @Option(help: "Left channel (and mono) color in RGB/RRGGBB/RRGGBBAA hex")
    var leftColor = "00ff99"

    @Option(help: "Right channel color in RGB/RRGGBB/RRGGBBAA hex")
    var rightColor = "99ff00"

    @Option(help: "Background color in RGB/RRGGBB/RRGGBBAA hex")
    var backgroundColor = "ffffff00"

    @Option(name: .shortAndLong, help: "Optional output file name (defaults to input+.png)")
    var outputFilename = ""

    @Option(help: "File extensions to process when given a directory")
    var fileExtensions = ["mp3", "m4a"]

    // Constrain our audio reading buffer size such that we stream
    // the audio but not in too small of chunk sizes and not too large.
    //
    // If you set this too large, the memory load and cache performance
    // can be impacted.  If you set it too small, the number stream read
    // requests go up and can impact performance.
    //
    // Perf Tests - when processing a directory tree with 579 MP3 files that
    // total around 3.4GB in size, all in highest quality variable bit rate
    // encodings, we get the following performance on my MacBookPro:
    // (Measured after multiple repeated runs with the same buffer size,
    // taking the best run which usually was the last run)
    //
    //       4194303        131071         65535         32767          4095  --buffer-size
    // ------------- ------------- ------------- ------------- -------------  -------------------------
    //          9.35          8.79          8.64          8.77          9.07  real time
    //        139.18        137.43        135.36        136.81        136.76  user (cpu user time)
    //          4.12          0.77          0.75          1.02          5.11  sys (cpu OS time)
    //    2806333440     206422016     162676736     141836288     123486208  maximum resident set size
    // 1956819286348 1944140432707 1944272337536 1946504453175 1979054857635  instructions retired
    //  511558401161  493050273427  485401415345  482658543430  506685982189  cycles elapsed
    //    2802748008     201704432     157926360     137069504     118653792  peak memory footprint
    //
    // Note how increased buffer sizes slow things down due to the impact on
    // CPU cache and smaller sizes slow things down due to number of additional
    // read stream calls.
    //
    // The peak performance is somewhere between 32K and 128K.  In order to test
    // all of this, I had it as a parameter but have the default value at the
    // peak performance point.
    //
    // I have this as an option so that it can be tuned for performance
    // and experimented with.  I have also found some cases where the
    // AVAudioFile.length is not accurate and reading until that length
    // can fail.  This is actually more common than I expected.  See comments
    // in the code where I handle those conditions.
    @Option(help: "Buffer size to use while streaming audio (in frames)")
    var bufferSize = AVAudioFrameCount(65535)

    @Flag(help: "Do not write anything")
    var dryRun = false

    @Flag(help: "Overwrite existing output file")
    var overwrite = false

    @Flag(help: "Quieter output")
    var quiet = false

    @Flag(help: "Verbose - show overwrite warnings")
    var verbose = false

    // The audio file(s) or directory to process.
    @Argument(help: "The audio file to process (or files/directory if not using --output-filename)")
    var audioFilenames: [String]

    /// An error type for handling argument validation issues.
    struct ArgumentError: LocalizedError {
        let description: String

        init(_ description: String) {
            self.description = description
        }

        var errorDescription: String? {
            description
        }
    }

    /// An error type for handling waveform generation issues.
    struct GenerationError: LocalizedError {
        let description: String

        init(_ description: String) {
            self.description = description
        }

        var errorDescription: String? {
            description
        }
    }

    /// Converts a hex color string to a `CGColor` object.
    /// Supports these RGB formats:
    ///  RGB      - transforms to RRGGBB
    ///  RRGGBB   - assumes alpha is 1.0
    ///  RRGGBBAA - alpha value is 00 to FF (0.0 to 1.0)
    ///
    /// - Parameter rgbString: hex string representing the color (RGB, RRGGBB, or RRGGBBAA).
    /// - Returns: A `CGColor` object.
    /// - Throws: An `ArgumentError` if the color string is invalid.
    func cgColorFrom(rgbString: String) throws -> CGColor {
        guard
            let rgb = UInt(rgbString, radix: 16)
        else {
            throw ArgumentError(
                "Invalid color format: '\(rgbString)' - invalid hex number")
        }
        // Simple 3 digit RGB color
        if rgbString.count == 3 {
            return CGColor(
                red: (CGFloat)(((rgb & 0x0F00) >> 8) * 17) / 255.0,
                green: (CGFloat)(((rgb & 0x00F0) >> 4) * 17) / 255.0,
                blue: (CGFloat)((rgb & 0x000F) * 17) / 255.0,
                alpha: 1.0)
        }
        // Common 6 digit RGB color
        if rgbString.count == 6 {
            return CGColor(
                red: (CGFloat)((rgb & 0x00FF_0000) >> 16) / 255.0,
                green: (CGFloat)((rgb & 0x0000_FF00) >> 8) / 255.0,
                blue: (CGFloat)(rgb & 0x0000_00FF) / 255.0,
                alpha: 1.0)
        }
        // Common 8 digit RGBA color
        if rgbString.count == 8 {
            return CGColor(
                red: (CGFloat)((rgb & 0x00_FF00_0000) >> 24) / 255.0,
                green: (CGFloat)((rgb & 0x00_00FF_0000) >> 16) / 255.0,
                blue: (CGFloat)((rgb & 0x00_0000_FF00) >> 8) / 255.0,
                alpha: (CGFloat)(rgb & 0x00_0000_00FF) / 255.0)
        }
        // Unknown format
        throw ArgumentError(
            "Invalid color format: '\(rgbString)' - We support RGB, RRGGBB, and RRGGBBAA hex formats"
        )
    }

    /// Validates the command-line arguments provided.  It can not mutate
    /// anything, just success or throws an error.  Automatically called
    /// by the argument parser after completing the parsing of the command
    /// line arguments and before calling the run() entry point.
    ///
    /// - Throws: An `ArgumentError` if any validation fails.
    func validate() throws {
        // Since validation can't actually save the results, just checks, we
        // just try to convert each color argument to validate that they do
        // correctly convert during validation such that we don't have to do it
        // later.
        _ = try cgColorFrom(rgbString: leftColor)
        _ = try cgColorFrom(rgbString: rightColor)
        _ = try cgColorFrom(rgbString: backgroundColor)

        if width < 16 {
            throw ArgumentError("--width must be at least 16 pixels")
        }

        // We need the height to be even (left/right channels) and at least 6 pixels
        if height < 6 || height & 1 == 1 {
            throw ArgumentError("--height must be at least 6 pixels and even")
        }

        if bufferSize < 1024 {
            throw ArgumentError("--buffer-size must be at least 1024 frames")
        }

        if audioFilenames.count < 1 {
            throw ArgumentError("No audio file specified")
        }

        // Check that output filename is not specified with multiple audio files
        if audioFilenames.count > 1 && outputFilename.count > 0 {
            throw ArgumentError("Cannot specify --output-filename with multiple audio files")
        }

        // Check that audio files/directories exist
        for filename in audioFilenames {
            var isDirectory: ObjCBool = false
            if !FileManager.default.fileExists(atPath: filename, isDirectory: &isDirectory) {
                throw ArgumentError("Could not find audio-filename: \(filename)")
            }
            // Check that output filename is not specified with a directory
            if outputFilename.count > 0 && isDirectory.boolValue {
                throw ArgumentError("Cannot specify --output-filename with a directory")
            }
        }
    }

    /// Prints messages to standard error.
    ///
    /// - Parameter items: The items to print.
    /// - Parameter separator: A string to separate the printed items (default is space).
    /// - Parameter terminator: The character(s) to print after the last item (default is newline).
    func printToStdErr(_ items: Any..., separator: String = " ", terminator: String = "\n") {
        if !quiet {
            let output =
                items
                .map { String(describing: $0) }
                .joined(separator: separator) + terminator

            FileHandle.standardError.write(output.data(using: .utf8)!)
        }
    }

    /// Generates a waveform PNG image from an audio file.
    ///
    /// This method reads audio data in chunks (streaming) to minimize memory usage,
    /// processes each chunk to compute the waveform, and draws it onto a Core Graphics context.
    ///
    /// - Parameters:
    ///   - inputFile: The path to the input audio file.
    ///   - outputFile: The path where the output PNG image should be saved.
    ///   - colors: An array of `CGColor` objects representing the left channel, right channel,
    ///     and background colors in that order.
    /// - Throws: A `GenerationError` if any issues occur during processing, such as file I/O errors
    ///           or invalid audio formats.
    func generateWavePng(
        inputFile: String,
        outputFile: String,
        colors: [CGColor]
    ) throws {
        // We were going to make the image and PNG as a 2-bit (4 color) image
        // but MacOS libraries no longer seem to support that.  This results in
        // PNG files that are a bit larger (about twice as large) as they would
        // be if they were 2bpp.  But they are rather small anyway.

        // Check for existing output file if overwriting is not allowed
        if !overwrite && FileManager.default.fileExists(atPath: outputFile) {
            if !verbose {
                return
            }
            throw GenerationError(
                "Output file '\(outputFile)' already exists - use --overwrite to overwrite")
        }
        do {
            // Open the MP3 file without loading it fully into memory
            guard
                let audioFile = try? AVAudioFile(forReading: URL(fileURLWithPath: inputFile))
            else {
                throw GenerationError("Could not open for audio processing")
            }

            guard
                let buffer = AVAudioPCMBuffer(
                    pcmFormat: audioFile.processingFormat,
                    frameCapacity: bufferSize)
            else {
                throw GenerationError("Could not create buffer for audio file")
            }

            let imageWidth = CGFloat(width)
            let imageHeight = CGFloat(height)
            let imageCenter = imageHeight / 2
            let pixelsPerSample = imageWidth / Double(audioFile.length)

            // Get our rendering context for an image of our size
            guard
                let context = CGContext(
                    data: nil,
                    width: width,
                    height: height,
                    bitsPerComponent: 8,  // 8 bits per component
                    bytesPerRow: 0,  // Let CGContext define this
                    space: CGColorSpace(name: CGColorSpace.sRGB)!,
                    bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)
            else {
                throw GenerationError("Could not create rendering context")
            }

            // Set the background color and fill the entire image with it
            context.setFillColor(colors[colors.count - 1])
            context.fill(CGRect(origin: .zero, size: CGSize(width: imageWidth, height: imageHeight)))

            // Tracks the maximum values for each channel
            var maxVals = [Float](repeating: 0.0, count: 2)

            // Track our progress along the graph.  We use fractional
            // pixel progress as the samples are unlikely to be a perfect
            // multiple of the image width.
            var pixelProgress = CGFloat(0.0)
            var pixelPos = CGFloat(0.0)
            var nextPixel = pixelPos + 1.0

            // Stream the audio data in chunks and render the waveform
            while audioFile.framePosition < audioFile.length {
                let beforePos = audioFile.framePosition
                do {
                    try audioFile.read(into: buffer)
                } catch {
                    // Some audio terminate a bit earlier than expected and
                    // if the buffer size happens to match a multiple of the
                    // early termination point then we did not detect it at
                    // the bottom of the loop due to partial buffer fill and
                    // have to notice it here.
                    // If we are under a buffer length left over, it was likely
                    // this same problem and we just happened to have a buffer
                    // size that was a whole fraction of the early termination
                    // point.  What this really means is that the length of
                    // an AVAudioFile looks to be inaccurate at times.  (Likely
                    // due to VBR MP3 files, which is what I have)
                    // So, in this case, we accept it as "it is what it is"
                    // with optional verbose "warning" being logged
                    if audioFile.length - beforePos < bufferSize {
                        if verbose {
                            printToStdErr("Warning: Premature end reading '\(inputFile)' @ \(beforePos)/\(audioFile.length) \(Double(beforePos*10000/audioFile.length)/100.0)% : \(error.localizedDescription)")
                        }
                        break
                    } else {
                        throw GenerationError(
                            "Error reading @ \(beforePos)/\(audioFile.length): \(error.localizedDescription)"
                        )
                    }
                }

                let channelCount = min(2, Int(buffer.format.channelCount))
                let frameCount = buffer.frameLength

                guard
                    let channelData = buffer.floatChannelData
                else {
                    break
                }

                // Loop over each frame (sample) in the buffer keeping track of
                // the maximum value for each channel.  We do this until we have
                // combined enough samples to fill a single pixel width of the
                // image.  Then we render the pixel and reset the maximum values
                // back to 0.
                for frame in 0..<Int(frameCount) {
                    for channel: Int in 0..<channelCount {
                        // Note that we assume values are -1.0 to 1.0 in the
                        // samples but we protect ourselves by clamping this to
                        // 1.0 max value using the min(1.0, value) trick.
                        maxVals[channel] = max(maxVals[channel], min(1.0, abs(channelData[channel][frame])))
                    }
                    pixelProgress += pixelsPerSample
                    if pixelProgress > nextPixel {
                        // Render this pixel of the image...
                        context.setFillColor(colors[0])
                        context.fill(
                            CGRect(
                                x: pixelPos, y: imageCenter, width: 1,
                                height: round(imageCenter * CGFloat(maxVals[0]))))
                        context.setFillColor(colors[channelCount - 1])
                        context.fill(
                            CGRect(
                                x: pixelPos, y: imageCenter, width: 1,
                                height: -round(imageCenter * CGFloat(maxVals[channelCount - 1]))))

                        // Get ready for the next pixel
                        pixelPos = nextPixel
                        nextPixel += 1.0
                        for channel: Int in 0..<channelCount {
                            maxVals[channel] = 0.0
                        }
                    }
                }

                if buffer.frameLength < bufferSize {
                    // Render the last pixel in the wave for
                    // those cases where we did not get to a
                    // perfect multiple of the image size.
                    if pixelPos < imageWidth {
                        context.setFillColor(colors[0])
                        context.fill(
                            CGRect(
                                x: pixelPos, y: imageCenter, width: 1,
                                height: round(imageCenter * CGFloat(maxVals[0]))))
                        context.setFillColor(colors[channelCount - 1])
                        context.fill(
                            CGRect(
                                x: pixelPos, y: imageCenter, width: 1,
                                height: -round(imageCenter * CGFloat(maxVals[channelCount - 1]))))
                    }
                    // We are done with the file - we should not need to break
                    // out but it turns out AVAudioFile.length is not always
                    // accurate enough and we may think there are a few more
                    // samples to get but, really, we are done since we got
                    // a partial read.  It is faster for us to just break out
                    // here rather than do the next read which will throw and
                    // then break out.
                    break
                }
            }

            // Generate the final image
            let image = context.makeImage()!

            // If we are not dry-run, write the image to the file.
            if !dryRun {
                guard
                    let pngData = NSBitmapImageRep(cgImage: image).representation(
                        using: NSBitmapImageRep.FileType.png, properties: [:])
                else {
                    throw GenerationError("Could not create PNG representation of image")
                }

                try pngData.write(to: URL(fileURLWithPath: outputFile))
                if !quiet {
                    print("Created \(outputFile)")
                }
            }
            else if verbose {
                    print("DryRun \(outputFile)")
            }
        } catch {
            throw GenerationError("Error processing '\(inputFile)' : \(error.localizedDescription)")
        }
    }

    /// Recursively processes files in a directory tree.
    ///
    /// This method traverses the directory structure starting at the given URL and applies
    /// the provided handler closure to each matching file.
    ///
    /// - Parameters:
    ///   - path: The URL pointing to a file or directory to process.
    ///   - handler: A closure that processes each matching file.  The closure receives a `URL`
    ///             pointing to the file to be processed.
    /// - Throws: A `GenerationError` if there are issues accessing files or directories.
    func handleFiles(path: URL, handler: (URL) -> Void) throws {
        let fileManager = FileManager.default

        // Check if the directory exists or is a file
        var isDirectory: ObjCBool = false
        guard
            fileManager.fileExists(atPath: path.path, isDirectory: &isDirectory)
        else {
            throw GenerationError("Directory does not exist: \(path.path)")
        }
        if !isDirectory.boolValue {
            return handler(path)  // If it is not a directory, just call the handler
        }

        // It is a directory.  Get the contents of the directory
        // and process them.
        let items = try fileManager.contentsOfDirectory(at: path, includingPropertiesForKeys: nil)

        for item in items {
            // If the item is a directory, recursively call the function
            if item.hasDirectoryPath {
                try handleFiles(path: item, handler: handler)
            } else {
                // Check if the item matches the pattern
                for fileExtension in fileExtensions {
                    if item.pathExtension == fileExtension {
                        handler(item)  // Call the handler with the matching file
                        break
                    }
                }
            }
        }
    }

    /// The main execution method that runs the Waver tool.
    ///
    /// - Throws: A `GenerationError` if any issues occur during processing.
    func run() throws {
        // This is the main code - after all of the options have been
        // parsed and somewhat validated.

        // Note that since validation already checked these, the "try" will
        // always work here.  (The validation would have failed if they were
        // invalid and thus run() would never have been called.)
        let colors = [
            try cgColorFrom(rgbString: leftColor),  // User left channel color (and mono)
            try cgColorFrom(rgbString: rightColor),  // User right channel color
            try cgColorFrom(rgbString: backgroundColor),  // User background color (last element)
        ]

        // We need a queue group to wait for work to finish...
        let workGroup = DispatchGroup()
        // We need a dispatch group that lets work run concurrently...
        let workQueue = DispatchQueue(label: "waver.workQueue", attributes: .concurrent)

        // Safely accumulate errors from concurrent file processing
        // We need a safe way to accumulate errors from the generators
        // Unfortunately, Swift does not have such a safe thing built in
        // and building it safely still requires users to mark it as unsafe.
        class ErrorList {
            private var strings: [String] = ["while processing files:"]
            private let lock = NSLock()

            // Method to append a string in a thread-safe manner
            func safeAppend(_ string: String) {
                lock.lock()
                defer { lock.unlock() }
                strings.append(string)
            }

            func hasErrors() -> Bool {
                lock.lock()
                defer { lock.unlock() }
                return strings.count > 1
            }

            func getText() -> String {
                lock.lock()
                defer { lock.unlock() }
                return strings.joined(separator: "\n* ")
            }
        }

        // This is safe when used to append to the list
        nonisolated(unsafe) let errors = ErrorList()

        for path in audioFilenames {
            do {
                // We run handleFiles which recursively processes a path
                // if it is a directory.  In all cases, it then calls
                // our closure (lambda) for each of the files.
                try handleFiles(path: URL(fileURLWithPath: path)) { fileURL in
                    // At this point we have a single file name
                    // as passed to us from the handleFiles function.
                    // We use the dispatch queue to dispatch the processing
                    // of that file here.  This way we don't actually do the
                    // work directly but just queue the work and let each
                    // file be processed in parallel with other files.
                    let filename = fileURL.path
                    workQueue.async(group: workGroup) {
                        do {
                            try generateWavePng(
                                inputFile: filename,
                                // If output filename given, use it, otherwise generate one
                                outputFile: (outputFilename.count > 0) ? outputFilename : filename + ".png",
                                colors: colors)
                        } catch {
                            let errorText = "\(error.localizedDescription)"
                            printToStdErr(errorText)
                            errors.safeAppend(errorText)
                        }
                    }
                }
            } catch {
                let errorText = "Error handling '\(path)' : \(error.localizedDescription)"
                printToStdErr(errorText)
                errors.safeAppend(errorText)
            }
        }

        // Wait for all file processing to complete
        workGroup.wait()

        // Throw an error if any issues occurred during processing
        if errors.hasErrors() {
            // If there are errors, print a blank line to split the output
            // and then throw a detailed error text that will be rendered
            // by the argument parser entry point.
            printToStdErr("")
            throw GenerationError(errors.getText())
        }
    }
}
```

### Rust
Built with: `cargo build --release`
```rust
/// Waver: Generate waveform visualizations from audio files.
///
/// This tool creates PNG visualizations of audio waveforms from various audio
/// file formats.  It supports multiple audio file processing, customizable
/// colors, and various output options.
///
/// # Architecture
///
/// The program follows a data processing pipeline:
/// 1. Parse and validate command-line arguments
/// 2. Collect audio files to process
/// 3. Process each file in parallel, generating waveform images
/// 4. Report any errors that occurred during processing
///
/// # Performance
///
/// Key performance optimizations:
/// - Parallel processing of audio files using rayon
/// - Streaming audio decoding rather than buffering
/// - 2-bit pixel depth in PNG output for smaller files

use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Mutex;

use clap::Parser;
use png::{Encoder, FilterType};
use rayon::prelude::*;
use symphonia::core::audio::{AudioBuffer, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use thiserror::Error;
use walkdir::WalkDir;

//------------------------------------------------------------------------------
// Audio Processing
//------------------------------------------------------------------------------

/// Audio processing functionality for waveform generation.

/// Generates a waveform visualization from an audio file.
///
/// # Arguments
///
/// * `input_path` - Path to the input audio file
/// * `output_path` - Path where the output PNG will be saved
/// * `args` - Command-line arguments containing configuration
///
/// # Returns
///
/// `Ok(())` on success, or an error if processing fails
pub fn generate_waveform(
    input_path: &AudioPath,
    output_path: impl AsRef<Path>,
    args: &WaverArgs,
) -> Result<()> {
    let input_path = input_path.path();
    let output_path = output_path.as_ref();

    // Skip if output exists and overwrite isn't allowed
    if !args.overwrite && output_path.exists() {
        if args.verbose {
            return Err(WaverError::generation_error(format!(
                "Output file '{}' already exists - use --overwrite",
                output_path.display()
            )));
        }
        return Ok(());
    }

    // Generate the image buffer
    let mut image = WaveImage::new(args.width, args.height);

    // Process audio file and generate waveform
    process_audio_file(input_path, &mut image, args.width())?;

    // Save or log the result
    if !args.dry_run {
        image.save_png(
            &args.background_color,
            &args.left_color,
            &args.right_color,
            output_path,
        )?;
        args.print_to_stdout(&format!("Created {}", output_path.display()));
    } else if args.verbose {
        args.print_verbose(&format!("DryRun {}", output_path.display()));
    }

    Ok(())
}

/// Processes an audio file and generates a waveform visualization using a streaming approach.
///
/// This function opens an audio file, decodes it frame by frame, and immediately
/// processes each frame to generate the waveform image, without storing all audio data in memory.
///
/// # Performance
///
/// This is a performance-critical function.  It uses a streaming approach rather than buffering the
/// entire audio file, which results in:
/// - ~24x lower memory usage
/// - ~6.7x faster execution time
/// - Significantly fewer system calls
///
/// Do not change this to buffer all audio samples, as that would cause severe performance degradation.
///
/// # Arguments
///
/// * `input_path` - Path to the input audio file
/// * `image` - The waveform image to draw into
/// * `width` - Width of the output image in pixels
///
/// # Returns
///
/// `Ok(())` on success, or an error if processing fails
fn process_audio_file(input_path: &Path, image: &mut WaveImage, width: u32) -> Result<()> {
    // Open and probe the audio file
    let file = File::open(input_path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = symphonia::default::get_probe().format(
        &Hint::new(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    // Extract the first audio track
    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| {
            WaverError::generation_error(format!(
                "No audio track found in '{}'",
                input_path.display()
            ))
        })?;

    // Initialize decoder
    let mut decoder = symphonia::default::get_codecs().make(
        &track.codec_params,
        &DecoderOptions {
            ..Default::default()
        },
    )?;

    // Get channel information
    let channel_count = track
        .codec_params
        .channels
        .map(|c| c.count())
        .unwrap_or(1)
        .min(2) as usize;

    // Get total number of frames (samples per channel) for scaling calculation
    let total_samples = track.codec_params.n_frames.unwrap_or(0).max(1) as u64;

    // Calculate samples per pixel and the fractional
    // samples per pixel in 1/width units - since we have
    // to use width as u64 a number of times, do that conversion once
    let width64 = width as u64;
    let samples_per_pixel = total_samples / width64;
    let fractional_samples = total_samples % width64;

    // Initialize state variables for processing
    let mut left = 0.0f32;
    let mut right = 0.0f32;
    let mut sample_progress = samples_per_pixel;
    let mut partial_progress = 0 as u64;
    let mut pixel_pos = 0;

    // Process audio stream packet by packet
    while let Ok(packet) = format.next_packet() {
        let decoded = decoder.decode(&packet)?;
        let mut buffer = AudioBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
        decoded.convert(&mut buffer);

        // Process each frame in the current packet
        for frame in 0..buffer.frames() {
            // Update max amplitude values for each channel
            left = left.max(buffer.chan(0)[frame].abs().min(1.0));
            if channel_count > 1 {
                right = right.max(buffer.chan(1)[frame].abs().min(1.0));
            }

            // Map samples to pixels
            sample_progress -= 1;

            if sample_progress == 0 {
                // When we've accumulated enough samples for a pixel, draw it
                if channel_count > 1 {
                    image.draw_point(pixel_pos, left, right);
                } else {
                    image.draw_point_mono(pixel_pos, left);
                }
                left = 0.0;
                right = 0.0; // Reset max values for next pixel
                pixel_pos += 1;
                sample_progress = samples_per_pixel;
                partial_progress += fractional_samples;
                // If we got enough fractional samples to get another
                // sample in this next section, bump it by one and
                // subtract the width.
                if partial_progress >= width64 {
                    partial_progress -= width64;
                    sample_progress += 1;
                }
            }
        }
    }

    // Draw any remaining partial pixel
    if pixel_pos < width {
        if channel_count > 1 {
            image.draw_point(pixel_pos, left, right);
        } else {
            image.draw_point_mono(pixel_pos, left);
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------
// Image Generation
//------------------------------------------------------------------------------

/// Waveform image generation functionality with optimized 2-bit PNG output.

/// Represents the different channel types in a waveform image.
///
/// Using an enum instead of constants provides better type safety and
/// makes the code more self-documenting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// Background (transparent or base color)
    Background = 0,
    /// Left audio channel (typically drawn above center)
    Left = 1,
    /// Right audio channel (typically drawn below center)
    Right = 2,
}

impl From<Channel> for u8 {
    fn from(channel: Channel) -> Self {
        channel as u8
    }
}

impl From<u8> for Channel {
    fn from(value: u8) -> Self {
        match value {
            0 => Channel::Background,
            1 => Channel::Left,
            2 => Channel::Right,
            _ => Channel::Background, // Default to background for invalid values
        }
    }
}

/// Represents a waveform visualization image with 2-bit pixel depth optimization.
///
/// Contains the image dimensions and pixel data where each pixel is represented
/// by an index value (0 for background, 1 for left channel, 2 for right channel).
/// Uses 2 bits per pixel for significant space savings in the output PNG.
pub struct WaveImage {
    /// Width of the image in pixels.
    width: u32,

    /// Height of the image in pixels.
    height: u32,

    /// Vertical center line position.
    center: u32,

    /// Line size in bytes (due to 2 bits per pixel)
    line_width: u32,

    /// Pixel data stored as channel indices.
    /// During image generation, we use 1 byte per pixel for simplicity.
    pixels: Vec<u8>,
}

/// Convert a color index to the bit location based on the x coordinate
///
/// # Arguments
///
/// # `color` - The 2-bit color
/// # `x` - The horizontal position of the pixel
///
/// # Returns
///
/// The u8 with the color bits shifted to the correct location for 2-bpp
fn draw_bits(color: u8, x: u32) -> u8 {
    (color & 3) << (2 * (x & 3))
}

impl WaveImage {
    /// Creates a new waveform 2-bit per pixel image with the specified
    /// dimensions.  We render directly into the 2-bit per pixel form
    /// to reduce memory footprint and because we can do it efficiently.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the image in pixels
    /// * `height` - Height of the image in pixels (must be even)
    ///
    /// # Returns
    ///
    /// A new WaveImage instance initialized with background pixels
    ///
    /// # Notes
    ///
    /// This is a 2-bit per pixel image since we really only need
    /// at most 4 colors:
    ///   0:  Background color
    ///   1:  Left Channel  (or mono)
    ///   2:  Right Channel
    ///   3:  Background due to Left and Right collision
    pub fn new(width: Width, height: Height) -> Self {
        let width_val = width.value();
        let line_val = (width_val + 3) >> 2;
        let height_val = height.value();

        Self {
            width: width_val,
            height: height_val,
            line_width: line_val,
            center: height.center(),
            pixels: vec![0 as u8; (line_val * height_val) as usize],
        }
    }

    /// Draws a single point (left and right channels) of the waveform.
    ///
    /// # Arguments
    ///
    /// * `x` - The horizontal position to draw at
    /// * `left` - Left channel maximum amplitude
    /// * `right` - Right channel maximum amplitude
    pub fn draw_point(&mut self, x: u32, left: f32, right: f32) {
        if x >= self.width {
            return;
        }

        // The byte offset where the 2-bit pixel will be
        let offset = x >> 2;

        // Draw left channel (above center, going up)
        // The bits for the left channel at this pixel offset
        let draw_left = draw_bits(Channel::Left as u8, x);
        let left_height = (self.center as f32 * left + 0.5) as u32;
        for y in self.center.saturating_sub(left_height)..self.center {
            let idx = (offset + y * self.line_width) as usize;
            self.pixels[idx] |= draw_left;
        }

        // Draw right channel (below center, going down)
        // The bits for the right channel at this pixel offset
        let draw_right = draw_bits(Channel::Right as u8, x);
        let right_height = (self.center as f32 * right + 0.5) as u32;
        let max_y = std::cmp::min(self.center + right_height, self.height);
        for y in self.center..max_y {
            let idx = (offset + y * self.line_width) as usize;
            self.pixels[idx] |= draw_right;
        }
    }

    /// Draws a single point for mono audio (symmetric around center).
    ///
    /// # Arguments
    ///
    /// * `x` - The horizontal position to draw at
    /// * `mono` - Mono channel maximum amplitude
    pub fn draw_point_mono(&mut self, x: u32, mono: f32) {
        if x >= self.width {
            return;
        }

        // The byte offset where the 2-bit pixel will be
        let offset = x >> 2;

        // Bit position for the pixel
        let draw = draw_bits(Channel::Left as u8, x);

        let wave_height = (self.center as f32 * mono + 0.5) as u32;
        let y_start = self.center.saturating_sub(wave_height);
        let y_end = std::cmp::min(self.center + wave_height, self.height);

        for y in y_start..y_end {
            let idx = (offset + y * self.line_width) as usize;
            self.pixels[idx] |= draw;
        }
    }

    /// Saves the waveform image as a PNG file with 2-bit pixel depth optimization.
    ///
    /// # Performance
    ///
    /// This function implements several critical optimizations:
    /// - Uses 2-bit color depth instead of 8-bit (75% size reduction)
    /// - Uses indexed color mode with a minimal 3-color palette
    /// - Applies the Up filter which is optimal for waveform imagery
    /// - Uses maximum PNG compression for smallest possible files
    ///
    /// Changing these settings, especially the bit depth or filter type,
    /// would significantly impact file size or performance.
    ///
    /// # Arguments
    ///
    /// * `background` - Background color
    /// * `left` - Left channel color
    /// * `right` - Right channel color
    /// * `output_path` - Path where the PNG file will be saved
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a WaverError on failure
    pub fn save_png(
        &self,
        background: &Rgba,
        left: &Rgba,
        right: &Rgba,
        output_path: impl AsRef<Path>,
    ) -> Result<()> {
        // Create palette for indexed color PNG
        let palette = [
            background.red,
            background.green,
            background.blue,
            left.red,
            left.green,
            left.blue,
            right.red,
            right.green,
            right.blue,
            background.red,
            background.green,
            background.blue,
        ];

        // Create transparency array
        let transparent = [background.alpha, left.alpha, right.alpha, background.alpha];

        // Create the output file and BufWriter
        let file = File::create(output_path)?;
        let mut encoder = Encoder::new(BufWriter::new(file), self.width, self.height);

        // Configure the PNG encoder - use 2-bit depth since we only need 3 colors
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Two);
        encoder.set_palette(&palette);
        encoder.set_trns(&transparent);

        // Optimize for waveform imagery which typically has vertical runs
        encoder.set_filter(FilterType::Up);

        // Use maximum compression
        encoder.set_compression(png::Compression::Best);

        // Write the PNG data
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixels)?;
        writer.finish()?;

        Ok(())
    }
}

//------------------------------------------------------------------------------
// CLI Types and Arguments
//------------------------------------------------------------------------------

/// Command-line interface functionality.

/// Custom types for command-line argument validation.
///
/// These types provide strongly-typed representations of command-line arguments
/// with built-in validation. These types make invalid states unrepresentable and
/// push validation to the earliest possible point - during argument parsing.
///
/// # Design Philosophy
///
/// Rather than validate arguments after parsing, we use Rust's type system to:
/// 1. Ensure values meet constraints (e.g., minimum width/height)
/// 2. Provide clear, targeted error messages directly during parsing
/// 3. Allow functions to assume arguments are already valid
/// 4. Make the code more self-documenting
///
/// # Usage
///
/// These types implement FromStr and can be used with clap's value_parser:
/// ```
/// #[arg(value_parser = clap::value_parser!(Width))]
/// pub width: Width,
/// ```

/// A validated width value for the waveform image.
///
/// Ensures the width is at least 16 pixels.
#[derive(Debug, Clone, Copy)]
pub struct Width(u32);

impl Width {
    /// The minimum allowed width in pixels.
    pub const MIN_WIDTH: u32 = 16;

    /// Creates a new validated width.
    pub fn new(width: u32) -> Result<Self> {
        if width < Self::MIN_WIDTH {
            return Err(WaverError::argument_error(format!(
                "Width must be at least {} pixels",
                Self::MIN_WIDTH
            )));
        }
        Ok(Self(width))
    }

    /// Returns the width value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl FromStr for Width {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        let width = s
            .parse::<u32>()
            .map_err(|_| WaverError::argument_error("Width must be a positive integer"))?;
        Self::new(width)
    }
}

/// A validated height value for the waveform image.
///
/// Ensures the height is at least 6 pixels and even.
#[derive(Debug, Clone, Copy)]
pub struct Height(u32);

impl Height {
    /// The minimum allowed height in pixels.
    pub const MIN_HEIGHT: u32 = 6;

    /// Creates a new validated height.
    pub fn new(height: u32) -> Result<Self> {
        if height < Self::MIN_HEIGHT {
            return Err(WaverError::argument_error(format!(
                "Height must be at least {} pixels",
                Self::MIN_HEIGHT
            )));
        }
        if height % 2 != 0 {
            return Err(WaverError::argument_error("Height must be an even number"));
        }
        Ok(Self(height))
    }

    /// Returns the height value.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Returns the vertical center line position.
    pub fn center(&self) -> u32 {
        self.0 / 2
    }
}

impl FromStr for Height {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        let height = s
            .parse::<u32>()
            .map_err(|_| WaverError::argument_error("Height must be a positive integer"))?;
        Self::new(height)
    }
}

/// A validated audio file path.
///
/// Ensures the path exists and is a file.
#[derive(Debug, Clone)]
pub struct AudioPath(PathBuf);

impl AudioPath {
    /// Creates a new validated audio path.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(WaverError::argument_error(format!(
                "Path does not exist: {}",
                path.display()
            )));
        }
        Ok(Self(path.to_path_buf()))
    }

    /// Returns whether this path points to a directory.
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    /// Returns the path.
    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl FromStr for AudioPath {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

/// A validated audio file extension.
///
/// Ensures the extension is valid.
#[derive(Debug, Clone)]
pub struct FileExtension(String);

impl FileExtension {
    /// Creates a new validated file extension.
    pub fn new(extension: impl AsRef<str>) -> Result<Self> {
        let extension = extension.as_ref().trim().to_lowercase();
        if extension.is_empty() {
            return Err(WaverError::argument_error("File extension cannot be empty"));
        }
        Ok(Self(extension))
    }

    /// Returns the extension string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for FileExtension {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

/// A collection of validated file extensions.
#[derive(Debug, Clone)]
pub struct FileExtensions(Vec<FileExtension>);

impl FileExtensions {
    /// Creates a new collection of validated file extensions.
    pub fn new(extensions: Vec<impl AsRef<str>>) -> Result<Self> {
        let mut validated_extensions = Vec::with_capacity(extensions.len());

        for ext in extensions {
            validated_extensions.push(FileExtension::new(ext)?);
        }

        Ok(Self(validated_extensions))
    }

    /// Returns the file extensions as a vector of strings.
    pub fn as_strings(&self) -> Vec<String> {
        self.0.iter().map(|e| e.as_str().to_string()).collect()
    }
}

impl FromStr for FileExtensions {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        let extensions = s
            .split(',')
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();

        if extensions.is_empty() {
            return Err(WaverError::argument_error("No file extensions specified"));
        }

        Self::new(extensions)
    }
}

//------------------------------------------------------------------------------
// Color Handling
//------------------------------------------------------------------------------

/// Color handling functionality for waveform visualization.

/// Represents an RGBA color.
#[derive(Clone, Debug)]
pub struct Rgba {
    /// Red component (0-255)
    pub red: u8,
    /// Green component (0-255)
    pub green: u8,
    /// Blue component (0-255)
    pub blue: u8,
    /// Alpha component (0-255)
    pub alpha: u8,
}

impl FromStr for Rgba {
    type Err = WaverError;

    /// Parses a color from a string in the following formats:
    /// - RGB (3-digit hex): e.g. "F00" for bright red
    /// - RRGGBB (6-digit hex): e.g. "FF0000" for bright red
    /// - RRGGBBAA (8-digit hex): e.g. "FF0000FF" for opaque bright red
    fn from_str(color: &str) -> Result<Self> {
        let hex = color.trim();
        let value = u32::from_str_radix(hex, 16)
            .map_err(|e| WaverError::argument_error(format!("Invalid color format: {}", e)))?;

        match hex.len() {
            3 => Ok(Rgba {
                red: ((value & 0xF00) >> 8) as u8 * 17,
                green: ((value & 0x0F0) >> 4) as u8 * 17,
                blue: (value & 0x00F) as u8 * 17,
                alpha: 255,
            }),
            6 => Ok(Rgba {
                red: ((value & 0xFF0000) >> 16) as u8,
                green: ((value & 0x00FF00) >> 8) as u8,
                blue: (value & 0x0000FF) as u8,
                alpha: 255,
            }),
            8 => Ok(Rgba {
                red: ((value & 0xFF000000) >> 24) as u8,
                green: ((value & 0x00FF0000) >> 16) as u8,
                blue: ((value & 0x0000FF00) >> 8) as u8,
                alpha: (value & 0x000000FF) as u8,
            }),
            _ => Err(WaverError::argument_error(
                "Color must be in RGB, RRGGBB, or RRGGBBAA format",
            )),
        }
    }
}

//------------------------------------------------------------------------------
// Error Handling
//------------------------------------------------------------------------------

/// Error types for the waver application.
///
/// This section defines the error types used throughout the application and
/// provides a consistent approach to error handling and propagation.
///
/// # Error Handling Strategy
///
/// The waver application uses a structured error handling approach:
///
/// 1. **Custom Error Types**: All errors are consolidated into the WaverError enum
/// 2. **Context Preservation**: External errors (IO, etc.) are wrapped with context
/// 3. **Early Validation**: Most errors are caught at argument parsing time
/// 4. **Result Propagation**: Errors bubble up with the `?` operator
/// 5. **User-Friendly Messages**: Errors are formatted to be helpful to the user
///
/// This approach makes errors easier to handle, debug, and report to users.

/// Represents all possible errors that can occur in the waver application.
#[derive(Error, Debug)]
pub enum WaverError {
    /// Error when parsing or validating command line arguments.
    #[error("Invalid argument: {0}")]
    ArgumentError(String),

    /// Error during waveform generation process.
    #[error("Waveform generation error: {0}")]
    GenerationError(String),

    /// Error from the underlying IO operations.
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    /// Error from the Symphonia audio decoding library.
    #[error("Audio decoding error: {0}")]
    SymphoniaError(#[from] symphonia::core::errors::Error),

    /// Error from the PNG encoding library.
    #[error("PNG encoding error: {0}")]
    PngError(#[from] png::EncodingError),
}

/// Type alias for Result with WaverError.
pub type Result<T> = std::result::Result<T, WaverError>;

impl WaverError {
    /// Create a new ArgumentError with the given message.
    ///
    /// Use this for errors related to command-line arguments or configuration.
    pub fn argument_error(msg: impl Into<String>) -> Self {
        WaverError::ArgumentError(msg.into())
    }

    /// Create a new GenerationError with the given message.
    ///
    /// Use this for errors during the waveform generation process.
    pub fn generation_error(msg: impl Into<String>) -> Self {
        WaverError::GenerationError(msg.into())
    }
}

/// Command line arguments for waveform generation.
#[derive(Parser, Debug)]
#[command(
    name = "waver",
    about = "Generate waveform visualizations from audio files",
    version,
    author
)]
pub struct WaverArgs {
    /// Width of the output image in pixels
    #[arg(long = "width", default_value = "2048", value_parser = clap::value_parser!(Width))]
    pub width: Width,

    /// Height of the output image in pixels (must be even)
    #[arg(long = "height", default_value = "128", value_parser = clap::value_parser!(Height))]
    pub height: Height,

    /// Color for left channel (RGB, RRGGBB, or RRGGBBAA)
    #[arg(long = "left-color", default_value = "00ff99", value_parser = clap::value_parser!(Rgba))]
    pub left_color: Rgba,

    /// Color for right channel (RGB, RRGGBB, or RRGGBBAA)
    #[arg(long = "right-color", default_value = "99ff00", value_parser = clap::value_parser!(Rgba))]
    pub right_color: Rgba,

    /// Background color (RGB, RRGGBB, or RRGGBBAA)
    #[arg(long = "background-color", default_value = "ffffff00", value_parser = clap::value_parser!(Rgba))]
    pub background_color: Rgba,

    /// Output PNG file name (only in single-file mode)
    #[arg(short = 'o', long = "output-filename")]
    pub output_filename: Option<String>,

    /// Comma-separated list of audio file extensions
    #[arg(long = "file-extensions", default_value = "mp3", value_parser = clap::value_parser!(FileExtensions))]
    pub file_extensions: FileExtensions,

    /// Perform actions without generating files
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// Overwrite existing output files
    #[arg(long = "overwrite")]
    pub overwrite: bool,

    /// Suppress most output
    #[arg(long = "quiet")]
    pub quiet: bool,

    /// Print additional information
    #[arg(long = "verbose")]
    pub verbose: bool,

    /// Audio files or directories to process
    #[arg(required = true, num_args = 1.., value_parser = clap::value_parser!(AudioPath))]
    pub audio_paths: Vec<AudioPath>,
}

impl WaverArgs {
    /// Parse command-line arguments and validate them.
    pub fn parse_and_validate() -> Result<Self> {
        let args = Self::parse();
        args.validate()?;
        Ok(args)
    }

    /// Validates inter-argument constraints that can't be handled by individual type validations.
    pub fn validate(&self) -> Result<()> {
        // Validate output filename constraints
        if self.audio_paths.len() > 1 && self.output_filename.is_some() {
            return Err(WaverError::argument_error(
                "Cannot specify --output-filename with multiple audio files",
            ));
        }

        // Check directory constraints
        if self.output_filename.is_some() {
            for path in &self.audio_paths {
                if path.is_dir() {
                    return Err(WaverError::argument_error(
                        "Cannot specify --output-filename with a directory",
                    ));
                }
            }
        }

        Ok(())
    }

    /// Prints messages to stderr unless quiet mode is enabled.
    pub fn print_to_stderr(&self, message: &str) {
        if !self.quiet {
            eprintln!("{message}");
        }
    }

    /// Prints messages to stdout (usually for successful operations).
    pub fn print_to_stdout(&self, message: &str) {
        if !self.quiet {
            println!("{message}");
        }
    }

    /// Prints verbose messages if verbose mode is enabled.
    pub fn print_verbose(&self, message: &str) {
        if self.verbose {
            println!("{message}");
        }
    }

    /// Returns the width value.
    pub fn width(&self) -> u32 {
        self.width.value()
    }

    /// Returns the file extensions as strings.
    pub fn file_extensions(&self) -> Vec<String> {
        self.file_extensions.as_strings()
    }
}

/// Main entry point for the waver application.
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Parse and validate command-line arguments
    let args = WaverArgs::parse_and_validate()?;

    // Collect all audio files to process
    let mut audio_files = Vec::new();
    for audio_path in &args.audio_paths {
        let path = audio_path.path();
        if path.is_file() {
            // Directly entered file names are just used as is
            // We don't filter it to the extensions
            audio_files.push(path.to_path_buf());
        } else if path.is_dir() {
            // We use WalkDir such that the complexity of loops/etc are handled
            // for us rather than getting us stuck
            for entry in WalkDir::new(path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry| entry.file_type().is_file())
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext_str| args.file_extensions().iter().any(|e| e.eq(ext_str)))
                        .unwrap_or(false)
                })
                .map(|entry| entry.into_path())
            {
                audio_files.push(entry);
            }
        }
    }

    if audio_files.is_empty() {
        return Err(Box::new(WaverError::argument_error(
            "No matching audio files found",
        )));
    }

    if args.verbose {
        args.print_verbose(&format!(
            "Found {} audio files to process",
            audio_files.len()
        ));
    }

    // Process files in parallel, collecting errors
    // PERFORMANCE: Parallel processing is critical for handling multiple files efficiently
    // This section uses Rayon's parallel iterator to process files concurrently
    // while safely collecting errors using a synchronized Mutex
    let errors = Mutex::new(Vec::<String>::new());

    // Convert PathBuf to AudioPath for processing
    audio_files.into_par_iter().for_each(|file_path| {
        // For each file, create a validated AudioPath
        match AudioPath::new(&file_path) {
            Ok(audio_path) => {
                let output_file = args
                    .output_filename
                    .clone()
                    .unwrap_or_else(|| format!("{}.png", file_path.display()));

                if let Err(e) = generate_waveform(&audio_path, &output_file, &args) {
                    let error_msg = format!("{}: {}", file_path.display(), e);
                    args.print_to_stderr(&error_msg);
                    errors.lock().unwrap().push(error_msg);
                }
            }
            Err(e) => {
                let error_msg = format!("Invalid audio path {}: {}", file_path.display(), e);
                args.print_to_stderr(&error_msg);
                errors.lock().unwrap().push(error_msg);
            }
        }
    });

    // Report any errors
    let errors = errors.lock().unwrap();
    if !errors.is_empty() {
        return Err(Box::new(WaverError::generation_error(format!(
            "{} errors occurred while processing files",
            errors.len()
        ))));
    }

    Ok(())
}
```