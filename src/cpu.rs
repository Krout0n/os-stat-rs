use std::io::{BufRead, BufReader, Read};

#[derive(Debug, PartialEq)]
pub struct Stat {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    pub guest: u64,
    pub guest_nice: u64,
    pub total: u64,
    pub cpu_count: i64,
    pub stat_count: i64,
}

impl Default for Stat {
    fn default() -> Self {
        Self {
            user: 0,
            nice: 0,
            system: 0,
            idle: 0,
            iowait: 0,
            irq: 0,
            softirq: 0,
            steal: 0,
            guest: 0,
            guest_nice: 0,
            total: 0,
            cpu_count: 0,
            stat_count: 0,
        }
    }
}

pub fn get() -> std::io::Result<Stat> {
    let file = std::fs::File::open("/proc/stat")?;
    collect_cpu_stats(file)
}

pub fn collect_cpu_stats<R: Read>(file: R) -> std::io::Result<Stat> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    // the first token is "cpu", so skip it
    let mut iterator = line.split_ascii_whitespace();
    iterator.next();

    let vals: Vec<_> = iterator
        .map(|val| {
            let val = val.parse::<u64>();
            if val.is_ok() {
                val.unwrap()
            } else {
                unimplemented!()
            }
        })
        .collect();

    // TODO: assignment by accessing the slice directly is unsafe.
    let mut stat = Stat::default();
    stat.user = vals[0];
    stat.nice = vals[1];
    stat.system = vals[2];
    stat.idle = vals[3];
    stat.iowait = vals[4];
    stat.irq = vals[5];
    stat.softirq = vals[6];
    stat.steal = vals[7];
    stat.guest = vals[8];
    stat.guest_nice = vals[9];

    stat.stat_count = vals.len() as i64;
    stat.total = vals.into_iter().fold_first(|acc, val| acc + val).unwrap();
    stat.total -= stat.guest;
    stat.total -= stat.guest_nice;

    for line in reader.lines() {
        if line.unwrap().starts_with("cpu") {
            stat.cpu_count += 1;
        }
    }

    Ok(stat)
}

#[test]
fn test_collect_cpu_stats() {
    let r = collect_cpu_stats("cpu  1415984 38486 429451 2500643 10585 157 2372 0 0 0
cpu0 708614 19410 217184 2188812 9733 144 808 0 0 0
cpu1 707370 19076 212266 311830 851 12 1564 0 0 0
intr 40269386 11401108 2407 0 0 0 0 0 0 1 2601 0 0 914 0 0 0 360 0 0 21183 0 54 0 16365 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 839980 2127556 1919962 429 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0

ctxt 151685704
btime 1507943277
processes 28087
procs_running 8
procs_blocked 0
softirq 10624366 42 5280893 11772 27757 826862 2 24721 2326791 28519 2097007".as_bytes());
    assert!(r.is_ok());
    let stats = r.unwrap();

    let expected = Stat {
        user: 1415984,
        nice: 38486,
        system: 429451,
        idle: 2500643,
        iowait: 10585,
        irq: 157,
        softirq: 2372,
        steal: 0,
        guest: 0,
        guest_nice: 0,
        total: 4397678,
        cpu_count: 2,
        stat_count: 10,
    };

    assert_eq!(stats, expected);
}
