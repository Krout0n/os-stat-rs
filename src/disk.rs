use std::io::{BufRead, BufReader, Read};

#[derive(Default, Debug, PartialEq)]
pub struct Disk {
    pub name: String,
    pub reads_completed: u64,
    pub writes_completed: u64,
}

pub fn get() -> std::io::Result<Vec<Disk>> {
    let file = std::fs::File::open("/proc/diskstats")?;
    collect_disk_stats(file)
}


pub fn collect_disk_stats<R: Read>(buf: R) -> std::io::Result<Vec<Disk>> {
    let reader = BufReader::new(buf);
    let mut disks: Vec<Disk> = vec![];
    for line in reader.lines() {
        if line.is_err() {
            unimplemented!()
        }
        let line = line.unwrap();
        let fields: Vec<_> = line.split_ascii_whitespace().collect();
        if fields.len() < 14 {
            continue;
        }
        let name = fields[2].to_owned();
        let reads_completed = fields[3].parse::<u64>().unwrap();
        let writes_completed = fields[7].parse::<u64>().unwrap();
        disks.push(Disk{
            name,
            reads_completed,
            writes_completed,
        })
    }
    Ok(disks)
}
#[test]
fn test_collect_disk_stats() {
    let buf = " 202       1 xvda1 750193 3037 28116978 368712 16600606 7233846 424712632 23987908 0 2355636 24345740
202       2 xvda2 1641 9310 87552 1252 6365 3717 80664 24192 0 15040 25428
  7       0 loop0 0 0 0 0 0 0 0 0 0 0 0
  7       1 loop1 0 0 0 0 0 0 0 0 0 0 0
253       0 dm-0 46095806 0 549095028 2243928 7192424 0 305024576 12521088 0 2728444 14782668
253     628 dm-628 3198 0 75410 1360 30802835 0 3942653176 1334317408 0 70948 1358596768
253       2 dm-2 2022 0 42250 488 30822403 0 3942809696 1364721232 0 93348 1382989868
".as_bytes();
    let expected = vec![
        Disk {
            name: "xvda1".to_owned(),
            reads_completed: 750193,
            writes_completed: 16600606,
        },
        Disk {
            name: "xvda2".to_owned(),
            reads_completed: 1641,
            writes_completed: 6365,
        },
        Disk {
            name: "loop0".to_owned(),
            reads_completed: 0,
            writes_completed: 0,
        },
        Disk {
            name: "loop1".to_owned(),
            reads_completed: 0,
            writes_completed: 0,
        },
        Disk {
            name: "dm-0".to_owned(),
            reads_completed: 46095806,
            writes_completed: 7192424,
        },
        Disk {
            name: "dm-628".to_owned(),
            reads_completed: 3198,
            writes_completed: 30802835,
        },
        Disk {
            name: "dm-2".to_owned(),
            reads_completed: 2022,
            writes_completed: 30822403,
        },
    ];
    let r = collect_disk_stats(buf);
    assert!(r.is_ok());
    let stats = r.unwrap();
    assert_eq!(stats, expected);
}