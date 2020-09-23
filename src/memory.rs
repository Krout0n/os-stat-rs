use std::io::{BufRead, BufReader, Read};
#[derive(Debug, PartialEq)]
pub struct Stats {
    pub total: u64,
    pub used: u64,
    pub buffers: u64,
    pub cached: u64,
    pub free: u64,
    pub available: u64,
    pub active: u64,
    pub inactive: u64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_cached: u64,
    pub swap_free: u64,
    pub mem_avaliable_enabled: bool,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            total: 0,
            used: 0,
            buffers: 0,
            cached: 0,
            free: 0,
            available: 0,
            active: 0,
            inactive: 0,
            swap_total: 0,
            swap_used: 0,
            swap_cached: 0,
            swap_free: 0,
            mem_avaliable_enabled: false,
        }
    }
}
pub fn get() -> std::io::Result<Stats> {
    let file = std::fs::File::open("/proc/meminfo")?;
    collect_memory_stats(file)
}

pub fn collect_memory_stats<R: Read>(buf: R) -> std::io::Result<Stats> {
    let reader = BufReader::new(buf);
    let mut stats = Stats::default();

    for line in reader.lines() {
        if line.is_err() {
            unimplemented!()
        }
        let line = line.unwrap();
        if !line.contains(":") {
            continue;
        }
        let line: Vec<_> = line.split_ascii_whitespace().collect();
        let key = line[0].trim_end_matches(":");
        let val = line[1].parse::<u64>();
        if let Ok(val) = val {
            let val = val * 1024;
            match key {
                "MemTotal" => stats.total = val,
                "MemFree" => stats.free = val,
                "MemAvailable" => {
                    stats.available = val;
                    stats.mem_avaliable_enabled = true
                }
                "Buffers" => stats.buffers = val,
                "Cached" => stats.cached = val,
                "Active" => stats.active = val,
                "Inactive" => stats.inactive = val,
                "SwapCached" => stats.swap_cached = val,
                "SwapTotal" => stats.swap_total = val,
                "SwapFree" => stats.swap_free = val,
                _ => (),
            }
        }
    }

    stats.swap_used = stats.swap_total - stats.swap_free;
    stats.used = if stats.mem_avaliable_enabled {
        stats.total - stats.available
    } else {
        stats.total - stats.free - stats.buffers - stats.cached
    };

    Ok(stats)
}

#[test]
fn collect_memory_stats_mem_avaliable_disabled() {
    let buf = "MemTotal:        1929620 kB
    MemFree:          113720 kB
    Buffers:           81744 kB
    Cached:           435712 kB
    SwapCached:          504 kB
    Active:           817412 kB
    Inactive:         754140 kB
    Active(anon):     647484 kB
    Inactive(anon):   570160 kB
    Active(file):     169928 kB
    Inactive(file):   183980 kB
    Unevictable:         124 kB
    Mlocked:             124 kB
    HighTotal:       1047928 kB
    HighFree:          18692 kB
    LowTotal:         881692 kB
    LowFree:           95028 kB
    SwapTotal:       1959932 kB
    SwapFree:        1957500 kB
    Dirty:               352 kB
    Writeback:             0 kB
    AnonPages:       1053804 kB
    Mapped:           151408 kB
    Shmem:            163548 kB
    Slab:             202768 kB
    SReclaimable:     177128 kB
    SUnreclaim:        25640 kB
    KernelStack:        4624 kB
    PageTables:        15944 kB
    NFS_Unstable:          0 kB
    Bounce:                0 kB
    WritebackTmp:          0 kB
    CommitLimit:     2924740 kB
    Committed_AS:    7238800 kB
    VmallocTotal:     122880 kB
    VmallocUsed:       16344 kB
    VmallocChunk:     102740 kB
    HardwareCorrupted:     0 kB
    AnonHugePages:    145408 kB
    HugePages_Total:       0
    HugePages_Free:        0
    HugePages_Rsvd:        0
    HugePages_Surp:        0
    Hugepagesize:       2048 kB
    DirectMap4k:       24568 kB
    DirectMap2M:      888832 kB
"
    .as_bytes();
    let r = collect_memory_stats(buf);
    assert!(r.is_ok());
    let expected = Stats {
        total: (1929620 * 1024),
        used: (1298444 * 1024),
        buffers: (81744 * 1024),
        cached: (435712 * 1024),
        free: (113720 * 1024),
        active: (817412 * 1024),
        inactive: (754140 * 1024),
        swap_total: (1959932 * 1024),
        swap_used: (2432 * 1024),
        swap_cached: (504 * 1024),
        swap_free: (1957500 * 1024),
        mem_avaliable_enabled: false,
        ..Default::default()
    };
    assert_eq!(r.unwrap(), expected);
}

#[test]
fn collect_memory_stats_mem_avaliable_enabled() {
    let buf = "MemTotal:        1929620 kB
    MemFree:          113720 kB
    MemAvailable:     533132 kB
    Buffers:           81744 kB
    Cached:           435712 kB
    SwapCached:          504 kB
    Active:           817412 kB
    Inactive:         754140 kB
    Active(anon):     647484 kB
    Inactive(anon):   570160 kB
    Active(file):     169928 kB
    Inactive(file):   183980 kB
    Unevictable:         124 kB
    Mlocked:             124 kB
    HighTotal:       1047928 kB
    HighFree:          18692 kB
    LowTotal:         881692 kB
    LowFree:           95028 kB
    SwapTotal:       1959932 kB
    SwapFree:        1957500 kB
    Dirty:               352 kB
    Writeback:             0 kB
    AnonPages:       1053804 kB
    Mapped:           151408 kB
    Shmem:            163548 kB
    Slab:             202768 kB
    SReclaimable:     177128 kB
    SUnreclaim:        25640 kB
    KernelStack:        4624 kB
    PageTables:        15944 kB
    NFS_Unstable:          0 kB
    Bounce:                0 kB
    WritebackTmp:          0 kB
    CommitLimit:     2924740 kB
    Committed_AS:    7238800 kB
    VmallocTotal:     122880 kB
    VmallocUsed:       16344 kB
    VmallocChunk:     102740 kB
    HardwareCorrupted:     0 kB
    AnonHugePages:    145408 kB
    HugePages_Total:       0
    HugePages_Free:        0
    HugePages_Rsvd:        0
    HugePages_Surp:        0
    Hugepagesize:       2048 kB
    DirectMap4k:       24568 kB
    DirectMap2M:      888832 kB
"
    .as_bytes();
    let r = collect_memory_stats(buf);
    assert!(r.is_ok());
    let expected = Stats {
        total: (1929620 * 1024),
        used: (1396488 * 1024),
        buffers: (81744 * 1024),
        cached: (435712 * 1024),
        free: (113720 * 1024),
        available: (533132 * 1024),
        active: (817412 * 1024),
        inactive: (754140 * 1024),
        swap_total: (1959932 * 1024),
        swap_used: (2432 * 1024),
        swap_cached: (504 * 1024),
        swap_free: (1957500 * 1024),
        mem_avaliable_enabled: true,
        ..Default::default()
    };
    assert_eq!(r.unwrap(), expected);
}