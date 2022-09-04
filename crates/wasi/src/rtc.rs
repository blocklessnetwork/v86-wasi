use std::rc::Weak;

use chrono::{Datelike, TimeZone, Timelike, Utc};
use wasmtime::Store;

use crate::{consts::*, Dev, Emulator, EmulatorTrait, CPU};

pub(crate) struct RTC {
    cmos_index: u8,
    cmos_data: Vec<u8>,
    rtc_time: i64,
    last_update: i64,
    next_interrupt: i64,
    next_interrupt_alarm: usize,
    store: Weak<Store<Emulator>>,
    periodic_interrupt: bool,
    periodic_interrupt_time: f64,
    nmi_disabled: u8,
    cmos_a: u8,
    cmos_b: u8,
    cmos_c: u8,
}

impl RTC {
    #[inline]
    fn cpu_mut(&mut self) -> Option<&mut CPU> {
        self.store.cpu_mut()
    }

    pub fn new(store: Weak<Store<Emulator>>) -> Self {
        let now = Utc::now().timestamp_millis();
        Self {
            store,
            cmos_index: 0,
            cmos_data: vec![0; 128],
            rtc_time: now,
            last_update: now,
            next_interrupt: 0,
            next_interrupt_alarm: 0,
            periodic_interrupt: false,
            periodic_interrupt_time: 1000.0 / 1024.0,
            nmi_disabled: 0,
            cmos_a: 0x26,
            cmos_b: 2,
            cmos_c: 0,
        }
    }

    pub fn init(&mut self) {
        let weak_store = self.store.clone();
        self.cpu_mut().map(|cpu| {
            cpu.io.register_write8(
                0x70,
                crate::Dev::Emulator(weak_store.clone()),
                |dev: &Dev, _: u32, v: u8| {
                    dev.rtc_mut().map(|rtc| {
                        rtc.cmos_index = v & 0x7f;
                        rtc.nmi_disabled = v >> 7;
                    });
                },
            );

            cpu.io.register_write8(
                0x71,
                crate::Dev::Emulator(weak_store.clone()),
                Self::cmos_port_write8,
            );
            cpu.io.register_read8(
                0x71,
                crate::Dev::Emulator(weak_store.clone()),
                Self::cmos_port_read8,
            );
        });
    }

    #[inline(always)]
    fn decode_time(&self, v: u32) -> u32 {
        if self.cmos_b & 4 != 0 {
            v
        } else {
            Self::bcd_unpack(v)
        }
    }

    #[inline(always)]
    fn bcd_unpack(n: u32) -> u32 {
        let low = n & 0xF;
        let high = n >> 4 & 0xF;

        assert!(n < 0x100);
        assert!(low < 10);
        assert!(high < 10);
        low + 10 * high
    }

    #[inline(always)]
    fn encode_time(&self, v: u8) -> u8 {
        if self.cmos_b & 4 != 0 {
            v
        } else {
            Self::bcd_pack(v)
        }
    }

    fn bcd_pack(mut n: u8) -> u8 {
        let mut i = 0;
        let mut result = 0;
        let mut digit = 0;
        while n != 0 {
            digit = n % 10;
            result |= digit << (4 * i);
            i += 1;
            n = (n - digit) / 10;
        }
        result
    }

    #[inline(always)]
    pub(crate) fn cmos_write(self: &mut RTC, index: u8, v: u8) {
        dbg_log!("cmos {:#02X} <- {:#02X}", index, v);
        assert!(index < 128);
        self.cmos_data[index as usize] = v;
    }

    fn cmos_port_read8(dev: &Dev, _port: u32) -> u8 {
        dev.rtc_mut().map_or(0, |rtc| {
            let index = rtc.cmos_index;
            match index {
                CMOS_RTC_SECONDS => {
                    let rtc_time = rtc.rtc_time;
                    rtc.encode_time(Utc.timestamp_millis(rtc_time).second() as _)
                }
                CMOS_RTC_MINUTES => {
                    let rtc_time = rtc.rtc_time;
                    rtc.encode_time(Utc.timestamp_millis(rtc_time).minute() as _)
                }
                CMOS_RTC_HOURS => {
                    let rtc_time = rtc.rtc_time;
                    rtc.encode_time(Utc.timestamp_millis(rtc_time).hour() as _)
                }
                CMOS_RTC_DAY_MONTH => {
                    let rtc_time = rtc.rtc_time;
                    rtc.encode_time(Utc.timestamp_millis(rtc_time).day() as _)
                }
                CMOS_RTC_MONTH => {
                    let rtc_time = rtc.rtc_time;
                    rtc.encode_time(Utc.timestamp_millis(rtc_time).month() as _)
                }
                CMOS_RTC_YEAR => {
                    let rtc_time = rtc.rtc_time;
                    rtc.encode_time((Utc.timestamp_millis(rtc_time).year() % 100) as u8)
                }
                CMOS_STATUS_A => rtc.cmos_a,
                CMOS_STATUS_B => rtc.cmos_b,
                CMOS_STATUS_C => {
                    //TODO: this.cpu.device_lower_irq(8);
                    dbg_log!("cmos reg C read");
                    let c = rtc.cmos_c;
                    let mask: u8 = !0xF0;
                    rtc.cmos_c &= mask;
                    c
                }
                CMOS_STATUS_D => 0xFF,
                CMOS_CENTURY => {
                    let rtc_time = rtc.rtc_time;
                    rtc.encode_time((Utc.timestamp_millis(rtc_time).year() % 100) as u8 | 0u8)
                }
                _ => {
                    let data = rtc.cmos_data[rtc.cmos_index as usize];
                    dbg_log!("cmos read from index {:#02X}: data: {:#02X}", index, data);
                    data
                }
            }
        })
    }

    fn cmos_port_write8(dev: &Dev, _port: u32, v: u8) {
        dev.rtc_mut().map(|rtc| match rtc.cmos_index {
            0xA => {
                rtc.cmos_a = v & 0x7F;
                rtc.periodic_interrupt_time = 1000.0 / (32768 >> (rtc.cmos_a & 0xF) - 1) as f64;
                dbg_log!(
                    "Periodic interrupt, a= 0x{:02}  t={}",
                    rtc.cmos_a,
                    rtc.periodic_interrupt_time
                );
            }
            0xB => {
                rtc.cmos_b = v;
                if rtc.cmos_b & 0x40 != 0 {
                    let now = Utc::now();
                    rtc.next_interrupt = now.timestamp_millis();
                }
                if rtc.cmos_b & 0x20 != 0 {
                    let now = Utc::now();
                    let secs = rtc.cmos_data[CMOS_RTC_SECONDS_ALARM as usize];
                    let minus = rtc.cmos_data[CMOS_RTC_MINUTES_ALARM as usize];
                    let hours = rtc.cmos_data[CMOS_RTC_HOURS_ALARM as usize];
                    let secs: u32 = rtc.decode_time(secs as _);
                    let minus: u32 = rtc.decode_time(minus as _);
                    let hours: u32 = rtc.decode_time(hours as _);
                    let alarm_date = Utc
                        .ymd(now.year(), now.month(), now.day())
                        .and_hms(hours, minus, secs);
                    let ms_from_now = alarm_date.timestamp_millis() - now.timestamp_millis();
                    dbg_log!(
                        "RTC alarm scheduled for {} hh:mm:ss={}:{}:{} ms_from_now={}",
                        alarm_date.to_string(),
                        hours,
                        minus,
                        secs,
                        ms_from_now
                    );
                    rtc.next_interrupt = alarm_date.timestamp_millis();
                }
            }
            CMOS_RTC_SECONDS_ALARM | CMOS_RTC_MINUTES_ALARM | CMOS_RTC_HOURS_ALARM => {
                rtc.cmos_write(rtc.cmos_index as _, v);
            }
            _ => {
                dbg_log!("cmos write index {:#X}: {:#X}", rtc.cmos_index, v);
            }
        });
    }
}
