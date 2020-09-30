use libc::getloadavg;

#[derive(Default, Debug, PartialEq)]
pub struct LoadAvg {
    pub loadavg1: f64,
    pub loadavg5: f64,
    pub loadavg15: f64,
}

impl LoadAvg {
    pub fn get() -> Self {
        let mut loadavgs: [f64; 3] = [0.0, 0.0, 0.0];
        let ret = unsafe { getloadavg(loadavgs.as_mut_ptr(), 3) };
        if ret != 3 {
            unimplemented!()
        }
        LoadAvg {
            loadavg1: loadavgs[0],
            loadavg5: loadavgs[1],
            loadavg15: loadavgs[2],
        }
    }
}
