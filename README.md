# Rust out-of-tree module

Rust out-of-tree module sample using extern crate.

## Usage
### 0. Build your kernel(riscv64) with Rust support

### 1. Compile the .ko file and put it into rootfs.img
Set your `KDIR`, `ARCH`, `CROSS_COMPILE` and `O` variables, then make.

(This might require a `sudo` to mount rootfs, see Makefile for detials)
```
make KDIR=../rust-for-linux ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- O=build
```

### 2. Run the kernel with rootfs.img on qemu
Set your `-kernel` and `rootfs.img` path.

```
qemu-system-riscv64 \
  -M virt \
  -m 256M \
  -nographic \
  -kernel ../rust-for-linux/build/arch/riscv/boot/Image \
  -drive file=rootfs.img,format=raw,id=hd0 \
  -device virtio-blk-device,drive=hd0 \
  -append "root=/dev/vda rw console=ttyS0"
```

### 3. insmod rust_out_of_tree.ko

```
➜  rust-out-of-tree-module git:(main) ✗ qemu-system-riscv64 \
  -M virt \
  -m 256M \
  -nographic \
  -kernel ../rust-for-linux/build/arch/riscv/boot/Image \
  -drive file=rootfs.img,format=raw,id=hd0 \
  -device virtio-blk-device,drive=hd0 \
  -append "root=/dev/vda rw console=ttyS0"

OpenSBI v1.0
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name             : riscv-virtio,qemu
Platform Features         : medeleg
Platform HART Count       : 1
Platform IPI Device       : aclint-mswi
Platform Timer Device     : aclint-mtimer @ 10000000Hz
Platform Console Device   : uart8250
Platform HSM Device       : ---
Platform Reboot Device    : sifive_test
Platform Shutdown Device  : sifive_test
Firmware Base             : 0x80000000
Firmware Size             : 252 KB
Runtime SBI Version       : 0.3

Domain0 Name              : root
Domain0 Boot HART         : 0
Domain0 HARTs             : 0*
Domain0 Region00          : 0x0000000002000000-0x000000000200ffff (I)
Domain0 Region01          : 0x0000000080000000-0x000000008003ffff ()
Domain0 Region02          : 0x0000000000000000-0xffffffffffffffff (R,W,X)
Domain0 Next Address      : 0x0000000080200000
Domain0 Next Arg1         : 0x000000008f000000
Domain0 Next Mode         : S-mode
Domain0 SysReset          : yes

Boot HART ID              : 0
Boot HART Domain          : root
Boot HART ISA             : rv64imafdcsuh
Boot HART Features        : scounteren,mcounteren,time
Boot HART PMP Count       : 16
Boot HART PMP Granularity : 4
Boot HART PMP Address Bits: 54
Boot HART MHPM Count      : 0
Boot HART MIDELEG         : 0x0000000000001666
Boot HART MEDELEG         : 0x0000000000f0b509
[    0.000000] Linux version 5.19.0-g459035ab65c0-dirty (xx@xx) (riscv64-linux-gnu-gcc (Ubuntu 9.4.0-1ubuntu1~20.04) 9.4.0, GNU ld (GNU Binutils for Ubuntu) 2.34) #2 SMP Fri Aug 12 17:09:41 CST 2022
[    0.000000] OF: fdt: Ignoring memory range 0x80000000 - 0x80200000
[    0.000000] Machine model: riscv-virtio,qemu
[    0.000000] efi: UEFI not found.
[    0.000000] Zone ranges:
[    0.000000]   DMA32    [mem 0x0000000080200000-0x000000008fffffff]
[    0.000000]   Normal   empty
[    0.000000] Movable zone start for each node
[    0.000000] Early memory node ranges
[    0.000000]   node   0: [mem 0x0000000080200000-0x000000008fffffff]
[    0.000000] Initmem setup node 0 [mem 0x0000000080200000-0x000000008fffffff]
[    0.000000] SBI specification v0.3 detected
[    0.000000] SBI implementation ID=0x1 Version=0x10000
[    0.000000] SBI TIME extension detected
[    0.000000] SBI IPI extension detected
[    0.000000] SBI RFENCE extension detected
[    0.000000] SBI SRST extension detected
[    0.000000] SBI HSM extension detected
[    0.000000] riscv: base ISA extensions acdfhim
[    0.000000] riscv: ELF capabilities acdfim
[    0.000000] percpu: Embedded 18 pages/cpu s34104 r8192 d31432 u73728
[    0.000000] Built 1 zonelists, mobility grouping on.  Total pages: 64135
[    0.000000] Kernel command line: root=/dev/vda rw console=ttyS0
[    0.000000] Dentry cache hash table entries: 32768 (order: 6, 262144 bytes, linear)
[    0.000000] Inode-cache hash table entries: 16384 (order: 5, 131072 bytes, linear)
[    0.000000] mem auto-init: stack:off, heap alloc:off, heap free:off
[    0.000000] Virtual kernel memory layout:
[    0.000000]       fixmap : 0xff1bfffffee00000 - 0xff1bffffff000000   (2048 kB)
[    0.000000]       pci io : 0xff1bffffff000000 - 0xff1c000000000000   (  16 MB)
[    0.000000]      vmemmap : 0xff1c000000000000 - 0xff20000000000000   (1024 TB)
[    0.000000]      vmalloc : 0xff20000000000000 - 0xff60000000000000   (16384 TB)
[    0.000000]       lowmem : 0xff60000000000000 - 0xff6000000fe00000   ( 254 MB)
[    0.000000]       kernel : 0xffffffff80000000 - 0xffffffffffffffff   (2047 MB)
[    0.000000] Memory: 235516K/260096K available (6718K kernel code, 4865K rwdata, 4096K rodata, 2172K init, 397K bss, 24580K reserved, 0K cma-reserved)
[    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=1, Nodes=1
[    0.000000] rcu: Hierarchical RCU implementation.
[    0.000000] rcu:     RCU restricting CPUs from NR_CPUS=8 to nr_cpu_ids=1.
[    0.000000] rcu:     RCU debug extended QS entry/exit.
[    0.000000]  Tracing variant of Tasks RCU enabled.
[    0.000000] rcu: RCU calculated value of scheduler-enlistment delay is 25 jiffies.
[    0.000000] rcu: Adjusting geometry for rcu_fanout_leaf=16, nr_cpu_ids=1
[    0.000000] NR_IRQS: 64, nr_irqs: 64, preallocated irqs: 0
[    0.000000] riscv-intc: 64 local interrupts mapped
[    0.000000] plic: plic@c000000: mapped 53 interrupts with 1 handlers for 2 contexts.
[    0.000000] rcu: srcu_init: Setting srcu_struct sizes based on contention.
[    0.000000] riscv_timer_init_dt: Registering clocksource cpuid [0] hartid [0]
[    0.000000] clocksource: riscv_clocksource: mask: 0xffffffffffffffff max_cycles: 0x24e6a1710, max_idle_ns: 440795202120 ns
[    0.000068] sched_clock: 64 bits at 10MHz, resolution 100ns, wraps every 4398046511100ns
[    0.005042] Console: colour dummy device 80x25
[    0.008319] Calibrating delay loop (skipped), value calculated using timer frequency.. 20.00 BogoMIPS (lpj=40000)
[    0.008455] pid_max: default: 32768 minimum: 301
[    0.010540] Mount-cache hash table entries: 512 (order: 0, 4096 bytes, linear)
[    0.010569] Mountpoint-cache hash table entries: 512 (order: 0, 4096 bytes, linear)
[    0.033018] cblist_init_generic: Setting adjustable number of callback queues.
[    0.033138] cblist_init_generic: Setting shift to 0 and lim to 1.
[    0.033452] riscv: ELF compat mode supported
[    0.033750] ASID allocator using 16 bits (65536 entries)
[    0.034518] rcu: Hierarchical SRCU implementation.
[    0.034544] rcu:     Max phase no-delay instances is 1000.
[    0.035802] EFI services will not be available.
[    0.037734] smp: Bringing up secondary CPUs ...
[    0.037803] smp: Brought up 1 node, 1 CPU
[    0.046597] devtmpfs: initialized
[    0.052261] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
[    0.052483] futex hash table entries: 256 (order: 2, 16384 bytes, linear)
[    0.056945] NET: Registered PF_NETLINK/PF_ROUTE protocol family
[    0.061398] cpuidle: using governor menu
[    0.083214] HugeTLB registered 2.00 MiB page size, pre-allocated 0 pages
[    0.087002] iommu: Default domain type: Translated
[    0.087031] iommu: DMA domain TLB invalidation policy: strict mode
[    0.087943] SCSI subsystem initialized
[    0.089842] usbcore: registered new interface driver usbfs
[    0.090119] usbcore: registered new interface driver hub
[    0.090246] usbcore: registered new device driver usb
[    0.099839] vgaarb: loaded
[    0.100960] clocksource: Switched to clocksource riscv_clocksource
[    0.115575] NET: Registered PF_INET protocol family
[    0.116425] IP idents hash table entries: 4096 (order: 3, 32768 bytes, linear)
[    0.119184] tcp_listen_portaddr_hash hash table entries: 128 (order: 0, 4096 bytes, linear)
[    0.119441] Table-perturb hash table entries: 65536 (order: 6, 262144 bytes, linear)
[    0.119481] TCP established hash table entries: 2048 (order: 2, 16384 bytes, linear)
[    0.119644] TCP bind hash table entries: 2048 (order: 4, 65536 bytes, linear)
[    0.119800] TCP: Hash tables configured (established 2048 bind 2048)
[    0.121592] UDP hash table entries: 256 (order: 2, 24576 bytes, linear)
[    0.121839] UDP-Lite hash table entries: 256 (order: 2, 24576 bytes, linear)
[    0.122830] NET: Registered PF_UNIX/PF_LOCAL protocol family
[    0.125353] RPC: Registered named UNIX socket transport module.
[    0.125396] RPC: Registered udp transport module.
[    0.125407] RPC: Registered tcp transport module.
[    0.125415] RPC: Registered tcp NFSv4.1 backchannel transport module.
[    0.125504] PCI: CLS 0 bytes, default 64
[    0.131318] workingset: timestamp_bits=62 max_order=16 bucket_order=0
[    0.143192] NFS: Registering the id_resolver key type
[    0.143924] Key type id_resolver registered
[    0.143955] Key type id_legacy registered
[    0.144239] nfs4filelayout_init: NFSv4 File Layout Driver Registering...
[    0.144313] nfs4flexfilelayout_init: NFSv4 Flexfile Layout Driver Registering...
[    0.145030] 9p: Installing v9fs 9p2000 file system support
[    0.146354] NET: Registered PF_ALG protocol family
[    0.146734] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 250)
[    0.146838] io scheduler mq-deadline registered
[    0.146905] io scheduler kyber registered
[    0.156229] pci-host-generic 30000000.pci: host bridge /soc/pci@30000000 ranges:
[    0.156915] pci-host-generic 30000000.pci:       IO 0x0003000000..0x000300ffff -> 0x0000000000
[    0.157269] pci-host-generic 30000000.pci:      MEM 0x0040000000..0x007fffffff -> 0x0040000000
[    0.157320] pci-host-generic 30000000.pci:      MEM 0x0400000000..0x07ffffffff -> 0x0400000000
[    0.157691] pci-host-generic 30000000.pci: Memory resource size exceeds max for 32 bits
[    0.158512] pci-host-generic 30000000.pci: ECAM at [mem 0x30000000-0x3fffffff] for [bus 00-ff]
[    0.159625] pci-host-generic 30000000.pci: PCI host bridge to bus 0000:00
[    0.159817] pci_bus 0000:00: root bus resource [bus 00-ff]
[    0.159884] pci_bus 0000:00: root bus resource [io  0x0000-0xffff]
[    0.159905] pci_bus 0000:00: root bus resource [mem 0x40000000-0x7fffffff]
[    0.159915] pci_bus 0000:00: root bus resource [mem 0x400000000-0x7ffffffff]
[    0.160984] pci 0000:00:00.0: [1b36:0008] type 00 class 0x060000
[    0.238026] Serial: 8250/16550 driver, 4 ports, IRQ sharing disabled
[    0.245583] printk: console [ttyS0] disabled
[    0.246842] 10000000.uart: ttyS0 at MMIO 0x10000000 (irq = 1, base_baud = 230400) is a 16550A
[    0.275811] printk: console [ttyS0] enabled
[    0.289673] loop: module loaded
[    0.290464] virtio_blk virtio0: 1/0/0 default/read/poll queues
[    0.293374] virtio_blk virtio0: [vda] 32768 512-byte logical blocks (16.8 MB/16.0 MiB)
[    0.310898] e1000e: Intel(R) PRO/1000 Network Driver
[    0.311319] e1000e: Copyright(c) 1999 - 2015 Intel Corporation.
[    0.311826] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
[    0.312207] ehci-pci: EHCI PCI platform driver
[    0.312528] ehci-platform: EHCI generic platform driver
[    0.313026] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
[    0.313337] ohci-pci: OHCI PCI platform driver
[    0.313654] ohci-platform: OHCI generic platform driver
[    0.314799] usbcore: registered new interface driver uas
[    0.315142] usbcore: registered new interface driver usb-storage
[    0.316097] mousedev: PS/2 mouse device common for all mice
[    0.318518] goldfish_rtc 101000.rtc: registered as rtc0
[    0.318989] goldfish_rtc 101000.rtc: setting system clock to 2022-08-13T11:44:55 UTC (1660391095)
[    0.321208] syscon-poweroff soc:poweroff: pm_power_off already claimed for sbi_srst_power_off
[    0.321765] syscon-poweroff: probe of soc:poweroff failed with error -16
[    0.323111] sdhci: Secure Digital Host Controller Interface driver
[    0.323597] sdhci: Copyright(c) Pierre Ossman
[    0.324010] sdhci-pltfm: SDHCI platform and OF driver helper
[    0.325040] usbcore: registered new interface driver usbhid
[    0.325326] usbhid: USB HID core driver
[    0.325824] riscv-pmu-sbi: SBI PMU extension is available
[    0.326586] riscv-pmu-sbi: 15 firmware and 2 hardware counters
[    0.326835] riscv-pmu-sbi: Perf sampling/filtering is not supported as sscof extension is not available
[    0.329369] rust_minimal: Rust minimal sample (init)
[    0.329714] rust_minimal: Am I built-in? true
[    0.330147] rust_print: Rust printing macros sample (init)
[    0.330386] rust_print: Emergency message (level 0) without args
[    0.330663] rust_print: Alert message (level 1) without args
[    0.330878] rust_print: Critical message (level 2) without args
[    0.331082] rust_print: Error message (level 3) without args
[    0.331314] rust_print: Warning message (level 4) without args
[    0.331539] rust_print: Notice message (level 5) without args
[    0.331762] rust_print: Info message (level 6) without args
[    0.331993] rust_print: A line that is continued without args
[    0.332255] rust_print: Emergency message (level 0) with args
[    0.332670] rust_print: Alert message (level 1) with args
[    0.333051] rust_print: Critical message (level 2) with args
[    0.333265] rust_print: Error message (level 3) with args
[    0.333450] rust_print: Warning message (level 4) with args
[    0.333683] rust_print: Notice message (level 5) with args
[    0.333875] rust_print: Info message (level 6) with args
[    0.334085] rust_print: A line that is continued with args
[    0.334374] rust_module_parameters: Rust module parameters sample (init)
[    0.334631] rust_module_parameters: Parameters:
[    0.334810] rust_module_parameters:   my_bool:    true
[    0.335077] rust_module_parameters:   my_i32:     42
[    0.335449] rust_module_parameters:   my_str:     default str val
[    0.335718] rust_module_parameters:   my_usize:   42
[    0.335951] rust_module_parameters:   my_array:   [0, 1]
[    0.336454] rust_sync: Rust synchronisation primitives sample (init)
[    0.336684] rust_sync: Value: 10
[    0.337240] rust_sync: Value: 10
[    0.337527] rust_chrdev: Rust character device sample (init)
[    0.338183] rust_miscdev: Rust miscellaneous device sample (init)
[    0.339255] rust_stack_probing: Rust stack probing sample (init)
[    0.339524] rust_stack_probing: Large array has length: 514
[    0.339804] rust_semaphore: Rust semaphore sample (init)
[    0.340367] rust_semaphore_c: Rust semaphore sample (in C, for comparison) (init)
[    0.343373] rust_selftests: Rust self tests (init)
[    0.343583] rust_selftests: test_example passed!
[    0.343624] rust_selftests: 1 tests run, 1 passed, 0 failed, 0 hit errors
[    0.344033] rust_selftests: All tests passed. Congratulations!
[    0.345967] NET: Registered PF_INET6 protocol family
[    0.351812] Segment Routing with IPv6
[    0.352136] In-situ OAM (IOAM) with IPv6
[    0.352559] sit: IPv6, IPv4 and MPLS over IPv4 tunneling driver
[    0.354961] NET: Registered PF_PACKET protocol family
[    0.355979] 9pnet: Installing 9P2000 support
[    0.356521] Key type dns_resolver registered
[    0.358854] debug_vm_pgtable: [debug_vm_pgtable         ]: Validating architecture page table helpers
[    0.409595] EXT4-fs (vda): mounted filesystem with ordered data mode. Quota mode: disabled.
[    0.410528] ext4 filesystem being mounted at /root supports timestamps until 2038 (0x7fffffff)
[    0.410894] VFS: Mounted root (ext4 filesystem) on device 254:0.
[    0.412910] devtmpfs: mounted
[    0.437064] Freeing unused kernel image (initmem) memory: 2172K
[    0.438158] Run /sbin/init as init process

Please press Enter to activate this console.
/ # insmod rust_out_of_tree.ko
[    9.869893] rust_out_of_tree: loading out-of-tree module taints kernel.
[    9.874058] rust_out_of_tree: Rust out-of-tree sample (init)
[    9.874272] rust_out_of_tree: 1 + 2 = 3
/ # rmmod rust_out_of_tree.ko
[   22.244617] rust_out_of_tree: My numbers are [72, 108, 200]
[   22.245129] rust_out_of_tree: Rust out-of-tree sample (exit)
/ #
```
