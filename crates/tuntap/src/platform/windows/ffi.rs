use std::{io, ptr, mem};

use winapi::shared::basetsd::ULONG_PTR;
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::*;
use winapi::shared::netioapi::ConvertInterfaceLuidToAlias;
use winapi::shared::netioapi::ConvertInterfaceLuidToGuid;
use winapi::shared::winerror::*;

use winapi::um::combaseapi::StringFromGUID2;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::CreateFileW;
use winapi::um::fileapi::ReadFile;
use winapi::um::fileapi::WriteFile;
use winapi::um::fileapi::OPEN_EXISTING;
use winapi::um::handleapi::*;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::setupapi::*;
use winapi::um::synchapi::*;
use winapi::um::winreg::*;

use winapi::shared::ifdef::NET_LUID;
use winapi::um::winnt::*;

use winreg::RegKey;

use scopeguard::{guard, ScopeGuard};

use crate::{Result, Error};
use super::dev::GUID_NETWORK_ADAPTER;
use super::{encode_utf16, decode_utf16};
use super::HARDWARE_ID;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Clone, Copy)]
/// Custom type to handle variable size SP_DRVINFO_DETAIL_DATA_W
pub struct SP_DRVINFO_DETAIL_DATA_W2 {
    pub cbSize: DWORD,
    pub InfDate: FILETIME,
    pub CompatIDsOffset: DWORD,
    pub CompatIDsLength: DWORD,
    pub Reserved: ULONG_PTR,
    pub SectionName: [WCHAR; 256],
    pub InfFileName: [WCHAR; 260],
    pub DrvDescription: [WCHAR; 256],
    pub HardwareID: [WCHAR; 512],
}

fn class_name_from_guid(guid: &GUID) -> Result<Vec<WCHAR>> {
    let mut class_name = vec![0; 32];
    syscall!(SetupDiClassNameFromGuidW(
        guid,
        class_name.as_mut_ptr(),
        class_name.len() as _,
        ptr::null_mut(),
    ));
    Ok(class_name)
}

pub fn luid_to_alias(luid: &NET_LUID) -> Result<Vec<WCHAR>> {
    // IF_MAX_STRING_SIZE + 1
    let mut alias = vec![0; 257];

    match unsafe {
        ConvertInterfaceLuidToAlias(luid, alias.as_mut_ptr(), alias.len())
    } {
        0 => Ok(alias),
        err => Err(Error::Io(io::Error::from_raw_os_error(err as _))),
    }
}

fn create_device_info(
    devinfo: HDEVINFO,
    device_name: &[WCHAR],
    guid: &GUID,
    device_description: &[WCHAR],
    creation_flags: DWORD
) -> Result<SP_DEVINFO_DATA> {
    let mut devinfo_data: SP_DEVINFO_DATA = unsafe { mem::zeroed() };
    devinfo_data.cbSize = mem::size_of_val(&devinfo_data) as _;

    syscall!(SetupDiCreateDeviceInfoW(
        devinfo,
        device_name.as_ptr(),
        guid,
        device_description.as_ptr(),
        ptr::null_mut(),
        creation_flags,
        &mut devinfo_data,
    ));
    Ok(devinfo_data)
}

fn enum_driver_info(
    devinfo: HDEVINFO,
    devinfo_data: &SP_DEVINFO_DATA,
    driver_type: DWORD,
    member_index: DWORD
) -> Option<Result<SP_DRVINFO_DATA_W>> {
    let mut drvinfo_data: SP_DRVINFO_DATA_W = unsafe { mem::zeroed() };
    drvinfo_data.cbSize = mem::size_of_val(&drvinfo_data) as _;
    match unsafe {
        SetupDiEnumDriverInfoW(
            devinfo,
            devinfo_data as *const _ as _,
            driver_type,
            member_index,
            &mut drvinfo_data,
        )
    } {
        0 if unsafe { GetLastError() == ERROR_NO_MORE_ITEMS } => None,
        0 => Some(Err(Error::Io(io::Error::last_os_error()))),
        _ =>Some(Ok(drvinfo_data)),
    }
}

fn get_driver_info_detail(
    devinfo: HDEVINFO,
    devinfo_data: &SP_DEVINFO_DATA,
    drvinfo_data: &SP_DRVINFO_DATA_W,
) -> Result<SP_DRVINFO_DETAIL_DATA_W2> {
    let mut drvinfo_detail: SP_DRVINFO_DETAIL_DATA_W2 = unsafe { mem::zeroed() };
    drvinfo_detail.cbSize = mem::size_of::<SP_DRVINFO_DETAIL_DATA_W>() as _;
    syscall!(SetupDiGetDriverInfoDetailW(
        devinfo,
        devinfo_data as *const _ as _,
        drvinfo_data as *const _ as _,
        &mut drvinfo_detail as *mut _ as _,
        mem::size_of_val(&drvinfo_detail) as _,
        ptr::null_mut(),
    ));
    Ok(drvinfo_detail)
}

fn set_selected_driver(
    devinfo: HDEVINFO,
    devinfo_data: &SP_DEVINFO_DATA,
    drvinfo_data: &SP_DRVINFO_DATA_W,
) -> Result<()>  {
    syscall!(SetupDiSetSelectedDriverW(
        devinfo,
        devinfo_data as *const _ as _,
        drvinfo_data as *const _ as _,
    ));
    Ok(())
}

fn open_dev_reg_key(
    devinfo: HDEVINFO,
    devinfo_data: &SP_DEVINFO_DATA,
    scope: DWORD,
    hw_profile: DWORD,
    key_type: DWORD,
    sam_desired: REGSAM,
) -> Result<HKEY> {
    const INVALID_KEY_VALUE: HKEY = INVALID_HANDLE_VALUE as _;

    unsafe {
        match SetupDiOpenDevRegKey(
            devinfo,
            devinfo_data as *const _ as _,
            scope,
            hw_profile,
            key_type,
            sam_desired,
        ) {
            INVALID_KEY_VALUE => Err(Error::Io(io::Error::last_os_error())),
            key => Ok(key),
        }
    }
}

fn notify_change_key_value(
    key: HKEY,
    watch_subtree: BOOL,
    notify_filter: DWORD,
    milliseconds: DWORD,
) -> Result<()> {
    let event = syscall_handle!(CreateEventW(ptr::null_mut(), FALSE, FALSE, ptr::null()))?;
    match unsafe {
        RegNotifyChangeKeyValue(key, watch_subtree, notify_filter, event, TRUE)
    } {
        0 => Ok(()),
        err => Err(Error::Io(io::Error::from_raw_os_error(err))),
    }?;
    match unsafe { WaitForSingleObject(event, milliseconds) } {
        0 => Ok(()),
        0x102 => Err(Error::Io(io::Error::new(
            io::ErrorKind::TimedOut,
            "Registry timed out",
        ))),
        _ => Err(Error::Io(io::Error::last_os_error())),
    }
}

pub fn create_interface() -> Result<NET_LUID> {
    let devinfo = syscall_handle!(
        SetupDiCreateDeviceInfoList(&GUID_NETWORK_ADAPTER, ptr::null_mut())
    )?;
    
    let _guard = guard((), |_| {
        unsafe {
            SetupDiDestroyDeviceInfoList(devinfo);
        }
    });
    let class_name = class_name_from_guid(&GUID_NETWORK_ADAPTER)?;
    let devinfo_data = create_device_info(devinfo, &class_name, &GUID_NETWORK_ADAPTER, &encode_utf16(""), DICD_GENERATE_ID)?;
    syscall!(SetupDiSetSelectedDevice(devinfo, &devinfo_data as *const _ as _));
    let hardware_id = &encode_utf16(HARDWARE_ID);
    
    syscall!(SetupDiSetDeviceRegistryPropertyW(
        devinfo, 
        &devinfo_data as *const _ as _,
        SPDRP_HARDWAREID,
        hardware_id.as_ptr() as _,
        hardware_id.len() as u32 * 2,
    ));
    syscall!(SetupDiBuildDriverInfoList(
        devinfo,
        &devinfo_data as *const _ as _,
        SPDIT_COMPATDRIVER,
    ));
    
    let mut driver_version = 0;
    let mut member_index = 0;
    while let Some(drv_info) = enum_driver_info(
        devinfo,
        &devinfo_data,
        SPDIT_COMPATDRIVER,
        member_index,
    ) {
        member_index += 1;
        let drvinfo_data = match drv_info {
            Ok(o) => o,
            _ => continue,
        };
        if drvinfo_data.DriverVersion <= driver_version {
            continue;
        }
        let drvinfo_detail = match get_driver_info_detail(
            devinfo,
            &devinfo_data,
            &drvinfo_data,
        ) {
            Ok(drvinfo_detail) => drvinfo_detail,
            _ => continue,
        };
        
        let is_compatible = drvinfo_detail
            .HardwareID
            .split(|b| *b == 0)
            .map(|id| decode_utf16(id))
            .any(|id| id.eq_ignore_ascii_case(HARDWARE_ID));
        
        if !is_compatible {
            continue;
        }
        match set_selected_driver(devinfo, &devinfo_data, &drvinfo_data) {
            Ok(_) => (),
            Err(_) => continue,
        }
        driver_version = drvinfo_data.DriverVersion;
    }
    
    if driver_version == 0 {
        return Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, "No driver found")));
    }
    let uninstaller = guard((), |_| {
        unsafe {
            SetupDiCallClassInstaller(DIF_REMOVE, devinfo, &devinfo_data as *const _ as _);
        }
    });
    syscall!(SetupDiCallClassInstaller(
        DIF_REGISTERDEVICE,
        devinfo,
        &devinfo_data as *const _ as _,
    ));
    syscall!(SetupDiCallClassInstaller(
        DIF_REGISTER_COINSTALLERS,
        devinfo,
        &devinfo_data as *const _ as _,
    ));
    
    syscall!(SetupDiCallClassInstaller(
        DIF_INSTALLINTERFACES,
        devinfo,
        &devinfo_data  as *const _ as _,
    ));

    syscall!(SetupDiCallClassInstaller(
        DIF_INSTALLDEVICE,
        devinfo,
        &devinfo_data   as *const _ as _,
    ));
    
    let key = open_dev_reg_key(
        devinfo,
        &devinfo_data,
        DICS_FLAG_GLOBAL,
        0,
        DIREG_DRV,
        KEY_QUERY_VALUE | KEY_NOTIFY,
    )?;

    let key = RegKey::predef(key);
    while let Err(_) = key.get_value::<DWORD, &str>("*IfType") {
        notify_change_key_value(
            key.raw_handle(),
            TRUE,
            REG_NOTIFY_CHANGE_NAME,
            2000,
        )?;
    }

    while let Err(_) = key.get_value::<DWORD, &str>("NetLuidIndex") {
        notify_change_key_value(
            key.raw_handle(),
            TRUE,
            REG_NOTIFY_CHANGE_NAME,
            2000,
        )?;
    }

    let if_type: DWORD = key.get_value("*IfType")?;
    let luid_index: DWORD = key.get_value("NetLuidIndex")?;

    ScopeGuard::into_inner(uninstaller);
    let mut luid = NET_LUID { Value: 0 };

    luid.set_IfType(if_type as _);
    luid.set_NetLuidIndex(luid_index as _);

    Ok(luid)
}

pub fn device_io_control(
    handle: HANDLE,
    io_control_code: DWORD,
    in_buffer: &impl Copy,
    out_buffer: &mut impl Copy,
) -> Result<()> {
    let mut junk = 0;

    match unsafe {
        DeviceIoControl(
            handle,
            io_control_code,
            in_buffer as *const _ as _,
            mem::size_of_val(in_buffer) as _,
            out_buffer as *mut _ as _,
            mem::size_of_val(out_buffer) as _,
            &mut junk,
            ptr::null_mut(),
        )
    } {
        0 => Err(Error::Io(io::Error::last_os_error())),
        _ => Ok(()),
    }
}

pub fn enum_device_info(
    devinfo: HDEVINFO,
    member_index: DWORD,
) -> Option<Result<SP_DEVINFO_DATA>> {
    let mut devinfo_data: SP_DEVINFO_DATA = unsafe { mem::zeroed() };
    devinfo_data.cbSize = mem::size_of_val(&devinfo_data) as _;

    match unsafe {
        SetupDiEnumDeviceInfo(devinfo, member_index, &mut devinfo_data)
    } {
        0 if unsafe { GetLastError() == ERROR_NO_MORE_ITEMS } => None,
        0 => Some(Err(Error::Io(io::Error::last_os_error()))),
        _ => Some(Ok(devinfo_data)),
    }
}

fn get_device_registry_property(
    devinfo: HDEVINFO,
    devinfo_data: &SP_DEVINFO_DATA,
    property: DWORD,
) -> Result<Vec<WCHAR>> {
    let mut value = vec![0; 32];

    match unsafe {
        SetupDiGetDeviceRegistryPropertyW(
            devinfo,
            devinfo_data as *const _ as _,
            property,
            ptr::null_mut(),
            value.as_mut_ptr() as _,
            (value.len() * 2) as _,
            ptr::null_mut(),
        )
    } {
        0 => Err(Error::Io(io::Error::last_os_error())),
        _ => Ok(value),
    }
}  

fn call_class_installer(
    devinfo: HDEVINFO,
    devinfo_data: &SP_DEVINFO_DATA,
    install_function: DI_FUNCTION,
) -> Result<()> {
    match unsafe {
        SetupDiCallClassInstaller(
            install_function,
            devinfo,
            devinfo_data as *const _ as _,
        )
    } {
        0 => Err(Error::Io(io::Error::last_os_error())),
        _ => Ok(()),
    }
}

fn create_file(
    file_name: &[WCHAR],
    desired_access: DWORD,
    share_mode: DWORD,
    creation_disposition: DWORD,
    flags_and_attributes: DWORD,
) -> Result<HANDLE> {
    match unsafe {
        CreateFileW(
            file_name.as_ptr(),
            desired_access,
            share_mode,
            ptr::null_mut(),
            creation_disposition,
            flags_and_attributes,
            ptr::null_mut(),
        )
    } {
        INVALID_HANDLE_VALUE => Err(Error::Io(io::Error::last_os_error())),
        handle => Ok(handle),
    }
}

fn luid_to_guid(luid: &NET_LUID) -> Result<GUID> {
    let mut guid = unsafe { mem::zeroed() };

    match unsafe { ConvertInterfaceLuidToGuid(luid, &mut guid) } {
        0 => Ok(guid),
        err => Err(Error::Io(io::Error::from_raw_os_error(err as _))),
    }
}

fn string_from_guid(guid: &GUID) -> Result<Vec<WCHAR>> {
    // GUID_STRING_CHARACTERS + 1
    let mut string = vec![0; 39];

    match unsafe {
        StringFromGUID2(guid, string.as_mut_ptr(), string.len() as _)
    } {
        0 => Err(Error::Io(io::Error::new(io::ErrorKind::Other, "Insufficent buffer"))),
        _ => Ok(string),
    }
}

pub fn open_interface(luid: &NET_LUID) -> Result<HANDLE> {
    let guid = luid_to_guid(luid)
        .and_then(|guid| string_from_guid(&guid))?;

    let path = format!(r"\\.\Global\{}.tap", &decode_utf16(&guid));

    create_file(
        &encode_utf16(&path),
        GENERIC_READ | GENERIC_WRITE,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        OPEN_EXISTING,
        FILE_ATTRIBUTE_SYSTEM,
    )
}

fn check_interface(luid: &NET_LUID) -> Result<()> {
    let devinfo = syscall_handle!(SetupDiGetClassDevsW(
        &GUID_NETWORK_ADAPTER, 
        ptr::null(), 
        ptr::null_mut(),
        DIGCF_PRESENT,
    ))?;
    let _guard = guard((), |_| {
        unsafe {
            SetupDiDestroyDeviceInfoList(devinfo);
        }
    });
    let mut member_index = 0;
    while let Some(devinfo_data) = enum_device_info(devinfo, member_index) {
        member_index += 1;

        let devinfo_data = match devinfo_data {
            Ok(devinfo_data) => devinfo_data,
            _ => continue,
        };

        let hardware_id = match get_device_registry_property(
            devinfo,
            &devinfo_data,
            SPDRP_HARDWAREID,
        ) {
            Ok(hardware_id) => hardware_id,
            Err(_) => continue,
        };

        if !decode_utf16(&hardware_id).eq_ignore_ascii_case(HARDWARE_ID) {
            continue;
        }

        let key = match open_dev_reg_key(
            devinfo,
            &devinfo_data,
            DICS_FLAG_GLOBAL,
            0,
            DIREG_DRV,
            KEY_QUERY_VALUE | KEY_NOTIFY,
        ) {
            Ok(key) => RegKey::predef(key),
            Err(_) => continue,
        };

        let if_type: DWORD = match key.get_value("*IfType") {
            Ok(if_type) => if_type,
            Err(_) => continue,
        };

        let luid_index: DWORD = match key.get_value("NetLuidIndex") {
            Ok(luid_index) => luid_index,
            Err(_) => continue,
        };

        let mut luid2 = NET_LUID { Value: 0 };

        luid2.set_IfType(if_type as _);
        luid2.set_NetLuidIndex(luid_index as _);

        if luid.Value != luid2.Value {
            continue;
        }

        // Found it!
        return Ok(());
    }

    Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, "Device not found")))
}

pub fn delete_interface(luid: &NET_LUID) -> Result<()> {
    let devinfo = syscall_handle!(SetupDiGetClassDevsW(
        &GUID_NETWORK_ADAPTER, 
        ptr::null(), 
        ptr::null_mut(),
        DIGCF_PRESENT,
    ))?;
    let _guard = guard((), |_| {
        unsafe {
            SetupDiDestroyDeviceInfoList(devinfo);
        }
    });
    let mut member_index = 0;
    while let Some(devinfo_data) = enum_device_info(devinfo, member_index) {
        member_index += 1;
        let devinfo_data = match devinfo_data {
            Ok(devinfo_data) => devinfo_data,
            Err(_) => continue,
        };

        let hardware_id = match get_device_registry_property(
            devinfo,
            &devinfo_data,
            SPDRP_HARDWAREID,
        ) {
            Ok(hardware_id) => hardware_id,
            Err(_) => continue,
        };

        if !decode_utf16(&hardware_id).eq_ignore_ascii_case(HARDWARE_ID) {
            continue;
        }

        let key = match open_dev_reg_key(
            devinfo,
            &devinfo_data,
            DICS_FLAG_GLOBAL,
            0,
            DIREG_DRV,
            KEY_QUERY_VALUE | KEY_NOTIFY,
        ) {
            Ok(key) => RegKey::predef(key),
            Err(_) => continue,
        };

        let if_type: DWORD = match key.get_value("*IfType") {
            Ok(if_type) => if_type,
            Err(_) => continue,
        };

        let luid_index: DWORD = match key.get_value("NetLuidIndex") {
            Ok(luid_index) => luid_index,
            Err(_) => continue,
        };

        let mut luid2 = NET_LUID { Value: 0 };

        luid2.set_IfType(if_type as _);
        luid2.set_NetLuidIndex(luid_index as _);

        if luid.Value != luid2.Value {
            continue;
        }

        return call_class_installer(devinfo, &devinfo_data, DIF_REMOVE);
    }

    Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, "Device not found")))
}

pub fn read_file(handle: HANDLE, buffer: &mut [u8]) -> io::Result<usize> {
    let mut ret = 0;

    match unsafe {
        ReadFile(
            handle,
            buffer.as_mut_ptr() as _,
            buffer.len() as _,
            &mut ret,
            ptr::null_mut(),
        )
    } {
        0 => Err(io::Error::last_os_error()),
        _ => Ok(ret as _),
    }
}

pub fn write_file(handle: HANDLE, buffer: &[u8]) -> io::Result<usize> {
    let mut ret = 0;

    match unsafe {
        WriteFile(
            handle,
            buffer.as_ptr() as _,
            buffer.len() as _,
            &mut ret,
            ptr::null_mut(),
        )
    } {
        0 => Err(io::Error::last_os_error()),
        _ => Ok(ret as _),
    }
}

pub fn close_handle(handle: HANDLE) -> Result<()> {
    syscall!(CloseHandle(handle));
    Ok(())
}