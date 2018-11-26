// lib.rs - Epoch time clock.
// Copyright (C) 2018  David Anthony Stainton.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

extern crate chrono;

use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{NaiveDate, Utc, DateTime};


#[derive(PartialEq, Debug, Clone, Default)]
pub struct Time {
    pub epoch: u64,
    pub elapsed: u64,
    pub till: u64,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Config {
    pub epoch: u64,
    pub period: u64,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Clock {
    cfg: Config,
}

impl Clock {
    pub fn new(cfg: Config) -> Self {
        Clock {
            cfg: cfg,
        }
    }

    pub fn period(&self) -> u64 {
        self.cfg.period
    }

    pub fn new_katzenpost() -> Self {
        Clock {
            cfg: Config {
                epoch: (DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2017, 6, 1).and_hms(0,0,0), Utc) -
                        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(1970, 1, 1).and_hms(0,0,0), Utc))
                    .num_seconds() as u64,
                period: 3 * 60 * 60,
            },
        }
    }

    pub fn now(&self) -> Time {
        self.get_epoch(SystemTime::now())
    }

    pub fn is_in_epoch(&self, e: u64, t: u64) -> bool {
        let delta_start = e * self.cfg.period;
        let delta_end = (e+1) * self.cfg.period;
        let start_time = self.cfg.epoch.wrapping_add(delta_start);
        let end_time = self.cfg.epoch.wrapping_add(delta_end);
        if t == start_time {
            return true
        }
        if t > start_time && t < end_time {
            return true
        }
        return false
    }

    fn get_epoch(&self, _time: SystemTime) -> Time {
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(x) => x.as_secs(),
            Err(e) => panic!(e),
        };
        let from_epoch = now.wrapping_sub(self.cfg.epoch);
        let current: u64 = from_epoch / self.cfg.period;
        let base = self.cfg.epoch.wrapping_add(current * self.cfg.period);
        let unix_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs() as u64,
            Err(_) => panic!("SystemTime is before the Unix epoch"),
        };
        let elapsed = unix_time.wrapping_sub(base);
        let till = (base + self.cfg.period).wrapping_sub(unix_time);
        Time{
            epoch: current,
            elapsed: elapsed,
            till: till,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use super::Clock;

    #[test]
    fn basic_time_now_test() {
        let e = Clock::new_katzenpost();
        let now = e.now();
        println!("current {} elapsed {} till {}", now.epoch, now.elapsed, now.till);
    }

    #[test]
    fn is_in_epoch_test() {
        let e = Clock::new_katzenpost();
        let now = e.now();
        let unix_epoch_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let now_secs = unix_epoch_now.as_secs();
        assert_eq!(e.is_in_epoch(now.epoch, now_secs), true);
    }
}
