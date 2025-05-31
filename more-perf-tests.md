# Performance Comparison Report

<style>
  table {border-collapse:collapse;width:100%;}
  td {padding:1px 0.25em 1px 0.25em;text-wrap:nowrap;border-top:0.25px solid black;}
  th {text-align:left;}
  th span {font-size:90%;}
  th span span {float:right;font-size:80%;}
  td.bar {width:99%;}
  td.num {text-align:right;}
</style>

## Table of Contents

- [Performance Comparison Report](#performance-comparison-report)
  - [Table of Contents](#table-of-contents)
  - [Platform Details](#platform-details)
  - [Benchmark Details](#benchmark-details)
  - [Wall Clock Time (Real Time)](#wall-clock-time-real-time)
  - [User CPU Time](#user-cpu-time)
  - [Maximum Resident Set Size](#maximum-resident-set-size)
  - [Instructions Retired](#instructions-retired)
  - [CPU Cycles](#cpu-cycles)
  - [Parallelism Factor](#parallelism-factor)
  - [Effective Clock Rate](#effective-clock-rate)
  - [Instructions Per Cycle (IPC)](#instructions-per-cycle-ipc)
  - [Peak Memory Footprint](#peak-memory-footprint)
  - [Memory Efficiency (Memory per Parallelism Unit)](#memory-efficiency-memory-per-parallelism-unit)
  - [Output Size](#output-size)
  - [Executable Size](#executable-size)

## Platform Details

This table shows the hardware and software details for each platform in the benchmarks. Note that the Intel parts list their base clock rate but they turbo boost too much higher rates when thermally possible.

| Platform | HW | CPU | macOS |
|----------|----|----|-------|
| M4-Max | MacBookPro-16 | Apple M4 Max | 15.5 |
| M1-Max | MacBookPro-16 | Apple M1 Max | 15.5 |
| i9 | Late 2019<br>MacBookPro-16 | Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz | 15.5 |
| i7 | Late 2013<br>MacBookPro-15 | Intel(R) Core(TM) i7-4960HQ CPU @ 2.60GHz | 11.7.10 |

## Benchmark Details

Each test was run on the computer when it was at idle stable temperature such that it could instantly burst to full clock rate. The benchmark was run over a fixed set of test files that were repeated in a number of directories in order to get the total to be reasonably useful. This is a synthetic test set rather than a personal music library, which would be both much larger (taking significantly longer to run) and would change over time as new content is added or existing content is re-encoded.

| Number of MP3 Files | Total Size of MP3 files (bytes) |
|---------------------|----------------------------------|
| 3,116 | 21,412,291,984 |


## Wall Clock Time (Real Time)
- **real**

If you are going to look at just 1 chart from this document, this is the one.

This metric represents the actual elapsed time from start to finish. It includes all computation, I/O operations, and scheduling delays. Lower values indicate faster overall completion.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Wall Clock Time <span>(seconds)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">33.17</td><td class="bar"><div style="background:#0066CC;width:8.71%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">42.14</td><td class="bar"><div style="background:#00CC66;width:11.07%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">45.52</td><td class="bar"><div style="background:#3399FF;width:11.95%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">51.20</td><td class="bar"><div style="background:#9966FF;width:13.44%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">56.56</td><td class="bar"><div style="background:#6600CC;width:14.85%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">56.78</td><td class="bar"><div style="background:#66FF99;width:14.91%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">71.84</td><td class="bar"><div style="background:#CC0000;width:18.86%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">91.94</td><td class="bar"><div style="background:#FF3333;width:24.14%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">94.62</td><td class="bar"><div style="background:#CC0066;width:24.85%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">102.02</td><td class="bar"><div style="background:#FF9933;width:26.79%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">104.59</td><td class="bar"><div style="background:#088;width:27.46%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">116.92</td><td class="bar"><div style="background:#088;width:30.70%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">125.28</td><td class="bar"><div style="background:#FF3399;width:32.90%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">134.03</td><td class="bar"><div style="background:#CC6600;width:35.19%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">169.18</td><td class="bar"><div style="background:#333;width:44.42%;">|</div></td></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">233.82</td><td class="bar"><div style="background:#0C0;width:61.40%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">234.26</td><td class="bar"><div style="background:#088;width:61.51%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">380.84</td><td class="bar"><div style="background:#C00;width:100.00%;">|</div></td></tr>
</table>

## User CPU Time
- **user**

This metric represents the total CPU time spent in user-mode code summed across all cores. It will exceed wall clock time on multi-core systems. Lower values indicate less total CPU usage.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">User CPU Time <span>(seconds)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">506.54</td><td class="bar"><div style="background:#0066CC;width:16.93%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">656.71</td><td class="bar"><div style="background:#00CC66;width:21.95%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">699.45</td><td class="bar"><div style="background:#CC0000;width:23.38%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">704.26</td><td class="bar"><div style="background:#3399FF;width:23.54%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">762.12</td><td class="bar"><div style="background:#9966FF;width:25.48%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">866.67</td><td class="bar"><div style="background:#FF3333;width:28.97%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">876.39</td><td class="bar"><div style="background:#66FF99;width:29.30%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">880.25</td><td class="bar"><div style="background:#6600CC;width:29.43%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">921.21</td><td class="bar"><div style="background:#CC0066;width:30.80%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">965.71</td><td class="bar"><div style="background:#FF9933;width:32.28%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">1210.13</td><td class="bar"><div style="background:#FF3399;width:40.45%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">1291.98</td><td class="bar"><div style="background:#CC6600;width:43.19%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">1600.08</td><td class="bar"><div style="background:#088;width:53.49%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">1809.88</td><td class="bar"><div style="background:#088;width:60.50%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">1814.10</td><td class="bar"><div style="background:#088;width:60.65%;">|</div></td></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">1831.12</td><td class="bar"><div style="background:#0C0;width:61.21%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">2639.31</td><td class="bar"><div style="background:#333;width:88.23%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">2991.32</td><td class="bar"><div style="background:#C00;width:100.00%;">|</div></td></tr>
</table>

## Maximum Resident Set Size
- **maximum_resident_set_size**

This metric shows the maximum amount of physical memory used by the process. Lower values indicate more memory-efficient code.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Maximum Resident Set Size <span>(MB)<span>[shorter is better]</span></span></th></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">7.08</td><td class="bar"><div style="background:#0C0;width:0.22%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">21.33</td><td class="bar"><div style="background:#CC0000;width:0.67%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">22.23</td><td class="bar"><div style="background:#FF3333;width:0.69%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">31.35</td><td class="bar"><div style="background:#088;width:0.98%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">33.66</td><td class="bar"><div style="background:#3399FF;width:1.05%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">34.28</td><td class="bar"><div style="background:#0066CC;width:1.07%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">120.46</td><td class="bar"><div style="background:#C00;width:3.76%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">437.74</td><td class="bar"><div style="background:#FF3399;width:13.67%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">442.89</td><td class="bar"><div style="background:#CC0066;width:13.83%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">703.58</td><td class="bar"><div style="background:#66FF99;width:21.97%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">736.98</td><td class="bar"><div style="background:#00CC66;width:23.02%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">744.14</td><td class="bar"><div style="background:#333;width:23.24%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">1560.16</td><td class="bar"><div style="background:#CC6600;width:48.72%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">2425.97</td><td class="bar"><div style="background:#088;width:75.76%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">3050.23</td><td class="bar"><div style="background:#9966FF;width:95.26%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">3103.19</td><td class="bar"><div style="background:#088;width:96.91%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">3165.58</td><td class="bar"><div style="background:#FF9933;width:98.86%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">3202.09</td><td class="bar"><div style="background:#6600CC;width:100.00%;">|</div></td></tr>
</table>

## Instructions Retired
- **instructions_retired**

This metric counts the total number of CPU instructions executed to complete the task. Lower values can indicate less computational work.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Instructions Retired <span>(billions)<span>[shorter is better]</span></span></th></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">7546.22</td><td class="bar"><div style="background:#CC0000;width:37.69%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">7548.15</td><td class="bar"><div style="background:#0066CC;width:37.70%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">7585.56</td><td class="bar"><div style="background:#088;width:37.89%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">7715.32</td><td class="bar"><div style="background:#088;width:38.54%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">7978.48</td><td class="bar"><div style="background:#088;width:39.85%;">|</div></td></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">7983.54</td><td class="bar"><div style="background:#0C0;width:39.88%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">11450.33</td><td class="bar"><div style="background:#CC0066;width:57.19%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">11453.92</td><td class="bar"><div style="background:#00CC66;width:57.21%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">11941.89</td><td class="bar"><div style="background:#3399FF;width:59.65%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">11951.00</td><td class="bar"><div style="background:#FF3333;width:59.69%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">12543.85</td><td class="bar"><div style="background:#C00;width:62.66%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">12552.53</td><td class="bar"><div style="background:#333;width:62.70%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">12693.00</td><td class="bar"><div style="background:#6600CC;width:63.40%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">12699.92</td><td class="bar"><div style="background:#CC6600;width:63.44%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">12738.43</td><td class="bar"><div style="background:#9966FF;width:63.63%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">12745.50</td><td class="bar"><div style="background:#FF9933;width:63.66%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">20017.47</td><td class="bar"><div style="background:#66FF99;width:99.99%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">20020.31</td><td class="bar"><div style="background:#FF3399;width:100.00%;">|</div></td></tr>
</table>

## CPU Cycles
- **cycles_elapsed**

This metric counts the total number of CPU cycles used to complete the task. Lower values indicate less computational work or better CPU efficiency.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">CPU Cycles <span>(billions)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">1827.85</td><td class="bar"><div style="background:#0066CC;width:19.86%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">2014.34</td><td class="bar"><div style="background:#CC0000;width:21.89%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">2316.65</td><td class="bar"><div style="background:#00CC66;width:25.18%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">2488.38</td><td class="bar"><div style="background:#3399FF;width:27.04%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">2510.44</td><td class="bar"><div style="background:#FF3333;width:27.28%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">2646.46</td><td class="bar"><div style="background:#CC0066;width:28.76%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">2689.17</td><td class="bar"><div style="background:#9966FF;width:29.23%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">2787.87</td><td class="bar"><div style="background:#FF9933;width:30.30%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">3032.01</td><td class="bar"><div style="background:#66FF99;width:32.95%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">3143.68</td><td class="bar"><div style="background:#6600CC;width:34.17%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">3487.63</td><td class="bar"><div style="background:#FF3399;width:37.90%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">3726.11</td><td class="bar"><div style="background:#CC6600;width:40.49%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">5147.90</td><td class="bar"><div style="background:#088;width:55.95%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">5171.81</td><td class="bar"><div style="background:#088;width:56.21%;">|</div></td></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">5679.25</td><td class="bar"><div style="background:#0C0;width:61.72%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">5882.56</td><td class="bar"><div style="background:#088;width:63.93%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">8719.05</td><td class="bar"><div style="background:#333;width:94.76%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">9201.45</td><td class="bar"><div style="background:#C00;width:100.00%;">|</div></td></tr>
</table>

## Parallelism Factor
- **parallelism_factor**

This metric is calculated as User Time / Real Time. It reflects how well the program utilizes multiple CPU cores. Higher values indicate better utilization and typically correlate with the number of CPU cores available. The M1/M4 have efficiency and performance cores, which may affect this value.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Parallelism Factor <span>(ratio)<span>[higher is better]</span></span></th></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">15.60</td><td class="bar"><div style="background:#333;width:100.00%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">15.58</td><td class="bar"><div style="background:#00CC66;width:99.89%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">15.56</td><td class="bar"><div style="background:#6600CC;width:99.76%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">15.52</td><td class="bar"><div style="background:#088;width:99.46%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">15.47</td><td class="bar"><div style="background:#3399FF;width:99.17%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">15.43</td><td class="bar"><div style="background:#66FF99;width:98.94%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">15.30</td><td class="bar"><div style="background:#088;width:98.06%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">15.27</td><td class="bar"><div style="background:#0066CC;width:97.89%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">14.89</td><td class="bar"><div style="background:#9966FF;width:95.41%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">9.74</td><td class="bar"><div style="background:#CC0000;width:62.41%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">9.74</td><td class="bar"><div style="background:#CC0066;width:62.41%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">9.66</td><td class="bar"><div style="background:#FF3399;width:61.92%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">9.64</td><td class="bar"><div style="background:#CC6600;width:61.79%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">9.47</td><td class="bar"><div style="background:#FF9933;width:60.68%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">9.43</td><td class="bar"><div style="background:#FF3333;width:60.42%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">7.85</td><td class="bar"><div style="background:#C00;width:50.35%;">|</div></td></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">7.83</td><td class="bar"><div style="background:#0C0;width:50.20%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">7.73</td><td class="bar"><div style="background:#088;width:49.52%;">|</div></td></tr>
</table>

## Effective Clock Rate
- **effective_clock_rate**

This metric is calculated as Cycles Elapsed / User Time. It shows the average CPU frequency during execution. Higher values indicate the CPU is running at higher clock speeds. Thermal throttling may reduce this value in longer-running programs.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Effective Clock Rate <span>(GHz)<span>[higher is better]</span></span></th></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">3.61</td><td class="bar"><div style="background:#0066CC;width:100.00%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">3.57</td><td class="bar"><div style="background:#6600CC;width:98.97%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">3.53</td><td class="bar"><div style="background:#3399FF;width:97.92%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">3.53</td><td class="bar"><div style="background:#9966FF;width:97.78%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">3.53</td><td class="bar"><div style="background:#00CC66;width:97.76%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">3.46</td><td class="bar"><div style="background:#66FF99;width:95.88%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">3.30</td><td class="bar"><div style="background:#333;width:91.55%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">3.24</td><td class="bar"><div style="background:#088;width:89.86%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">3.23</td><td class="bar"><div style="background:#088;width:89.57%;">|</div></td></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">3.10</td><td class="bar"><div style="background:#0C0;width:85.95%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">3.08</td><td class="bar"><div style="background:#C00;width:85.24%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">2.90</td><td class="bar"><div style="background:#FF3333;width:80.27%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">2.89</td><td class="bar"><div style="background:#FF9933;width:80.00%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">2.88</td><td class="bar"><div style="background:#CC6600;width:79.92%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">2.88</td><td class="bar"><div style="background:#FF3399;width:79.87%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">2.88</td><td class="bar"><div style="background:#CC0000;width:79.81%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">2.87</td><td class="bar"><div style="background:#CC0066;width:79.61%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">2.84</td><td class="bar"><div style="background:#088;width:78.82%;">|</div></td></tr>
</table>

## Instructions Per Cycle (IPC)
- **instructions_per_cycle**

This metric is calculated as Instructions Retired / Cycles Elapsed. Higher values indicate better CPU efficiency, with more instructions executed per cycle. Modern CPU architectures with better instruction-level parallelism achieve higher IPC values.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Instructions Per Cycle <span>(ratio)<span>[higher is better]</span></span></th></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">6.60</td><td class="bar"><div style="background:#66FF99;width:100.00%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">5.74</td><td class="bar"><div style="background:#FF3399;width:86.95%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">4.94</td><td class="bar"><div style="background:#00CC66;width:74.89%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">4.80</td><td class="bar"><div style="background:#3399FF;width:72.69%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">4.76</td><td class="bar"><div style="background:#FF3333;width:72.11%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">4.74</td><td class="bar"><div style="background:#9966FF;width:71.75%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">4.57</td><td class="bar"><div style="background:#FF9933;width:69.25%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">4.33</td><td class="bar"><div style="background:#CC0066;width:65.54%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">4.13</td><td class="bar"><div style="background:#0066CC;width:62.55%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">4.04</td><td class="bar"><div style="background:#6600CC;width:61.16%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">3.75</td><td class="bar"><div style="background:#CC0000;width:56.74%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">3.41</td><td class="bar"><div style="background:#CC6600;width:51.63%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">1.49</td><td class="bar"><div style="background:#088;width:22.60%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">1.47</td><td class="bar"><div style="background:#088;width:22.32%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">1.44</td><td class="bar"><div style="background:#333;width:21.81%;">|</div></td></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">1.41</td><td class="bar"><div style="background:#0C0;width:21.29%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">1.36</td><td class="bar"><div style="background:#C00;width:20.65%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">1.36</td><td class="bar"><div style="background:#088;width:20.54%;">|</div></td></tr>
</table>

## Peak Memory Footprint
- **peak_memory_footprint**

This metric shows the peak memory usage of the process. It is influenced by two main factors: (1) code efficiency in handling 'live' data and streaming processing, and (2) degree of parallelism. Higher parallelism typically requires more memory as multiple instances of the same code run concurrently. This explains why the M4-Max uses more memory than the M1-Max (due to higher parallelism) and why the i7 uses significantly less memory (due to fewer cores).

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Peak Memory Footprint <span>(MB)<span>[shorter is better]</span></span></th></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">6.00</td><td class="bar"><div style="background:#0C0;width:0.19%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">20.49</td><td class="bar"><div style="background:#FF3333;width:0.65%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">20.50</td><td class="bar"><div style="background:#CC0000;width:0.65%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">26.94</td><td class="bar"><div style="background:#088;width:0.85%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">30.11</td><td class="bar"><div style="background:#0066CC;width:0.95%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">32.71</td><td class="bar"><div style="background:#3399FF;width:1.03%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">77.16</td><td class="bar"><div style="background:#C00;width:2.43%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">88.42</td><td class="bar"><div style="background:#CC0066;width:2.79%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">143.21</td><td class="bar"><div style="background:#333;width:4.52%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">160.03</td><td class="bar"><div style="background:#00CC66;width:5.05%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">437.63</td><td class="bar"><div style="background:#FF3399;width:13.81%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">704.61</td><td class="bar"><div style="background:#66FF99;width:22.23%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">1531.82</td><td class="bar"><div style="background:#CC6600;width:48.33%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">2377.90</td><td class="bar"><div style="background:#088;width:75.02%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">3054.65</td><td class="bar"><div style="background:#9966FF;width:96.37%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">3097.75</td><td class="bar"><div style="background:#088;width:97.73%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">3164.89</td><td class="bar"><div style="background:#6600CC;width:99.85%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">3169.61</td><td class="bar"><div style="background:#FF9933;width:100.00%;">|</div></td></tr>
</table>

## Memory Efficiency (Memory per Parallelism Unit)
- **memory_per_parallelism_unit**

This metric shows peak memory footprint divided by parallelism factor, indicating how efficiently memory is used regardless of the degree of parallelism. Lower values suggest better memory efficiency per unit of parallelism. This helps compare memory usage more fairly across platforms with different core counts.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Memory Efficiency <span>(MB/unit)<span>[shorter is better]</span></span></th></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">0.77</td><td class="bar"><div style="background:#0C0;width:0.19%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">1.74</td><td class="bar"><div style="background:#088;width:0.43%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">1.97</td><td class="bar"><div style="background:#0066CC;width:0.49%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">2.11</td><td class="bar"><div style="background:#CC0000;width:0.53%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">2.11</td><td class="bar"><div style="background:#3399FF;width:0.53%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">2.17</td><td class="bar"><div style="background:#FF3333;width:0.54%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">9.08</td><td class="bar"><div style="background:#CC0066;width:2.27%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">9.18</td><td class="bar"><div style="background:#333;width:2.29%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">9.82</td><td class="bar"><div style="background:#C00;width:2.45%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">10.27</td><td class="bar"><div style="background:#00CC66;width:2.56%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">45.31</td><td class="bar"><div style="background:#FF3399;width:11.30%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">45.65</td><td class="bar"><div style="background:#66FF99;width:11.39%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">155.43</td><td class="bar"><div style="background:#088;width:38.77%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">158.91</td><td class="bar"><div style="background:#CC6600;width:39.63%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">203.36</td><td class="bar"><div style="background:#6600CC;width:50.72%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">205.21</td><td class="bar"><div style="background:#9966FF;width:51.18%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">334.85</td><td class="bar"><div style="background:#FF9933;width:83.51%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">400.95</td><td class="bar"><div style="background:#088;width:100.00%;">|</div></td></tr>
</table>

## Output Size
- **total_size_produced**

This metric shows the total size of PNG files produced by the benchmark. Swift produces significantly larger output files due to its dependence on macOS media libraries, which don't support generating indexed 4-color (2-bit) images. In contrast, the Rust and C implementations use custom PNG encoding that creates more compact indexed-color images.

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Output Size <span>(KB)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">12728.74</td><td class="bar"><div style="background:#00CC66;width:28.09%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">12728.74</td><td class="bar"><div style="background:#66FF99;width:28.09%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">12728.74</td><td class="bar"><div style="background:#CC0066;width:28.09%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">12728.74</td><td class="bar"><div style="background:#FF3399;width:28.09%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">12728.74</td><td class="bar"><div style="background:#333;width:28.09%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">12728.74</td><td class="bar"><div style="background:#C00;width:28.09%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">13221.70</td><td class="bar"><div style="background:#0066CC;width:29.17%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">13221.70</td><td class="bar"><div style="background:#3399FF;width:29.17%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">13221.70</td><td class="bar"><div style="background:#CC0000;width:29.17%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">13221.70</td><td class="bar"><div style="background:#FF3333;width:29.17%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">13221.70</td><td class="bar"><div style="background:#088;width:29.17%;">|</div></td></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">13221.70</td><td class="bar"><div style="background:#0C0;width:29.17%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">45318.93</td><td class="bar"><div style="background:#6600CC;width:99.99%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">45318.93</td><td class="bar"><div style="background:#CC6600;width:99.99%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">45321.22</td><td class="bar"><div style="background:#9966FF;width:100.00%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">45321.22</td><td class="bar"><div style="background:#FF9933;width:100.00%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">45321.22</td><td class="bar"><div style="background:#088;width:100.00%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">45321.22</td><td class="bar"><div style="background:#088;width:100.00%;">|</div></td></tr>
</table>

## Executable Size
- **size**

This shows the file size of the compiled program executable. The Swift executables should have been significantly smaller due to leveraging macOS system libraries for media and image processing but it is the C executables that are the most compact, utilizing only a few shared C, math, and threading libraries while natively implementing all of the media code. In contrast, the Rust executables were expected to be larger because they are fully statically linked and don't offload functionality to shared libraries (other than kernel OS calls).

<table>
<tr><th>Platform</th><th>Version</th><th colspan="2">Executable Size <span>(KB)<span>[shorter is better]</span></span></th></tr>
<tr><td>M4-Max</td><td>c-x86</td><td class="num">87.60</td><td class="bar"><div style="background:#66FF99;width:5.47%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c-x86</td><td class="num">87.60</td><td class="bar"><div style="background:#FF3399;width:5.47%;">|</div></td></tr>
<tr><td>i9</td><td>c-x86</td><td class="num">87.60</td><td class="bar"><div style="background:#333;width:5.47%;">|</div></td></tr>
<tr><td>i7</td><td>c-x86</td><td class="num">87.60</td><td class="bar"><div style="background:#C00;width:5.47%;">|</div></td></tr>
<tr><td>M4-Max</td><td>c</td><td class="num">100.43</td><td class="bar"><div style="background:#00CC66;width:6.27%;">|</div></td></tr>
<tr><td>M1-Max</td><td>c</td><td class="num">100.43</td><td class="bar"><div style="background:#CC0066;width:6.27%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift</td><td class="num">726.68</td><td class="bar"><div style="background:#6600CC;width:45.37%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift</td><td class="num">726.68</td><td class="bar"><div style="background:#CC6600;width:45.37%;">|</div></td></tr>
<tr><td>M4-Max</td><td>swift-x86</td><td class="num">752.45</td><td class="bar"><div style="background:#9966FF;width:46.98%;">|</div></td></tr>
<tr><td>M1-Max</td><td>swift-x86</td><td class="num">752.45</td><td class="bar"><div style="background:#FF9933;width:46.98%;">|</div></td></tr>
<tr><td>i9</td><td>swift-x86</td><td class="num">752.45</td><td class="bar"><div style="background:#088;width:46.98%;">|</div></td></tr>
<tr><td>i7</td><td>swift-x86</td><td class="num">752.45</td><td class="bar"><div style="background:#088;width:46.98%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust</td><td class="num">1444.90</td><td class="bar"><div style="background:#0066CC;width:90.20%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust</td><td class="num">1444.90</td><td class="bar"><div style="background:#CC0000;width:90.20%;">|</div></td></tr>
<tr><td>M4-Max</td><td>rust-x86</td><td class="num">1601.80</td><td class="bar"><div style="background:#3399FF;width:100.00%;">|</div></td></tr>
<tr><td>M1-Max</td><td>rust-x86</td><td class="num">1601.80</td><td class="bar"><div style="background:#FF3333;width:100.00%;">|</div></td></tr>
<tr><td>i9</td><td>rust-x86</td><td class="num">1601.80</td><td class="bar"><div style="background:#088;width:100.00%;">|</div></td></tr>
<tr><td>i7</td><td>rust-x86</td><td class="num">1601.80</td><td class="bar"><div style="background:#0C0;width:100.00%;">|</div></td></tr>
</table>
