use std::io::{BufRead, BufReader, Read};

#[derive(Default, Debug, PartialEq)]
pub struct Network {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

pub fn get() -> std::io::Result<Vec<Network>> {
    let file = std::fs::File::open("/proc/net/dev")?;
    collect_network_stats(file)
}

fn collect_network_stats<R: Read>(buf: R) -> std::io::Result<Vec<Network>> {
    let reader = BufReader::new(buf);
    let networks = reader
        .lines()
        .skip(2)
        .map(|line| {
            let line = line.unwrap();
            let columns: Vec<_> = line.split(":").collect();
            if columns.len() < 2 {
                unimplemented!();
            }
            let name = columns[0].trim_start();
            let columns: Vec<_> = columns[1].split_ascii_whitespace().collect();
            if columns.len() < 9 {
                unimplemented!();
            }
            Network {
                name: name.to_owned(),
                rx_bytes: columns[0].parse::<u64>().unwrap(),
                tx_bytes: columns[8].parse::<u64>().unwrap(),
            }
        })
        .filter(|network| network.name != "lo")
        .collect();
    Ok(networks)
}

#[test]
fn test_collect_network_stats() {
    let buf = "Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
 wlan0: 1188035151  850857    0    0    0     0          0         0 49774221  428282    0    0    0     0       0          0
    lo: 1292817    9913    0    0    0     0          0         0  1292817    9913    0    0    0     0       0          0
  eth0: 26054426   73542    0    0    0     0          0         0 12352148   58473    0    0    0     0       0          0
  eth1:183651236    3482    0    0    0     0          0         0 93127469    1924    0    0    0     0       0          0".as_bytes();
    let expected = vec![
        Network {
            name: "wlan0".to_owned(),
            rx_bytes: 1188035151,
            tx_bytes: 49774221,
        },
        Network {
            name: "eth0".to_owned(),
            rx_bytes: 26054426,
            tx_bytes: 12352148,
        },
        Network {
            name: "eth1".to_owned(),
            rx_bytes: 183651236,
            tx_bytes: 93127469,
        },
    ];
    let r = collect_network_stats(buf);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), expected);
}
