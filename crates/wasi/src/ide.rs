#![allow(unused)]
use std::{collections::HashSet};

use crate::{StoreT, ContextTrait, Dev, log::LOG, bus::BusData, io::IO, FileBuffer, pci::{GenericPCIDevice, PCIBar}, CMOS_BIOS_DISKTRANSFLAG, CMOS_DISK_DATA, CMOS_DISK_DRIVE1_CYL};

const CDROM_SECTOR_SIZE: u32 = 2048;

const HD_SECTOR_SIZE: u32 = 512;

struct IDEInterface {
    store: StoreT,
    nr: u8,
    head: u8,
    error: u16,
    status: u8,
    is_lba: u8,
    is_cd: bool,
    sector: u16,
    last_id: u32,
    data: Vec<u8>,
    bytecount: u32,
    is_atapi: bool,
    lba_count: u16,
    is_master: bool,
    head_count: u16,
    drive_head: u16,
    data_end: usize,
    sector_size: u32,
    cylinder_low: u16,
    write_dest: usize,
    cylinder_high: u16,
    sector_count: u32,
    last_io_id: usize,
    data_length: usize,
    data_pointer: usize,
    sectors_per_drq: u8,
    current_command: u8,
    cylinder_count: u32,
    sectors_per_track: u16,
    current_atapi_command: u8,
    data16: &'static mut [u16],
    data32: &'static mut [u32],
    buffer: Option<Box<dyn FileBuffer>>,
    cancelled_io_ids: HashSet<usize>,
    in_progress_io_ids: HashSet<usize>,
}

impl IDEInterface {
    fn new(
        store: StoreT, 
        buffer: Option<Box<dyn FileBuffer>>, 
        is_cd: bool, 
        nr: u8, 
        is_master: bool
    ) -> IDEInterface {
        let sector_size = if is_cd {
            CDROM_SECTOR_SIZE
        } else {
            HD_SECTOR_SIZE
        };
        let is_atapi = is_cd;
        let sector_count = 0;
        let mut data = vec![0; 64 * 1024];
        let (data16, data32) = unsafe {
            let d_ptr = data.as_mut_ptr();
            let data16 = std::slice::from_raw_parts_mut(d_ptr as *mut u16, data.len()/2);
            let data32 = std::slice::from_raw_parts_mut(d_ptr as *mut u32, data.len()/4);
            (data16, data32)
        };
        let last_id = 0;
        let cancelled_io_ids = HashSet::new();
        let in_progress_io_ids = HashSet::new();
        let status = 0x50;
        let bytecount = 0;
        let sector = 0;
        let error = 0;
        let head = 0;
        let is_lba = 0;
        let cylinder_low = 0;
        let cylinder_high = 0;
        let data_end = 0;
        let drive_head = 0;
        let data_pointer = 0;
        let data_length = 512;
        let cylinder_count = 0;
        let current_command = 0xFF;
        let current_atapi_command = 0xFF;
        let sectors_per_track = 0;
        let head_count = 0;
        let lba_count = 0;
        let last_io_id = 0;
        let write_dest = 0;
        let sectors_per_drq = 0x80;
        IDEInterface { 
            nr,
            data,
            head,
            is_cd,
            store,
            status,
            error,
            data16,
            buffer,
            sector,
            data32,
            is_lba,
            last_id,
            data_end,
            is_atapi,
            bytecount,
            is_master,
            lba_count,
            head_count,
            drive_head,
            last_io_id,
            write_dest,
            data_length,
            sector_size,
            sector_count,
            data_pointer,
            cylinder_low,
            cylinder_high,
            cylinder_count,
            current_command,
            sectors_per_drq,
            cancelled_io_ids,
            sectors_per_track,
            in_progress_io_ids,
            current_atapi_command,
        }
    }

    fn init(&mut self) {
        if self.buffer.is_some() {
            let byte_length = self.buffer.as_ref().map(|b| b.byte_length()).unwrap();
            let sector_count = byte_length as f64 / self.sector_size as f64;

            self.sector_count = if self.sector_count != (self.sector_count | 0) {
                dbg_log!(LOG::DISK, "Warning: Disk size not aligned with sector size");
                sector_count.ceil() as u32
            } else {
                sector_count as u32
            };

            if self.is_cd {
                self.head_count = 1;
                self.sectors_per_track = 0;
            } else {
                // "default" values: 16/63
                // common: 255, 63
                self.head_count = 16;
                self.sectors_per_track = 63;
            }


            let cylinder_count = self.sector_count as f64 / self.head_count as f64 / self.sectors_per_track as f64;
            

            self.cylinder_count = if self.cylinder_count != (self.cylinder_count | 0) {
                dbg_log!(LOG::DISK, "Warning: Rounding up cylinder count. Choose different head number");
                cylinder_count.floor() as u32
            } else  {
                cylinder_count as u32
            };
            self.store.rtc_mut().map(|rtc| {
                rtc.cmos_write(CMOS_BIOS_DISKTRANSFLAG,
                    rtc.cmos_read(CMOS_BIOS_DISKTRANSFLAG) | 1 << self.nr * 4); 

                rtc.cmos_write(CMOS_DISK_DATA, rtc.cmos_read(CMOS_DISK_DATA) & 0x0F | 0xF0);
                let reg = CMOS_DISK_DRIVE1_CYL;
                rtc.cmos_write(reg + 0, (self.cylinder_count & 0xFF) as u8);
                rtc.cmos_write(reg + 1, (self.cylinder_count >> 8 & 0xFF) as u8);
                rtc.cmos_write(reg + 2, (self.head_count & 0xFF) as u8);
                rtc.cmos_write(reg + 3, 0xFF);
                rtc.cmos_write(reg + 4, 0xFF);
                rtc.cmos_write(reg + 5, 0xC8);

                rtc.cmos_write(reg + 6, (self.cylinder_count & 0xFF) as u8);
                rtc.cmos_write(reg + 7, (self.cylinder_count >> 8 & 0xFF) as u8);
                rtc.cmos_write(reg + 8, (self.sectors_per_track & 0xFF) as u8);
            });

        }
    }

    fn read_data(&mut self, length: u8) -> u32 {
        let length = length as usize;
        if self.data_pointer < self.data_end {
            assert!(self.data_pointer + length - 1 < self.data_end);
            assert!(self.data_pointer % length == 0, "{:#X} {}", self.data_pointer, length);
    
            let result: u32 = if length == 1 {
                self.data[self.data_pointer] as u32
            } else if length == 2 {
                self.data16[self.data_pointer >> 1]  as u32
            } else {
                self.data32[self.data_pointer >> 2] as u32
            };
    
            self.data_pointer += length;
    
            let align = if (self.data_end & 0xFFF) == 0 { 
                0xFFF 
            } else { 
                0xFF 
            };
            if (self.data_pointer & align) == 0 {
                let curr_data = if self.data_pointer < self.data.len() {
                    self.data[self.data_pointer]
                } else {
                    0
                };
                dbg_log!(
                    LOG::DISK,
                    "Read 1F0: {:#X} cur={:#X} cnt={:#X}", 
                    curr_data,
                    self.data_pointer,
                    self.data_length,
                );
            }
    
            if self.data_pointer >= self.data_end {
                self.read_end();
            }
            return result;
        } else {
            dbg_log!(LOG::DISK, "Read 1F0: empty");
            self.data_pointer += length;
            return 0;
        }
    }

    #[inline]
    fn write_data_port8(&mut self, data: u8) {
        self.write_data_port(data as u32, 1);
    }

    #[inline]
    fn write_data_port16(&mut self, data: u16) {
        self.write_data_port(data as u32, 2);
    }

    #[inline]
    fn write_data_port32(&mut self, data: u32) {
        self.write_data_port(data, 4);
    }

    fn write_data_port(&mut self, data: u32, length: u8) {
        assert!(self.data_pointer % (length as usize) == 0);

        if self.data_pointer >= self.data_end {
            dbg_log!(LOG::DISK, 
                "Redundant write to data port: {:#X} count={:#X} cur={:#X}",
                data,
                self.data_end,
                self.data_pointer
            );
        } else {
            let align = if (self.data_end & 0xFFF) == 0 { 
                0xFFF 
            } else { 
                0xFF
            };
            if (self.data_pointer + (length as usize) & align) == 0 || self.data_end < 20 {
                dbg_log!(
                    LOG::DISK, 
                    "Data port: {:#X} count={:#X} cur={:#X}",
                    data,
                    self.data_end,
                    self.data_pointer);
            }

            if length == 1 {
                self.data[self.data_pointer] = data as u8;
                self.data_pointer += 1;
            } else if length == 2 {
                self.data16[self.data_pointer >> 1] = data as u16;
                self.data_pointer += 2;
            } else {
                self.data32[self.data_pointer >> 2] = data;
                self.data_pointer += 4;
            }

            assert!(self.data_pointer <= self.data_end);
            if self.data_pointer == self.data_end {
                self.write_end();
            }
        }
    }

    fn do_write(&mut self) {
        self.status = 0x50;
    
        assert!(self.data_length <= self.data.len());
        //dbg_log(hex_dump(data), LOG_DISK);
        assert!(self.data_length % 512 == 0);
        self.ata_advance(self.current_command, (self.data_length / 512) as u32);
        self.push_irq();

        self.buffer.as_mut().map(|b| {
            let data = &self.data[0..self.data_length];
            b.set(self.write_dest, data, Box::new(|store| {}));
        });
        self.report_write(self.data_length);
    }

    fn report_write(&self, byte_count: usize) {
        let sector_count = byte_count / (self.sector_size as usize) | 0;
        // this.stats.sectors_written += sector_count;
        // this.stats.bytes_written += byte_count;
        self.store.bus_mut().map(|bus| {
            bus.send("ide-write-end", BusData::IdeWriteEnd(self.nr, byte_count, sector_count));
        });
    }

    fn write_end(&mut self) {
        if self.current_command == 0xA0 {
            self.atapi_handle();
        } else {
            dbg_log!(
                LOG::DISK, 
                "write_end data_pointer={:#X}  data_length={:#X}",
                self.data_pointer,
                self.data_length
            );

            if self.data_pointer >= self.data_length {
                self.do_write();
            } else {
                assert!(self.current_command == 0x30 ||
                    self.current_command == 0x34 ||
                    self.current_command == 0xC5,
                    "Unexpected command: {:#X}", 
                    self.current_command
                );

                // XXX: Should advance here, but do_write does all the advancing
                //this.ata_advance(this.current_command, 1);
                self.status = 0x58;
                self.data_end += 512;
                self.push_irq();
            }
        }
    }

    #[inline]
    fn data_set(&mut self, data: &[u8]) {
        self.data_allocate_noclear(data.len());
        self.data[0..data.len()].copy_from_slice(data);
    }

    #[inline]
    fn data_allocate_noclear(&mut self, len: usize) {
        if self.data.len() < len {
            self.data = vec![0; len + 3 & !3];
            (self.data16, self.data32) = unsafe {
                let p = self.data.as_mut_ptr();
                let data16 = std::slice::from_raw_parts_mut(p as *mut u16, self.data.len()/2);
                let data32 = std::slice::from_raw_parts_mut(p as *mut u32, self.data.len()/4);
                (data16, data32)
            };
        }
        self.data_length = len;
        self.data_pointer = 0;
    }

    #[inline]
    fn data_allocate(&mut self, len: usize) {
        self.data_allocate_noclear(len);
        for i in 0..(len + 3 >> 2) {
            self.data32[i] = 0;
        }
    }

    fn atapi_read_dma(&mut self) {
        let cmd: &[u8] = &self.data;
        // Note: Big Endian
        let lba = (cmd[2] as u32) << 24 | (cmd[3] as u32) << 16 | (cmd[4] as u32) << 8 | cmd[5] as u32;
        let count = (cmd[7] as u16) << 8 | cmd[8] as u16;
        let flags = cmd[1];
        let byte_count = (count as u32) * self.sector_size;
        let start = lba * self.sector_size;

        dbg_log!(
            LOG::DISK,
            "CD read DMA lba={:#X} lbacount={:#X} bytecount={:#X} flags={:#X}",
            lba, count, byte_count, flags
        );

        let buffer = self.buffer.as_ref().unwrap();
        if start as usize >= buffer.byte_length()  {
            assert!(
                false, 
                "CD read: Outside of disk  end={:#X} size={:#X}", 
                (start + byte_count), 
                buffer.byte_length()
            );
            self.status = 0xFF;
            self.push_irq();
        } else {
            self.status = 0x50 | 0x80;
            self.report_read_start();
            let is_master = self.is_master;
            self.read_buffer(start as usize, byte_count as usize, Box::new(move |store, bs| {
                dbg_log!(LOG::DISK, "atapi_read_dma: Data arrived");
                store.ide_mut().map(|ide| {
                    let ide_i = ide.interface_mut(is_master);
                    ide_i.report_read_end(bs.len());
                    ide_i.status = 0x58;
                    ide_i.bytecount = ide_i.bytecount & !7 | 2;
                    ide_i.data_set(bs);
                    ide_i.do_atapi_dma(); 
                });
            }));
        }
    }

    fn read_buffer(&mut self, start: usize, length: usize, callb: Box<dyn Fn(&StoreT, &[u8])>) {
        let id = self.last_io_id;
        self.last_io_id += 1;
        self.in_progress_io_ids.insert(id);
        self.buffer.as_mut().map(|buffer| {
            let is_master = self.is_master;
            buffer.get(start, length, Box::new(move |store, bs| {
                store.ide_mut().map(|ide| {
                    let ide_i = ide.interface_mut(is_master);
                    if ide_i.cancelled_io_ids.remove(&id) {
                        assert!(!ide_i.in_progress_io_ids.contains(&id));
                        return;
                    }
                    let removed = ide_i.in_progress_io_ids.remove(&id);
                    assert!(removed);
                    callb(store, bs);
                });
            }));
        });
    }

    fn create_identify_packet(&mut self) {
        if self.drive_head & 0x10 > 0 {
            // slave
            self.data_allocate(0);
            return;
        }

        for i in 0..512 {
            self.data[i] = 0;
        }
        let cylinder_count = if self.cylinder_count > 0 {
            16383.min(self.cylinder_count)
        } else {
            16383
        };
        
        let apapi = if self.is_atapi { 
            0x85 
        } else {
            0
        }; 
        self.data_set(&[
            0x40, apapi, 
            cylinder_count as u8, (cylinder_count >> 8) as u8,
            0, 0,
            (self.head_count) as u8, (self.head_count >> 8) as u8,
            (self.sectors_per_track / 512) as u8, (self.sectors_per_track / 512 >> 8) as u8,
            0, (512 >> 8) as u8,
            // sectors per track
            (self.sectors_per_track) as u8, (self.sectors_per_track >> 8) as u8,
            0, 0, 0, 0, 0, 0,
            // 10-19 serial number
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // 15
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // 20
            3, 0,
            0, 2,
            4, 0,
            // 23-26 firmware revision
            0, 0, 0, 0, 0, 0, 0, 0,

            // 27 model number
            56, 118, 32, 54, 68, 72, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
            32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,

            // 47 max value for set multiple mode
            0x80, 0,
            1, 0,
            //0, 3,  // capabilities, 2: Only LBA / 3: LBA and DMA
            0, 2,  // capabilities, 2: Only LBA / 3: LBA and DMA
            // 50
            0, 0,
            0, 2,
            0, 2,
            7, 0,
            // 54 cylinders
            cylinder_count as u8, (cylinder_count >> 8) as u8,
            // 55 heads
            self.head_count as u8, (self.head_count >> 8) as u8,
            // 56 sectors per track
            self.sectors_per_track as u8, 0,
            // capacity in sectors
            (self.sector_count & 0xFF) as u8, (self.sector_count >> 8 & 0xFF) as u8,
            (self.sector_count >> 16 & 0xFF) as u8, (self.sector_count >> 24 & 0xFF) as u8,

            0, 0,
            // 60
            (self.sector_count & 0xFF) as u8, (self.sector_count >> 8 & 0xFF) as u8,
            (self.sector_count >> 16 & 0xFF) as u8, (self.sector_count >> 24 & 0xFF) as u8,

            0, 0,
            // 63, dma supported mode, dma selected mode
            if self.current_command == 0xA0 { 0 } else {7}, if self.current_command == 0xA0 { 0 } else { 4 },
            //0, 0, // no DMA

            0, 0,
            // 65
            30, 0, 30, 0, 30, 0, 30, 0, 0, 0,
            // 70
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // 75
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // 80
            0x7E, 0, 0, 0, 0, 0, 0, 0x74, 0, 0x40,
            // 85
            0, 0x40, 0, 0x74, 0, 0x40, 0, 0, 0, 0,
            // 90
            0, 0, 0, 0, 0, 0, 1, 0x60, 0, 0,
            // 95
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // 100
            (self.sector_count & 0xFF) as u8, (self.sector_count >> 8 & 0xFF) as u8,
            (self.sector_count >> 16 & 0xFF) as u8, (self.sector_count >> 24 & 0xFF) as u8,    
        ]);
        self.data_length = 512;
        self.data_end = 512;
    }

    fn ata_command(&mut self, cmd: u8) {
        dbg_log!(LOG::DISK, "ATA Command: {:#X} slave={}", cmd, self.drive_head >> 4 & 1);

        if self.buffer.is_none() {
            dbg_log!(LOG::DISK, "abort: No buffer");
            self.error = 4;
            self.status = 0x41;
            self.push_irq();
            return;
        }

        self.current_command = cmd;
        self.error = 0;

        match cmd {
            0x08 => {
                dbg_log!(LOG::DISK, "ATA device reset");
                self.data_pointer = 0;
                self.data_end = 0;
                self.data_length = 0;
                self.device_reset();
                self.push_irq();
            }

            0x10 => {
                // calibrate drive
                self.status = 0x50;
                self.cylinder_low = 0;
                self.push_irq();
            }

            0xF8 => {
                // read native max address
                self.status = 0x50;
                let last_sector = self.sector_count - 1;
                self.sector = (last_sector & 0xFF) as u16;
                self.cylinder_low = (last_sector >> 8 & 0xFF) as u16;
                self.cylinder_high = (last_sector >> 16 & 0xFF) as u16;
                self.drive_head = self.drive_head & 0xF0 | (last_sector >> 24 & 0x0F) as u16;
                self.push_irq();
            }

            0x27 => {
                // read native max address ext
                self.status = 0x50;
                let last_sector = self.sector_count - 1;
                self.sector = (last_sector & 0xFF) as u16;
                self.cylinder_low = (last_sector >> 8 & 0xFF) as u16;
                self.cylinder_high = (last_sector >> 16 & 0xFF) as u16;
                self.sector |= (last_sector >> 24 << 8 & 0xFF00) as u16;
                self.push_irq();
            }

            // 0x20 read sectors
            // 0x24 read sectors ext
            // 0xC4 read multiple
            // 0x29 read multiple ext
            0x20|0x24|0x29|0xC4 => self.ata_read_sectors(cmd),

            // 0x30 write sectors
            // 0x34 write sectors ext
            // 0xC5 write multiple
            // 0x39 write multiple ext
            0x30|0x34|0x39|0xC5 => self.ata_write_sectors(cmd),

            0x90 => {
                // execute device diagnostic
                self.push_irq();
                self.error = 0x101;
                self.status = 0x50;
            }
            0x91 => {
                // initialize device parameters
                self.status = 0x50;
                self.push_irq();
            }

            0xA0 => {
                // ATA packet
                if self.is_atapi {
                    self.status = 0x58;
                    self.data_allocate(12);
                    self.data_end = 12;
                    self.bytecount = 1;
                    self.push_irq();
                }
            }

            0xA1 => {
                dbg_log!(LOG::DISK, "ATA identify packet device");

                if self.is_atapi {
                    self.create_identify_packet();
                    self.status = 0x58;

                    self.cylinder_low = 0x14;
                    self.cylinder_high = 0xEB;

                    self.push_irq();
                } else {
                    self.status = 0x41;
                    self.push_irq();
                }
            }

            0xC6 => {
                // set multiple mode
                // Logical sectors per DRQ Block in word 1
                dbg_log!(
                    LOG::DISK, 
                    "Logical sectors per DRQ Block: {:#X}",
                    self.bytecount & 0xFF
                );
                self.sectors_per_drq = (self.bytecount & 0xFF) as u8;
                self.status = 0x50;
                self.push_irq();
            }
            // read dma ext
            // read dma
            0x25|0xC8 => self.ata_read_sectors_dma(cmd),
            // write dma ext
            // write dma
            0x35|0xCA => self.ata_write_sectors_dma(cmd),
            0x40 => {
                dbg_log!(LOG::DISK, "read verify sectors");
                self.status = 0x50;
                self.push_irq();
            }
            0xDA => {
                dbg_log!(LOG::DISK, "Unimplemented: get media status");
                self.status = 0x41;
                self.error = 4;
                self.push_irq();
            }

            0xE0 => {
                dbg_log!(LOG::DISK, "ATA standby immediate");
                self.status = 0x50;
                self.push_irq();
            }

            0xE1 => {
                dbg_log!(LOG::DISK, "ATA idle immediate");
                self.status = 0x50;
                self.push_irq();
            }

            0xE7 => {
                dbg_log!(LOG::DISK, "ATA flush cache");
                self.status = 0x50;
                self.push_irq();
            }

            0xEC => {
                dbg_log!(LOG::DISK, "ATA identify device");

                if self.is_atapi {
                    self.status = 0x41;
                    self.error = 4;
                    self.push_irq();
                    return;
                }

                self.create_identify_packet();
                self.status = 0x58;

                self.push_irq();
            }

            0xEA => {
                dbg_log!(LOG::DISK, "flush cache ext");
                self.status = 0x50;
                self.push_irq();
            }

            0xEF => {
                dbg_log!(LOG::DISK, "set features: {:#X}", self.bytecount & 0xFF);
                self.status = 0x50;
                self.push_irq();
            }

            0xDE => {
                // obsolete
                self.status = 0x50;
                self.push_irq();
            }

            0xF5 => {
                dbg_log!(LOG::DISK, "security freeze lock");
                self.status = 0x50;
                self.push_irq();
            }

            0xF9 => {
                dbg_log!(LOG::DISK, "Unimplemented: set max address");
                self.status = 0x41;
                self.error = 4;
            }
            
            _ => {
                assert!(false, "New ATA cmd on 1F7: {:#X}", cmd);

                self.status = 0x41;
                // abort bit set
                self.error = 4;
            }
        }
    }

    fn ata_write_sectors_dma(&mut self, cmd: u8) {
        let is_lba48 = cmd == 0x35;
        let count = self.get_count(is_lba48);
        let lba = self.get_lba(is_lba48);

        let byte_count = count * self.sector_size;
        let start = lba * self.sector_size;

        dbg_log!(
            LOG::DISK, 
            "ATA DMA write lba={:#X} lbacount={:#X} bytecount={:#X}",
            lba, count, byte_count
        );
        let byte_length =  self.buffer.as_ref().map(|b| b.byte_length()).unwrap();
        if (start + byte_count) as usize > byte_length {
            assert!(false, "ATA DMA write: Outside of disk");

            self.status = 0xFF;
            self.push_irq();
            return;
        }

        self.status = 0x58;
        self.store.ide_mut().map(|ide| ide.dma_status |= 1);
    }

    fn ata_read_sectors_dma(&mut self, cmd: u8) {
        let is_lba48 = cmd == 0x25;
        let count = self.get_count(is_lba48);
        let lba = self.get_lba(is_lba48);

        let byte_count = (count * self.sector_size) as usize;
        let start = (lba * self.sector_size) as usize;

        dbg_log!(LOG::DISK, 
            "ATA DMA read lba={:#X} lbacount={:#X} bytecount={:#X}", 
            lba, count, byte_count
        );
        let byte_length = self.buffer.as_ref().map(|b| b.byte_length()).unwrap();
        if start + byte_count > byte_length {
            assert!(false, "ATA read: Outside of disk");

            self.status = 0xFF;
            self.push_irq();
            return;
        }

        self.status = 0x58;
        self.store.ide_mut().map(|ide| ide.dma_status |= 1);
    }

    fn ata_write_sectors(&mut self, cmd: u8) {
        let is_lba48 = cmd == 0x34 || cmd == 0x39;
        let count = self.get_count(is_lba48);
        let lba = self.get_lba(is_lba48);

        let is_single = cmd == 0x30 || cmd == 0x34;

        let byte_count = (count * self.sector_size) as usize;
        let start = (lba * self.sector_size) as usize;

        let mode = if self.is_lba > 0 {
            "lba"
        } else {
            "chs"
        };
        dbg_log!(
            LOG::DISK, 
            "ATA write lba={:#X} mode={} lbacount={:#X} bytecount={:#X}",
            lba, mode, count, byte_count
        );
        let byte_length = self.buffer.as_ref().map(|b| b.byte_length()).unwrap();
        if start + byte_count > byte_length {
            assert!(false, "ATA write: Outside of disk");

            self.status = 0xFF;
            self.push_irq();
        } else {
            self.status = 0x58;
            self.data_allocate_noclear(byte_count);
            self.data_end = if is_single {
                512
            } else {
                byte_count.min((self.sectors_per_drq as usize) * 512)
            };
            self.write_dest = start;
        }
    }

    fn ata_read_sectors(&mut self, cmd: u8) {
        let is_lba48 = cmd == 0x24 || cmd == 0x29;
        let count = self.get_count(is_lba48);
        let lba = self.get_lba(is_lba48);

        let is_single = cmd == 0x20 || cmd == 0x24;

        let byte_count = (count * self.sector_size) as usize;
        let start = (lba * self.sector_size) as usize;

        let mode = if self.is_lba > 0 {
            "lba"
        }  else { 
            "chs"
        };
        dbg_log!(
            LOG::DISK, 
            "ATA read cmd={:#X} mode={} lba={:#X} lbacount={:#X} bytecount={:#X}",
            cmd, mode, lba, count, byte_count
        );
        let byte_len = self.buffer.as_ref().map(|buffer| buffer.byte_length()).unwrap();
        if start + byte_count > byte_len {
            assert!(false, "ATA read: Outside of disk");

            self.status = 0xFF;
            self.push_irq();
        } else {
            self.status = 0x80 | 0x40;
            self.report_read_start();
            let is_master = self.is_master;
            self.read_buffer(start, byte_count, Box::new(move |store, data| {
                store.ide_mut().map(|ide| {
                    dbg_log!(LOG::DISK, "ata_read: Data arrived");
                    let ide_i = ide.interface_mut(is_master);
                    ide_i.data_set(data);
                    ide_i.status = 0x58;
                    ide_i.data_end = if is_single {
                        512
                    } else {
                        byte_count.min((ide_i.sectors_per_drq as usize) * 512)
                    };
                    let ata_p = if is_single {
                        1
                    }  else {
                        count.min(ide_i.sectors_per_track as u32) 
                    };
                    ide_i.ata_advance(cmd, ata_p);
                    ide_i.push_irq();
                    ide_i.report_read_end(byte_count);
                });
            }));
        }
    }

    fn atapi_handle(&mut self) {
        dbg_log!(
            LOG::DISK,
            "ATAPI Command: {:#X} slave={}", 
            self.data[0],
            self.drive_head >> 4 & 1
        );
    
        self.data_pointer = 0;
        self.current_atapi_command = self.data[0];
    
        match self.current_atapi_command {
            0x00 => {
                dbg_log!(LOG::DISK, "test unit ready");
                // test unit ready
                self.data_allocate(0);
                self.data_end = self.data_length;
                self.status = 0x50;
            }
            0x03 => {
                // request sense
                self.data_allocate(self.data[4] as usize);
                self.data_end = self.data_length;
                self.status = 0x58;
    
                self.data[0] = 0x80 | 0x70;
                self.data[2] = 5; // illegal request
                self.data[7] = 8;
            }
    
            0x12 => {
                // inquiry
                let length = self.data[4];
                self.status = 0x58;
    
                dbg_log!(LOG::DISK, "inquiry: {:#X} length={}", self.data[1], length);
    
                // http://www.t10.org/ftp/x3t9.2/document.87/87-106r0.txt
                //this.data_allocate(36);
                self.data.copy_from_slice(&[
                    0x05, 0x80, 0x01, 0x31,
                    // additional length
                    31,
                    0, 0, 0,
    
                    // 8
                    0x53, 0x4F, 0x4E, 0x59,
                    0x20, 0x20, 0x20, 0x20,
    
                    // 16
                    0x43, 0x44, 0x2D, 0x52,
                    0x4F, 0x4D, 0x20, 0x43,
                    0x44, 0x55, 0x2D, 0x31,
                    0x30, 0x30, 0x30, 0x20,
    
                    // 32
                    0x31, 0x2E, 0x31, 0x61,
                ]);
                self.data_end = 36.min(length as usize);
                self.data_length = self.data_end;
            }
    
            0x1A => {
                // mode sense (6)
                self.data_allocate(self.data[4] as usize);
                self.data_end = self.data_length;
                self.status = 0x58;
            }
            0x1E => {
                // prevent/allow medium removal
                self.data_allocate(0);
                self.data_end = self.data_length;
                self.status = 0x50;
            }
    
            0x25 => {
                // read capacity
                let count = self.sector_count - 1;
                self.data_set(&[
                    (count >> 24 & 0xFF) as u8,
                    (count >> 16 & 0xFF) as u8,
                    (count >> 8 & 0xFF) as u8,
                    (count & 0xFF) as u8,
                    0,
                    0,
                    (self.sector_size >> 8 & 0xFF) as u8,
                    (self.sector_size & 0xFF) as u8,
                ]);
                self.data_end = self.data_length;
                self.status = 0x58;
            }
            0x28 => {
                // read
                if self.lba_count & 1 > 0 {
                    self.atapi_read_dma();
                } else {
                    self.atapi_read();
                }
            }
    
            0x42 => {
                let length = self.data[8] as usize;
                self.data_allocate(8.min(length));
                self.data_end = self.data_length;
                dbg_log!(LOG::DISK, "read q subcode: length={}", length);
                self.status = 0x58;
            }
    
            0x43 => {
                // read toc
                let length = (self.data[8] as usize) | (self.data[7] as usize) << 8;
                let format = self.data[9] >> 6;
    
                self.data_allocate(length);
                self.data_end = self.data_length;
                dbg_log!(
                    LOG::DISK, 
                    "read toc: {:#X} length={} {} {:#X}",
                    format, length, self.data[1] & 2, self.data[6]
                );
    
                if format == 0 {
                    let sector_count = self.sector_count;
                    self.data.copy_from_slice(&[
                        0, 18, // length
                        1, 1, // first and last session
    
                        0,
                        0x14,
                        1, // track number
                        0,
                        0, 0, 0, 0,
    
                        0,
                        0x16,
                        0xAA, // track number
                        0,
                        (sector_count >> 24) as u8,
                        (sector_count >> 16 & 0xFF) as u8,
                        (sector_count >> 8 & 0xFF) as u8,
                        (sector_count & 0xFF) as u8,
                    ]);
                } else if format == 1 {
                    self.data.copy_from_slice(&[
                        0, 10, // length
                        1, 1, // first and last session
                        0, 0,
                        0, 0,
                        0, 0,
                        0, 0,
                    ]);
                } else {
                    assert!(false, "Unimplemented format: {}", format);
                }
    
                self.status = 0x58;
            }
    
            0x46 => {
                // get configuration
                let mut length = (self.data[8] as usize)  | (self.data[7] as usize) << 8;
                length = length.min(32);
                self.data_allocate(length);
                self.data_end = self.data_length;
                self.data[0] = (length - 4 >> 24 & 0xFF) as u8;
                self.data[1] = (length - 4 >> 16 & 0xFF) as u8;
                self.data[2] = (length - 4 >> 8 & 0xFF) as u8;
                self.data[3] = (length - 4 & 0xFF) as u8;
                self.data[6] = 0x08;
                self.data[10] = 3;
                self.status = 0x58;
            }
            0x51 => {
                // read disk information
                self.data_allocate(0);
                self.data_end = self.data_length;
                self.status = 0x50;
            }
            0x52 => {
                dbg_log!(
                    LOG::DISK, 
                    "Unimplemented ATAPI command: {:#X}",
                    self.data[0]
                );
                self.status = 0x51;
                self.data_length = 0;
                self.error = 5 << 4;
            }
            0x5A => {
                // mode sense
                let length = (self.data[8] as usize) | (self.data[7] as usize) << 8;
                let page_code = self.data[2];
                dbg_log!(
                    LOG::DISK,
                    "mode sense: {:#X} length={}",
                    page_code,
                    length
                );
                if page_code == 0x2A {
                    self.data_allocate(30.min(length));
                }
                self.data_end = self.data_length;
                self.status = 0x58;
            }
            0xBD => {
                // mechanism status
                self.data_allocate((self.data[9] as usize) | (self.data[8] as usize) << 8);
                self.data_end = self.data_length;
                self.data[5] = 1;
                self.status = 0x58;
            }
            0x4A => {
                self.status = 0x51;
                self.data_length = 0;
                self.error = 5 << 4;
                dbg_log!(
                    LOG::DISK,
                    "Unimplemented ATAPI command: {:#X}",
                    self.data[0]
                );
            }
            0xBE => {
                // Hiren's boot CD
                dbg_log!(
                    LOG::DISK, 
                    "Unimplemented ATAPI command: {:#X}",
                    self.data[0]
                );
                self.data_allocate(0);
                self.data_end = self.data_length;
                self.status = 0x50;
            }
            _ => {
                self.status = 0x51;
                self.data_length = 0;
                self.error = 5 << 4;
                dbg_log!(
                    LOG::DISK,
                    "Unimplemented ATAPI command: {:#X}",
                    self.data[0]
                );
                assert!(false);
            }
        }
    
        self.bytecount = self.bytecount & !7 | 2;
    
        if (self.status & 0x80) == 0 {
            self.push_irq();
        }
    
        if (self.status & 0x80) == 0 && self.data_length == 0 {
            self.bytecount |= 1;
            self.status &= !8;
        }
    }

    #[inline]
    fn report_read_end(&self, byte_count: usize) {
        let sector_count = byte_count / (self.sector_size as usize) | 0;
        self.store.bus_mut().map(|bus| {
            bus.send("ide-read-end", BusData::IdeReadEnd(self.nr, byte_count, sector_count));
        });
    }

    #[inline]
    fn report_read_start(&self) {
        self.store.bus_mut().map(|bus| {
            bus.send("ide-read-start", BusData::None);
        });
    }

    fn atapi_read(&mut self) {
        let cmd: &[u8] = &self.data;
        // Note: Big Endian
        let lba = (cmd[2] as u32) << 24 | (cmd[3] as u32) << 16 | (cmd[4] as u32) << 8 | cmd[5] as u32;
        let count = (cmd[7] as usize) << 8 | cmd[8] as usize;
        let flags = cmd[1];
        let mut byte_count = count * (self.sector_size as usize);
        let start = lba * self.sector_size;

        dbg_log!(
            LOG::DISK,
            "CD read lba={:#X}  lbacount={:#X}  bytecount={:#X} flags={:#X}",
            lba, count, byte_count, flags
        );

        self.data_length = 0;
        let mut req_length = (self.cylinder_high as usize) << 8 & 0xFF00 | (self.cylinder_low as usize) & 0xFF;
        dbg_log!(LOG::DISK, "{:#X} {:#X}", self.cylinder_high, self.cylinder_low);
        self.cylinder_low = 0;
        self.cylinder_high = 0; // oak technology driver (windows 3.0)

        if req_length == 0xFFFF {
            req_length -= 1;
        }

        if req_length > byte_count {
            req_length = byte_count;
        }
        let buffer = self.buffer.as_mut().unwrap();
        if start as usize >= buffer.byte_length() {
            assert!(
                false, 
                "CD read: Outside of disk  end={:#X} size={:#X}", 
                start + byte_count as u32, buffer.byte_length()
            );

            self.status = 0xFF;
            self.push_irq();
        } else if byte_count == 0 {
            self.status = 0x50;

            self.data_pointer = 0;
        } else {
            byte_count = byte_count.min(buffer.byte_length() - start as usize);
            self.status = 0x50 | 0x80;
            self.report_read_start();
            let is_master = self.is_master;
            self.read_buffer(start as usize, byte_count, Box::new(move |store, data| {
                dbg_log!(LOG::DISK, "cd read: data arrived");
                store.ide_mut().map(|ide| {
                    let mut req_length = req_length as usize;
                    let ide_i = ide.interface_mut(is_master);
                    ide_i.data_set(data);
                    ide_i.status = 0x58;
                    ide_i.bytecount = ide_i.bytecount & !7 | 2;

                    ide_i.push_irq();

                    req_length &= !3;

                    ide_i.data_end = req_length;
                    if ide_i.data_end > ide_i.data_length {
                        ide_i.data_end = ide_i.data_length;
                    }
                    ide_i.cylinder_low = (ide_i.data_end & 0xFF) as u16;
                    ide_i.cylinder_high = (ide_i.data_end >> 8 & 0xFF) as u16;

                    ide_i.report_read_end(byte_count);
                });
                
            }));
        }
    }

    fn read_end(&mut self) {
        dbg_log!(
            LOG::DISK,
            "read_end cmd={:#X} data_pointer={:#X} end={:#X} length={:#X}",
            self.current_command,
            self.data_pointer,
            self.data_end,
            self.data_length,
        );

        if self.current_command == 0xA0 {
            if self.data_end == self.data_length {
                self.status = 0x50;
                self.bytecount = self.bytecount & !7 | 3;
                self.push_irq();
            } else {
                self.status = 0x58;
                self.bytecount = self.bytecount & !7 | 2;
                self.push_irq();
                let byte_count = (self.cylinder_high as usize) << 8 & 0xFF00 | (self.cylinder_low as usize) & 0xFF;

                if self.data_end + byte_count > self.data_length {
                    self.cylinder_low = ((self.data_length - self.data_end) & 0xFF) as u16;
                    self.cylinder_high = ((self.data_length - self.data_end) >> 8 & 0xFF) as u16;
                    self.data_end = self.data_length;
                } else {
                    self.data_end += byte_count;
                }
                dbg_log!(LOG::DISK, "data_end={:#X}", self.data_end);
            }
        } else {
            self.error = 0;
            if self.data_pointer >= self.data_length {
                self.status = 0x50;
                self.push_irq();
            } else {
                let sector_count = if self.current_command == 0xC4 || self.current_command == 0x29 {
                    let sector_count = (self.sectors_per_drq as usize).min(
                        (self.data_length - self.data_end) / 512);
                    assert!(sector_count % 1 == 0);
                    sector_count
                } else {
                    assert!(self.current_command == 0x20 || self.current_command == 0x24);
                    let sector_count = 1;
                    sector_count
                };
                self.ata_advance(self.current_command, sector_count as u32);
                self.data_end += 512 * sector_count;
                self.status = 0x58;
                self.push_irq();
            }
        }
    }

    #[inline]
    fn get_lba48(&self) -> u32 {
        // Note: Bits over 32 missing
        (self.sector as u32) & 0xFF |
                (self.cylinder_low as u32) << 8 & 0xFF00 |
                (self.cylinder_high as u32) << 16 & 0xFF0000 |
                ((self.sector as u32) >> 8) << 24 & 0xFF000000
    }

    #[inline]
    fn get_lba28(&self) -> u32{
        (self.sector as u32) & 0xFF |
            (self.cylinder_low as u32) << 8 & 0xFF00 |
            (self.cylinder_high as u32) << 16 & 0xFF0000 |
            ((self.head & 0xF) as u32) << 24
    }

    #[inline]
    fn get_chs(&self) -> u32 {
        let c = (self.cylinder_low as u32)  & 0xFF | (self.cylinder_high as u32) << 8 & 0xFF00;
        let h = self.head as u32;
        let s = (self.sector & 0xFF) as u32;

        dbg_log!(LOG::DISK, "get_chs: c={}  h={} s={}", c, h, s);

        return (c * (self.head_count as u32)  + h) * (self.sectors_per_track as u32) + s - 1;
    }

    fn ata_advance(&mut self, cmd: u8, sectors: u32) {
        dbg_log!(LOG::DISK, "Advance sectors={}  old_bytecount={}",sectors, self.bytecount);
        self.bytecount -= sectors;

        if cmd == 0x24 || cmd == 0x29 || cmd == 0x34 ||
            cmd == 0x39 || cmd == 0x25 || cmd == 0x35 {
            let new_sector = (sectors as u32) + self.get_lba48();
            self.sector = (new_sector & 0xFF | new_sector >> 16 & 0xFF00) as u16;
            self.cylinder_low = (new_sector >> 8 & 0xFF) as u16;
            self.cylinder_high = (new_sector >> 16 & 0xFF) as u16;
        } else if self.is_lba > 0 {
            let new_sector = sectors as u32 + self.get_lba28();
            self.sector = (new_sector & 0xFF) as u16;
            self.cylinder_low = (new_sector >> 8 & 0xFF) as u16;
            self.cylinder_high = (new_sector >> 16 & 0xFF) as u16;
            self.head = self.head & !0xF | (new_sector & 0xF) as u8;
        } else {
            // chs
            let new_sector = sectors as u32 + self.get_chs();

            let c = new_sector / ((self.head_count * self.sectors_per_track) as u32) | 0;
            self.cylinder_low = (c & 0xFF) as u16;
            self.cylinder_high = (c >> 8 & 0xFF) as u16;
            self.head = ((new_sector / (self.sectors_per_track as u32) | 0) % (self.head_count  as u32) & 0xF) as u8;
            self.sector = ((new_sector % (self.sectors_per_track as u32) + 1) & 0xFF) as u16;

            assert!(new_sector == self.get_chs());
        }
    }

    #[inline]
    fn push_irq(&mut self) {
        self.store.ide_mut().map(|ide| {
            ide.push_irq();
        });
    }

    #[inline]
    fn get_count(&mut self, is_lba48: bool) -> u32 {
        if is_lba48 {
            let mut count = self.bytecount;
            if count == 0 {
                count = 0x10000;
            }
            count
        } else {
            let mut count = self.bytecount & 0xFF;
            if count == 0 {
                count = 0x100;
            }
            count
        }
    }

    #[inline]
    fn get_lba(&mut self, is_lba48: bool) -> u32 {
        if is_lba48 {
            return self.get_lba48();
        } else if self.is_lba > 0 {
            return self.get_lba28();
        } else {
            return self.get_chs();
        }
    }

    #[inline]
    fn do_ata_read_sectors_dma(&mut self) {
        let cmd = self.current_command;

        let is_lba48 = cmd == 0x25;
        let count = self.get_count(is_lba48);
        let lba = self.get_lba(is_lba48);

        let byte_count = (count as usize) * self.sector_size as usize;
        let start = (lba * self.sector_size) as usize;
        let buffer = self.buffer.as_mut().unwrap();
        assert!((lba as usize) < buffer.byte_length());
        self.report_read_start();


        let orig_prdt_start = self.store.ide().map(|ide| ide.prdt_addr).unwrap();
        let is_master = self.is_master;
        self.read_buffer(start, byte_count, Box::new(move |store, data| {
            store.ide_mut().map(|ide| {
                dbg_log!(LOG::DISK, "do_ata_read_sectors_dma: Data arrived");
                let mut prdt_start = ide.prdt_addr;
                let mut prdt_start = store.ide().map(|ide| ide.prdt_addr).unwrap();
                let mut offset = 0usize;
                
                assert!(orig_prdt_start == prdt_start);
                let mut end = 0;

                while end == 0 {
                    let (prd_addr, mut prd_count, e) = store.cpu_mut().map(|cpu| {
                        let addr = cpu.read32s(prdt_start);
                        let count = cpu.read16(prdt_start + 4) as usize;
                        let e = (cpu.read8(prdt_start + 7) & 0x80) as u8;
                        (addr, count, e)
                    }).unwrap();
                    end = e;

                    if prd_count == 0 {
                        prd_count = 0x10000;
                        dbg_log!(LOG::DISK, "dma: prd count was 0");
                    }

                    dbg_log!(
                        LOG::DISK, 
                        "dma read transfer dest={:#X} prd_count={:#X}",
                        prd_addr, prd_count
                    );
                    store.cpu_mut().map(|cpu| {
                        cpu.mem8_write_slice(prd_addr as usize, &data[offset..offset + prd_count]);
                    });

                    offset += prd_count;
                    prdt_start += 8;
                }

                assert!(offset == byte_count);

                {
                    let ide_i = ide.interface_mut(is_master);
                    ide_i.ata_advance(ide_i.current_command, count);
                    ide_i.status = 0x50;
                }
                ide.dma_status &= !1;
                {
                    let ide_i = ide.interface_mut(is_master);
                    ide_i.current_command = 0xFF;
                    ide_i.push_irq();

                    ide_i.report_read_end(byte_count);
                }
            });
        }));
    }

    fn device_reset(&mut self) {
        if self.is_atapi {
            self.status = 0;
            self.bytecount = 1;
            self.error = 1;
            self.sector = 1; // lba_low
            self.cylinder_low = 0x14; // lba_mid
            self.cylinder_high = 0xEB; // lba_high
        } else {
            self.status = 0x50 | 1;
            self.bytecount = 1;
            self.error = 1;
            self.sector = 1; // lba_low
            // 0, 0 needed by bochs bios
            self.cylinder_low = 0; // lba_mid
            self.cylinder_high = 0; // lba_high
        }

        self.cancel_io_operations();
    }

    #[inline]
    fn cancel_io_operations(&mut self) {
        self.cancelled_io_ids.extend(&self.in_progress_io_ids);
        self.in_progress_io_ids.clear();
    }
    
    fn do_atapi_dma(&mut self) {
        let (dma_status, prdt_addr) = self.store.ide().map_or((false, 0), |ide| {
            if (ide.dma_status & 1) == 0 {
                return (false, 0);
            }
            return (true, ide.prdt_addr);
        });
        if !dma_status {
            dbg_log!(LOG::DISK, "do_atapi_dma: Status not set");
            return
        }
        

        if (self.status & 0x8) == 0 {
            dbg_log!(LOG::DISK, "do_atapi_dma: DRQ not set");
            return;
        }

        dbg_log!(LOG::DISK, "atapi dma transfer len={}", self.data_length);

        let mut prdt_start = prdt_addr;
        let mut offset = 0;

        let data = &self.data;
        let mut end: u8 = 0;

        while end == 0 {
            let (addr, mut count, e) = self.store.cpu_mut().map(|cpu| {
                let addr = cpu.read32s(prdt_start);
                let count = cpu.read16(prdt_start + 4) as u32;
                let e = (cpu.read8(prdt_start + 7) & 0x80) as u8;
                (addr, count, e)
            }).unwrap();
            end = e;

            if count == 0 {
                count = 0x10000;
            }

            dbg_log!(
                LOG::DISK,
                "dma read dest={:#X} count={:#X} datalen={:#X}",
                addr, count, self.data_length
            );
            self.store.cpu_mut().map(|cpu| {
                let l = ((offset + count) as usize).min(self.data_length);
                cpu.mem8_write_slice(addr as usize, &data[offset as usize..l]);
            });

            offset += count;
            prdt_start += 8;

            if offset as usize >= self.data_length && end == 0 {
                dbg_log!(
                    LOG::DISK, 
                    "leave early end={} offset={:#X} data_length={:#X} cmd={:#X}",
                    end, offset, self.data_length, self.current_command
                );
                break;
            }
        }

        dbg_log!(LOG::DISK, "end offset={}", offset);

        self.status = 0x50;
        self.store.ide_mut().map(|ide| {
            ide.dma_status &= !1;
        });
        self.bytecount = self.bytecount & !7 | 3;
        self.push_irq();
    }

    fn do_ata_write_sectors_dma(&mut self) {
        let cmd = self.current_command;

        let is_lba48 = cmd == 0x35;
        let count = self.get_count(is_lba48);
        let lba = self.get_lba(is_lba48);

        let byte_count = (count as u32) * self.sector_size;
        let start = (lba * self.sector_size) as usize;

        let mut prdt_start = self.store.ide().map(|ide| ide.prdt_addr).unwrap();
        let mut offset: usize = 0;

        dbg_log!(LOG::DISK, "prdt addr: {:#X}", prdt_start);

        let mut buffer = vec![0; byte_count as usize];
        let mut end: u8 = 0;
        while end == 0 {
            let (prd_addr, mut prd_count, e) = self.store.cpu_mut().map(|cpu| {
                let addr = cpu.read32s(prdt_start) as usize;
                let count = cpu.read16(prdt_start + 4) as usize;
                let e = (cpu.read8(prdt_start + 7) & 0x80) as u8;
                (addr, count, e)
            }).unwrap();
            end = e;

            if prd_count == 0 {
                prd_count = 0x10000;
                dbg_log!(LOG::DISK, "dma: prd count was 0");
            }

            dbg_log!(
                LOG::DISK,
                "dma write transfer dest={:#X} prd_count={:#X}",
                prd_addr, prd_count
            );

            //var slice = this.cpu.mem8.subarray(prd_addr, prd_addr + prd_count);
            self.store.cpu_mut().map(|cpu| {
                let sl = &mut buffer[offset..offset+prd_count];
                assert!(sl.len() == prd_count);
                cpu.mem8_read_slice(prd_addr, sl);
            });
            

            //replace by upper code, buffer.set(slice, offset);

            //if(DEBUG)
            //{
            //    dbg_log(hex_dump(slice), LOG_DISK);
            //}

            offset += prd_count;
            prdt_start += 8;
        }

        assert!(offset as usize == buffer.len());
        let is_master = self.is_master;
        self.buffer.as_mut().map(|b| {
            b.set(start, &buffer, Box::new(move |store| {
                store.ide_mut().map(|ide| {
                    dbg_log!(LOG::DISK, "dma write completed"); 
                    let ide_i = ide.interface_mut(is_master);
                    ide_i.ata_advance(ide_i.current_command, count);
                    ide_i.status = 0x50;
                    ide_i.push_irq();
                    ide_i.current_command = 0xFF;
                    ide.dma_status &= !1;
                });
            }));
        });
        self.report_write(byte_count as _);
    }

}

pub(crate) struct IDEDevice {
    store: StoreT,
    nr: u8,
    irq: u8, 
    pci_id: u16,
    ata_port: u16, 
    prdt_addr: u32, 
    dma_status: u8, 
    is_master: bool,
    dma_command: u8,
    master_port: u16, 
    device_control: u8, 
    ata_port_high: u16, 
    slave: IDEInterface,
    master: IDEInterface,
}

impl IDEDevice {

    pub fn new(
        store: StoreT,
        master_buffer: Option<Box<dyn FileBuffer>>,
        slave_buffer: Option<Box<dyn FileBuffer>>,
        is_cd: bool,
        nr: u8,
    ) -> Self {
        let (ata_port, irq, pci_id) =  match nr {
            0 => (0x1F0, 14, 0x1E << 3),
            1 => (0x170, 15, 0x1F << 3),
            _ => {
                assert!(false, "IDE device with nr {} ignored", nr);
                (0, 0, 0)
            }
        };
        let dma_status = 0;
        let prdt_addr = 0;
        let dma_command = 0;
        let ata_port_high = ata_port|0x204;
        let device_control = 2;
        let is_master = true;
        let master_port = 0xB400;
        let master = IDEInterface::new(store.clone(), master_buffer, is_cd, nr, true);
        let slave = IDEInterface::new(store.clone(), slave_buffer, is_cd, nr, false);
        Self {
            nr,
            irq,
            store,
            slave,
            master,
            pci_id,
            ata_port,
            prdt_addr,
            is_master,
            dma_status,
            dma_command,
            master_port,
            ata_port_high,
            device_control,
        }
    }

    pub fn init(&mut self) {
        self.master.init();
        self.slave.init();
        self.store.io_mut().map(|io| {
            io.register_read8(
                (self.ata_port as u32) | 7, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        dbg_log!(LOG::DISK, "lower irq {}", ide.irq);
                        dev.cpu_mut().map(|cpu| cpu.device_lower_irq(ide.irq));
                        ide.read_status()
                    })
                }
            );

            io.register_read8(
                (self.ata_port_high as u32) | 2, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        ide.read_status()
                    })
                }
            );

            io.register_write8(
                (self.ata_port_high as u32) | 2, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, val: u8| {
                    dev.ide_mut().map(|ide| {
                        ide.write_control(val);
                    });
                }
            );
            io.register_read(
                self.ata_port as u32|0, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide_mut().map_or(0, |ide| {
                        ide.current_interface_mut().read_data(1) as u8
                    })
                }
                , 
                |dev: &Dev, _addr: u32| {
                    dev.ide_mut().map_or(0, |ide| {
                        ide.current_interface_mut().read_data(2) as u16
                    })
                }
                , 
                |dev: &Dev, _addr: u32| {
                    dev.ide_mut().map_or(0, |ide| {
                        ide.current_interface_mut().read_data(4)
                    })
                }
            );

            io.register_read8(
                (self.ata_port as u32) | 1, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        dbg_log!(
                            LOG::DISK, 
                            "Read error: {:#X} slave={}",
                            ide.current_interface().error & 0xFF,
                            !ide.is_master,
                        );
                        (ide.current_interface().error & 0xFF) as u8
                    })
                }
            );

            io.register_read8(
                (self.ata_port as u32) | 2, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        dbg_log!(
                            LOG::DISK, 
                            "Read bytecount: {:#X}",
                            ide.current_interface().bytecount & 0xFF,
                        );
                        (ide.current_interface().bytecount & 0xFF) as u8
                    })
                }
            );

            io.register_read8(
                (self.ata_port as u32) | 3, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        dbg_log!(
                            LOG::DISK, 
                            "Read sector: {:#X}",
                            ide.current_interface().sector & 0xFF,
                        );
                        (ide.current_interface().sector & 0xFF) as u8
                    })
                }
            );

            io.register_read8(
                (self.ata_port as u32) | 4, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        dbg_log!(
                            LOG::DISK, 
                            "Read 1F4: {:#X}",
                            ide.current_interface().cylinder_low & 0xFF,
                        );
                        (ide.current_interface().cylinder_low & 0xFF) as u8
                    })
                }
            );

            io.register_read8(
                (self.ata_port as u32) | 5, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        dbg_log!(
                            LOG::DISK, 
                            "Read 1F5: {:#X}",
                            ide.current_interface().cylinder_high & 0xFF,
                        );
                        (ide.current_interface().cylinder_high & 0xFF) as u8
                    })
                }
            );

            io.register_read8(
                (self.ata_port as u32) | 6, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        dbg_log!(
                            LOG::DISK, 
                            "Read 1F6",
                        );
                        (ide.current_interface().drive_head & 0xFF) as u8
                    })
                }
            );

            io.register_write(
                (self.ata_port as u32) | 0, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, val: u8| {
                    dev.ide_mut().map(|ide| {
                        ide.current_interface_mut().write_data_port8(val);
                    });
                },
                |dev: &Dev, _addr: u32, val: u16| {
                    dev.ide_mut().map(|ide| {
                        ide.current_interface_mut().write_data_port16(val);
                    });
                },
                |dev: &Dev, _addr: u32, val: u32| {
                    dev.ide_mut().map(|ide| {
                        ide.current_interface_mut().write_data_port32(val);
                    });
                }
            );

            io.register_write8(
                (self.ata_port as u32) | 1, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.ide_mut().map(|ide| {
                        let data = data as u16;
                        dbg_log!(LOG::DISK, "1F1/lba_count: {:#X}", data);
                        ide.master.lba_count = ((ide.master.lba_count as u16) << 8 | data) & 0xFFFF;
                        ide.slave.lba_count = ((ide.slave.lba_count as u16) << 8 | data) & 0xFFFF;
                    });
                },
            );

            io.register_write8(
                (self.ata_port as u32) | 2, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.ide_mut().map(|ide| {
                        let data = data as u32;
                        dbg_log!(LOG::DISK, "1F2/bytecount: {:#X}", data);
                        ide.master.bytecount = ((ide.master.bytecount as u32) << 8 | data) & 0xFFFF;
                        ide.slave.bytecount = ((ide.slave.bytecount as u32) << 8 | data) & 0xFFFF;
                    });
                },
            );

            io.register_write8(
                (self.ata_port as u32) | 3, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.ide_mut().map(|ide| {
                        let data = data as u16;
                        dbg_log!(LOG::DISK, "1F3/sector: {:#X}", data);
                        ide.master.sector = ((ide.master.sector as u16) << 8 | data) & 0xFFFF;
                        ide.slave.sector = ((ide.slave.sector as u16) << 8 | data) & 0xFFFF;
                    });
                },
            );
            
            io.register_write8(
                (self.ata_port as u32) | 4, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.ide_mut().map(|ide| {
                        let data = data as u16;
                        dbg_log!(LOG::DISK, "1F4/sector low: {:#X}", data);
                        ide.master.cylinder_low = ((ide.master.cylinder_low as u16) << 8 | data) & 0xFFFF;
                        ide.slave.cylinder_low = ((ide.slave.cylinder_low as u16) << 8 | data) & 0xFFFF;
                    });
                },
            );

            io.register_write8(
                (self.ata_port as u32) | 5, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.ide_mut().map(|ide| {
                        let data = data as u16;
                        dbg_log!(LOG::DISK, "1F5/sector high: {:#X}", data);
                        ide.master.cylinder_high = ((ide.master.cylinder_high as u16) << 8 | data) & 0xFFFF;
                        ide.slave.cylinder_high = ((ide.slave.cylinder_high as u16) << 8 | data) & 0xFFFF;
                    });
                },
            );

            io.register_write8(
                (self.ata_port as u32) | 6, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.ide_mut().map(|ide| {
                        let slave = data & 0x10;
                        let mode = data & 0xE0;
                        dbg_log!(LOG::DISK, "1F6/drive: {:#X}", data);
                        if slave > 0 {
                            dbg_log!(LOG::DISK, "Slave");
                            ide.is_master = false;
                        } else {
                            ide.is_master = true;
                        }
                        ide.master.drive_head = data as u16;
                        ide.slave.drive_head = data as u16;
                        ide.slave.is_lba = data >> 6 & 1;
                        ide.master.is_lba = ide.slave.is_lba;
                        ide.slave.head = data & 0xF;
                        ide.master.head = ide.slave.head;
                    });
                },
            );

            io.register_write8(
                (self.ata_port as u32) | 7, 
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.ide_mut().map(|ide| {
                        dbg_log!(LOG::DISK, "lower irq {}", ide.irq);
                        dev.cpu_mut().map(|cpu| cpu.device_lower_irq(ide.irq));
                        ide.current_interface_mut().ata_command(data);
                    });
                }
            );

            io.register_read(
                (self.master_port as u32) | 4,
                Dev::Emulator(self.store.clone()),
                IO::empty_read8,
                IO::empty_read16,
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        ide.dma_read_addr()
                    })
                }
            );

            io.register_write(
                (self.master_port as u32) | 4,
                Dev::Emulator(self.store.clone()),
                IO::empty_write8,
                IO::empty_write16,
                |dev: &Dev, _addr: u32, data: u32| {
                    dev.ide_mut().map(|ide| {
                        ide.dma_set_addr(data);
                    });
                }
            );

            io.register_read(
                self.master_port as u32,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide_mut().map_or(0, |ide| {
                        ide.dma_read_command8()
                    })
                },
                IO::empty_read16,
                |dev: &Dev, _addr: u32| {
                    dev.ide_mut().map_or(0, |ide| {
                        ide.dma_read_command()
                    })
                }
            );

            io.register_write(
                self.master_port as u32,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.ide_mut().map(|ide| {
                        ide.dma_write_command8(data);
                    });
                },
                IO::empty_write16,
                |dev: &Dev, _addr: u32, data: u32| {
                    dev.ide_mut().map(|ide| {
                        ide.dma_write_command(data);
                    });
                }
            );

            io.register_read8(
                self.master_port as u32 | 2,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide_mut().map_or(0, |ide| {
                        ide.dma_read_status()
                    })
                }
            );

            io.register_write8(
                self.master_port as u32 | 2,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32, data: u8| {
                    dev.ide_mut().map(|ide| {
                        ide.dma_write_status(data);
                    });
                }
            );

            io.register_read8(
                self.master_port as u32 | 0x8,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        dbg_log!(LOG::DISK, "DMA read 0x8"); 
                        0
                    })
                }
            );

            io.register_read8(
                self.master_port as u32 | 0xA,
                Dev::Emulator(self.store.clone()),
                |dev: &Dev, _addr: u32| {
                    dev.ide().map_or(0, |ide| {
                        dbg_log!(LOG::DISK, "DMA read 0xA"); 
                        0
                    })
                }
            );

        });
        let pci_space = vec![
            0x86, 0x80, 0x10, 0x70, 0x05, 0x00, 0xA0, 0x02,
            0x00, 0x80, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,
            0 | 1, 0, 0x00, 0x00,
            0 | 1, 0, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // second device
            0x00, 0x00, 0x00, 0x00, // second device
            (self.master_port & 0xFF) as u8 | 1,   (self.master_port >> 8) as u8, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x43, 0x10, 0xD4, 0x82,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, self.irq, 0x01, 0x00, 0x00,

            // 0x40
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            // 0x80
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let pci_bars = vec![
            Some(PCIBar::new(8)),
            Some(PCIBar::new(4)),
            None,
            None,
            Some(PCIBar::new(0x10))
        ];
        self.store.pci_mut().map(|pci| {
            let pci_dev = GenericPCIDevice::new(
                self.pci_id,
                pci_space,
                pci_bars,
                &format!("ide{}", self.nr),
            );
            pci.register_device(pci_dev);
        });
    }

    #[inline]
    fn dma_read_status(&mut self) -> u8 {
        dbg_log!(LOG::DISK, "DMA read status: {:#X}", self.dma_status);
        self.dma_status
    }

    #[inline]
    fn dma_write_status(&mut self, value: u8) {
        dbg_log!(LOG::DISK, "DMA set status: {:#X}", value);
        self.dma_status &= !(value & 6);
    }

    #[inline]
    fn dma_write_command(&mut self, value: u32) {
        dbg_log!(LOG::DISK, "DMA write command: {:#X}", value);

        self.dma_write_command8((value & 0xFF) as u8);
        self.dma_write_status((value >> 16 & 0xFF) as u8);
    }

    fn dma_write_command8(&mut self, value: u8) {
        dbg_log!(LOG::DISK, "DMA write command8: {:#X}", value);
    
        let old_command = self.dma_command;
        self.dma_command = value & 0x9;
    
        if (old_command & 1) == (value & 1) {
            return;
        }
    
        if (value & 1) == 0 {
            self.dma_status &= !1;
            return;
        }
    
        self.dma_status |= 1;
        let current_command = self.current_interface().current_command;
        match  current_command {
            0x25 | 0xC8 => {
                self.current_interface_mut().do_ata_read_sectors_dma();
            }
            0xCA | 0x35 => {
                self.current_interface_mut().do_ata_write_sectors_dma();
            }
            0xA0 => {
                self.current_interface_mut().do_atapi_dma();
            }
            _ => {
                dbg_log!(
                    LOG::DISK, 
                    "Spurious dma command write, current command: {:#X}",
                    current_command
                );
                assert!(false);

            }
        }
    }

    #[inline]
    fn dma_read_command(&self) -> u32 {
        return self.dma_read_command8() as u32 | (self.dma_read_command8() as u32) << 16;
    }

    #[inline]
    fn dma_read_command8(&self) -> u8 {
        dbg_log!(LOG::DISK, "DMA read command: {:#X}", self.dma_command);
        return self.dma_command;
    }

    #[inline]
    fn dma_set_addr(&mut self, data: u32) {
        dbg_log!(LOG::DISK, "dma set address: {:#X}", data);
        self.prdt_addr = data;
    }

    #[inline]
    fn dma_read_addr(&self) -> u32 {
        dbg_log!(LOG::DISK, "dma get address: {:#X}", self.prdt_addr);
        return self.prdt_addr;
    }

    fn write_control(&mut self, data: u8) {
        let inter = if data & 2 > 0 {
            "disabled"
        } else {
            "enabled"
        };
        dbg_log!(LOG::DISK, "set device control: {:#X} interrupts {}", data, inter);

        if data & 4 > 0 {
            dbg_log!(LOG::DISK, "Reset via control port");

            self.store.cpu_mut().map(|cpu| cpu.device_lower_irq(self.irq));
            self.master.device_reset();
            self.slave.device_reset();
        }
        self.device_control = data;
    }

    #[inline]
    fn read_status(&self) -> u8 {
        if self.current_interface().buffer.is_some() {
            let ret = self.current_interface().status;
            dbg_log!(LOG::DISK, "ATA read status: {:#X}", ret);
            return ret;
        } else {
            return 0;
        }
    }

    #[inline]
    fn current_interface(&self) ->&IDEInterface {
        if self.is_master {
            &self.master
        } else {
            &self.slave
        }
    }

    #[inline]
    fn interface(&mut self, master: bool) ->&IDEInterface {
        if master {
            &self.master
        } else {
            &self.slave
        }
    }

    #[inline]
    fn interface_mut(&mut self, master: bool) ->&mut IDEInterface {
        if master {
            &mut self.master
        } else {
            &mut self.slave
        }
    }

    #[inline]
    fn current_interface_mut(&mut self) ->&mut IDEInterface {
        if self.is_master {
            &mut self.master
        } else {
            &mut self.slave
        }
    }

    #[inline]
    fn push_irq(&mut self) {
        if (self.device_control & 2) == 0 {
            dbg_log!(LOG::DISK, "push irq");
            self.dma_status |= 4;
            self.store.cpu_mut().map(|cpu| cpu.device_raise_irq(self.irq));
        }
    }

}

