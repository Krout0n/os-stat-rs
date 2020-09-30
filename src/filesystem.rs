use std::io::{BufRead, BufReader, Read};
use std::process::Command;

#[derive(Default, Debug, PartialEq)]
pub struct FileSystem {
    pub name: String,
    pub used: u64,
    pub size: u64,
}

impl FileSystem {
    pub fn get() -> std::io::Result<Vec<FileSystem>> {
        let output = Command::new("df")
            .arg("-Pkl")
            .output()
            .expect("failed to run");
        Self::collect_filesystem_stats(std::io::Cursor::new(output.stdout))
    }

    fn collect_filesystem_stats<R: Read>(buf: R) -> std::io::Result<Vec<FileSystem>> {
        let reader = BufReader::new(buf);
        let file_systems = reader
            .lines()
            .skip(1)
            .map(|line| line.unwrap())
            .filter(|line| {
                line.starts_with("/dev/")
                    && !line.starts_with("/dev/mapper/docker-")
                    && !line.starts_with("/dev/dm-")
                    && !line.contains("devicemapper/mnt")
            })
            .map(|line| {
                let columns: Vec<_> = line.split_ascii_whitespace().collect();
                if columns.len() < 4 {
                    unimplemented!()
                }
                let used_kb = columns[2].parse::<u64>().unwrap();
                let available_kb = columns[3].parse::<u64>().unwrap();
                FileSystem {
                    name: columns[0].trim_start_matches("/dev/").to_owned(),
                    used: used_kb * 1024,
                    size: (used_kb + available_kb) * 1024,
                }
            })
            .collect();
        Ok(file_systems)
    }
}

#[test]
fn test_collect_filesystem_stats() {
    let output = "Filesystem                         1024-blocks     Used Available Capacity Mounted on
/dev/sda1                             19734388 16868164 1863772        91% /
tmpfs                                   517224        0  517224         0% /lib/init/rw
udev                                    512780       96  512684         1% /dev
tmpfs                                   517224        4  517220         1% /dev/shm
/dev/mapper/docker-000:0-000-00000    10190136   168708 9480756         2% /var/lib/docker/devicemapper/mnt/00000
/dev/dm-4                             10474496   149684 10324812        2% /var/lib/docker/devicemapper/mnt/11111".as_bytes();
    let expected = vec![FileSystem {
        name: "sda1".to_owned(),
        used: 17272999936,
        size: 19181502464,
    }];
    let r = FileSystem::collect_filesystem_stats(output);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), expected);
}
