#[allow(dead_code)]
#[cfg(windows)]
pub mod env {
    #[cfg(feature = "std_use_time")]
    use std::time::{self, Duration, SystemTime};

    use crate::renderer::cfg;
    pub struct TimerE {
        id: u64,
        program_start_time: time::SystemTime,
        last_time: u128,
        fps_smooth: f64,
        fps_smooth_factor: f64,
    }

    #[derive(Clone)]
    pub struct UtcTimeD {
        pub year: u64,
        pub mounth: u64,
        pub day: u64,
        pub hour: u64,
        pub min: u64,
        pub sec: u64,
        pub offset_day: i64,
        pub time_zone: u64,
        pub is_odd: bool,
    }

    pub enum ETimeAs {
        Day(Duration),
        Year(Duration),
        Month(Duration),
        Min(Duration),
        Hour(Duration),
        None,
    }

    impl Default for TimerE {
        fn default() -> Self {
            let _fps = match cfg::env::RENDERER::DEFAULT_RENDER_FRAME_STRIDE {
                0 => 1.0,
                _ => {
                    1000_0000_00.0
                        / (cfg::env::RENDERER::DEFAULT_RENDER_FRAME_STRIDE as f64)
                }
            };
            Self {
                id: 0,
                program_start_time: time::SystemTime::now(),
                last_time: 0,
                fps_smooth: 0.0,
                fps_smooth_factor: 1.0 - (1.0 / _fps),
            }
        }
    }

    impl TimerE {

        
        pub fn build_smooth_fps_factor(mut self, factor: f64) -> Self {
            self.fps_smooth_factor = factor.fract();
            return self;
        }

        /// use it to get current fps
        pub fn fps(&mut self) -> f64 {
            let _now = self.get_programtime().as_micros();
            let _fps = f64::from(1000000.0 / ((_now - self.last_time) as f64));
            let _fps = ((_fps * 100.0).ceil()) / 100.0;

            self.last_time = _now;
            return _fps;
        }

        pub fn fps_smooth(&mut self) -> f64 {
            self.fps_smooth = self.fps_smooth_factor * self.fps_smooth
                + (1.0 - self.fps_smooth_factor) * self.fps();
            self.fps_smooth = ((self.fps_smooth * 100.0).ceil()) / 100.0;

            return self.fps_smooth;
        }

        pub fn delta_time_ns(&self) -> u64 {
            let _now = self.get_programtime().as_micros();
            let _r: u64 = (_now - self.last_time) as u64;

            return _r;
        }

        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }

        pub fn set_smooth_fps_factor(&mut self, factor: f64) {
            self.fps_smooth_factor = factor.fract();
        }

        pub fn id_mut(&mut self) -> &mut u64 {
            return &mut self.id;
        }

        pub fn build(self) -> Self {
            return self;
        }

        pub fn init(&self) {}

        pub fn get_programtime(&self) -> Duration {
            return self.program_start_time.elapsed().unwrap();
        }

        pub fn systime(&self) -> Duration {
            return SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
        }

        pub fn get_runing_time(&self) -> Duration {
            return SystemTime::now()
                .duration_since(self.program_start_time)
                .unwrap();
        }

        /*

        */
        pub fn get_systime_as(vin_now: ETimeAs) -> u32 {
            match vin_now {
                ETimeAs::Day(ref val) => return u32::try_from(val.as_secs() / 86400).unwrap(),
                ETimeAs::Year(ref _val) => {
                    println!("fk dont use it")
                }
                ETimeAs::Month(ref _val) => {
                    println!("fk dont use it")
                }
                ETimeAs::Min(ref val) => {
                    return u32::try_from(val.as_secs() / 60).unwrap();
                }
                ETimeAs::Hour(ref val) => return u32::try_from(val.as_secs() / 3600).unwrap(),
                ETimeAs::None => {}
            }
            return 0;
        }

        pub fn get_systime_utc() -> UtcTimeD {
            return UtcTimeD::build();
        }
    }

    #[allow(dead_code)]

    impl UtcTimeD {
        pub fn build() -> Self {
            return UtcTimeD {
                year: 0,
                mounth: 0,
                day: 0,
                hour: 0,
                min: 0,
                sec: 0,
                offset_day: 0,
                time_zone: 8,
                is_odd: false,
            };
        }

        pub fn from1970(&mut self, now_sys_sec: &u64) {
            let now: u64;
            match self.time_zone {
                0..=12 => now = now_sys_sec + self.time_zone * 3600 - 3600 * 24,
                _ => now = now_sys_sec + self.time_zone * 3600,
            }
            self.set_year(&now);
            self.set_mounth(&now, 1);
            self.set_day(&now);
            self.set_hour(&now);
            self.set_min(&now);
            self.set_sec();
            self.year = self.year + 1970;
        }

        pub fn build_from1970(&mut self, now_sys_sec: &u64) -> Self {
            let now: u64;
            match self.time_zone {
                0..=12 => now = now_sys_sec + self.time_zone * 3600 - 3600 * 24,
                _ => now = now_sys_sec + self.time_zone * 3600,
            }
            self.set_year(&now);
            self.set_mounth(&now, 1);
            self.set_day(&now);
            self.set_hour(&now);
            self.set_min(&now);
            self.set_sec();
            self.year = self.year + 1970;
            return self.clone();
        }

        pub fn get_all_as_str(&mut self) -> String {
            return String::from(
                self.year.to_string()
                    + "-"
                    + &self.mounth.to_string()
                    + "-"
                    + &self.day.to_string()
                    + "-"
                    + &self.hour.to_string()
                    + "-"
                    + &self.min.to_string()
                    + "-"
                    + &self.sec.to_string(),
            );
        }

        pub fn test_console_print(&mut self) {
            println!("{}", self.get_all_as_str());
        }

        pub fn set_timezone(&mut self, tin: u64) {
            self.time_zone = tin;
        }

        fn set_sec(&mut self) {
            self.sec = self.offset_day as u64;
            self.offset_day = 0;
        }

        fn set_hour(&mut self, now: &u64) {
            self.hour = (self.offset_day / 3600) as u64;
            self.offset_day = (now % 3600) as i64;
        }

        fn set_min(&mut self, now: &u64) {
            self.min = (self.offset_day / 60) as u64;
            self.offset_day = (now % 60) as i64;
        }

        fn set_day(&mut self, now: &u64) {
            self.day = (now / 86400) - self.offset_day as u64;
            self.offset_day = (now % 86400) as i64;
        }

        fn set_mounth(&mut self, now: &u64, mut begin: u64) {
            match begin {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => self.offset_day = self.offset_day + 31,
                4 | 6 | 9 | 11 => self.offset_day = self.offset_day + 30,
                2 => {
                    if self.is_odd {
                        self.offset_day = self.offset_day + 29
                    } else {
                        self.offset_day = self.offset_day + 28
                    }
                }
                _ => {}
            }
            if self.offset_day > ((now / 86400) as i64) {
                self.mounth = begin;
                match begin {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => self.offset_day = self.offset_day - 31,
                    4 | 6 | 9 | 11 => self.offset_day = self.offset_day - 30,
                    2 => {
                        if self.is_odd {
                            self.offset_day = self.offset_day - 29
                        } else {
                            self.offset_day = self.offset_day - 28
                        }
                    }
                    _ => {}
                }
            } else {
                begin = begin + 1;
                if begin > 12 {
                } else {
                    self.set_mounth(now, begin);
                }
            }
        }

        fn set_year(&mut self, now: &u64) {
            self.year = now / 31_536_000;
            self.offset_day = self.offset_day + (self.year * 365) as i64;
            let _mod_sec = now - self.year * 31_536_000;

            self.check_year(_mod_sec);
        }

        fn check_year(&mut self, mod_sec_in: u64) {
            let mut _sub = 0u64;
            _sub = (self.year - 2) / 4;
            let mod_400 = self.year % 400;
            if mod_400 > 30 {
                _sub = _sub - (self.year / 400 + 1);
            }

            if self.year % 4 == 0 && mod_400 != 30 {
                self.is_odd = true
            } else {
                self.is_odd = false
            }

            if (mod_sec_in as i32 - _sub as i32 * 86_400) < 0 {
                self.year = self.year - 1;
                if self.is_odd {
                    self.offset_day = self.offset_day - 366;
                } else {
                    self.offset_day = self.offset_day - 365;
                }
                _sub = (self.year - 2) / 4;
                let mod_400 = self.year % 400;
                if mod_400 > 30 {
                    _sub = _sub - (self.year / 400 + 1);
                }
            }
            self.offset_day = self.offset_day + _sub as i64;
        }
    }
}
