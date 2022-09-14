pub const MMAP_BLOCK_BITS: usize = 17;
pub const MMAP_BLOCK_SIZE: usize = 1 << MMAP_BLOCK_BITS;
pub const FLAG_INTERRUPT: usize = 512;
pub const TIME_PER_FRAME: usize = 1;

pub const CMOS_RTC_SECONDS: u8 = 0x00;
pub const CMOS_RTC_SECONDS_ALARM: u8 = 0x01;
pub const CMOS_RTC_MINUTES: u8 = 0x02;
pub const CMOS_RTC_MINUTES_ALARM: u8 = 0x03;
pub const CMOS_RTC_HOURS: u8 = 0x04;
pub const CMOS_RTC_HOURS_ALARM: u8 = 0x05;
pub const CMOS_RTC_DAY_WEEK: u8 = 0x06;
pub const CMOS_RTC_DAY_MONTH: u8 = 0x07;
pub const CMOS_RTC_MONTH: u8 = 0x08;
pub const CMOS_RTC_YEAR: u8 = 0x09;
pub const CMOS_STATUS_A: u8 = 0x0a;
pub const CMOS_STATUS_B: u8 = 0x0b;
pub const CMOS_STATUS_C: u8 = 0x0c;
pub const CMOS_STATUS_D: u8 = 0x0d;
pub const CMOS_RESET_CODE: u8 = 0x0f;

pub const CMOS_FLOPPY_DRIVE_TYPE: u8 = 0x10;
pub const CMOS_DISK_DATA: u8 = 0x12;
pub const CMOS_EQUIPMENT_INFO: u8 = 0x14;
pub const CMOS_MEM_BASE_LOW: u8 = 0x15;
pub const CMOS_MEM_BASE_HIGH: u8 = 0x16;
pub const CMOS_MEM_OLD_EXT_LOW: u8 = 0x17;
pub const CMOS_MEM_OLD_EXT_HIGH: u8 = 0x18;
pub const CMOS_DISK_DRIVE1_TYPE: u8 = 0x19;
pub const CMOS_DISK_DRIVE2_TYPE: u8 = 0x1a;
pub const CMOS_DISK_DRIVE1_CYL: u8 = 0x1b;
pub const CMOS_DISK_DRIVE2_CYL: u8 = 0x24;
pub const CMOS_MEM_EXTMEM_LOW: u8 = 0x30;
pub const CMOS_MEM_EXTMEM_HIGH: u8 = 0x31;
pub const CMOS_CENTURY: u8 = 0x32;
pub const CMOS_MEM_EXTMEM2_LOW: u8 = 0x34;
pub const CMOS_MEM_EXTMEM2_HIGH: u8 = 0x35;
pub const CMOS_BIOS_BOOTFLAG1: u8 = 0x38;
pub const CMOS_BIOS_DISKTRANSFLAG: u8 = 0x39;
pub const CMOS_BIOS_BOOTFLAG2: u8 = 0x3d;
pub const CMOS_MEM_HIGHMEM_LOW: u8 = 0x5b;
pub const CMOS_MEM_HIGHMEM_MID: u8 = 0x5c;
pub const CMOS_MEM_HIGHMEM_HIGH: u8 = 0x5d;
pub const CMOS_BIOS_SMP_COUNT: u8 = 0x5f;

pub const WASM_TABLE_SIZE: u32 = 900;

/** @const */
pub const WASM_TABLE_OFFSET: u32 = 1024;

pub const FW_CFG_SIGNATURE: u16 = 0x00;
pub const FW_CFG_ID: u16 = 0x01;
pub const FW_CFG_RAM_SIZE: u16 = 0x03;
pub const FW_CFG_NB_CPUS: u16 = 0x05;
pub const FW_CFG_MAX_CPUS: u16 = 0x0F;
pub const FW_CFG_NUMA: u16 = 0x0D;
pub const FW_CFG_FILE_DIR: u16 = 0x19;
pub const FW_CFG_CUSTOM_START: u16 = 0x8000;
pub const FW_CFG_FILE_START: u16 = 0xC000;
pub const FW_CFG_SIGNATURE_QEMU: u32 = 0x554D4551;
