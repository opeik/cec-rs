use std::mem;

use arrayvec::ArrayVec;
use num_traits::ToPrimitive;

pub use crate::*;

impl From<KnownCecLogicalAddress> for CecLogicalAddress {
    fn from(address: KnownCecLogicalAddress) -> Self {
        address.0
    }
}

impl From<KnownCecLogicalAddress> for cec_logical_address {
    fn from(address: KnownCecLogicalAddress) -> Self {
        address.0.repr()
    }
}

impl From<KnownAndRegisteredCecLogicalAddress> for CecLogicalAddress {
    fn from(address: KnownAndRegisteredCecLogicalAddress) -> Self {
        address.0
    }
}

impl From<KnownAndRegisteredCecLogicalAddress> for cec_logical_address {
    fn from(address: KnownAndRegisteredCecLogicalAddress) -> Self {
        address.0.repr()
    }
}

impl From<CecDatapacket> for cec_datapacket {
    fn from(datapacket: CecDatapacket) -> cec_datapacket {
        let mut data = [0u8; 64];
        data[..datapacket.0.len()].clone_from_slice(datapacket.0.as_slice());
        cec_datapacket {
            data,
            size: datapacket.0.len() as u8,
        }
    }
}

impl From<cec_datapacket> for CecDatapacket {
    fn from(datapacket: cec_datapacket) -> CecDatapacket {
        let end = datapacket.size as usize;
        let mut packet = CecDatapacket(ArrayVec::new());
        packet
            .0
            .try_extend_from_slice(&datapacket.data[..end])
            .unwrap();
        packet
    }
}

impl From<CecCommand> for cec_command {
    fn from(command: CecCommand) -> cec_command {
        cec_command {
            initiator: command.initiator.repr(),
            destination: command.destination.repr(),
            ack: command.ack.into(),
            eom: command.eom.into(),
            opcode: command.opcode.repr(),
            parameters: command.parameters.into(),
            opcode_set: command.opcode_set.into(),
            transmit_timeout: command.transmit_timeout.as_millis() as i32,
        }
    }
}

impl From<CecLogicalAddresses> for cec_logical_addresses {
    fn from(addresses: CecLogicalAddresses) -> cec_logical_addresses {
        // cec_logical_addresses.addresses is a 'mask'
        // cec_logical_addresses.addresses[logical_address value] = 1 when mask contains the address
        let mut data = cec_logical_addresses {
            primary: addresses.primary.into(),
            addresses: [0; 16],
        };
        for known_address in addresses.addresses {
            let address: CecLogicalAddress = known_address.into();
            let address_mask_position = address.repr();
            data.addresses[address_mask_position as usize] = 1;
        }
        data
    }
}

impl From<CecDeviceTypeVec> for cec_device_type_list {
    fn from(device_types: CecDeviceTypeVec) -> cec_device_type_list {
        let mut devices = cec_device_type_list {
            types: [CecDeviceType::Reserved.repr(); 5],
        };
        for (i, type_id) in device_types.0.iter().enumerate() {
            devices.types[i] = (*type_id).repr();
        }
        devices
    }
}

impl From<&CecConnectionCfg> for libcec_configuration {
    fn from(config: &CecConnectionCfg) -> libcec_configuration {
        let mut cfg: libcec_configuration;
        unsafe {
            cfg = mem::zeroed::<libcec_configuration>();
            libcec_clear_configuration(&mut cfg);
        }
        cfg.clientVersion = libcec_version::LIBCEC_VERSION_CURRENT as _;
        cfg.strDeviceName = first_n::<{ LIBCEC_OSD_NAME_SIZE as usize }>(&config.device_name);
        cfg.deviceTypes = config.device_types.clone().into();
        if let Some(v) = config.physical_address {
            cfg.iPhysicalAddress = v;
        }
        if let Some(v) = config.base_device {
            cfg.baseDevice = v.repr();
        }
        if let Some(v) = config.hdmi_port {
            cfg.iHDMIPort = v;
        }
        if let Some(v) = config.tv_vendor {
            cfg.tvVendor = v;
        }
        if let Some(v) = config.wake_devices.clone() {
            cfg.wakeDevices = v.into();
        }
        if let Some(v) = config.power_off_devices.clone() {
            cfg.powerOffDevices = v.into();
        }
        if let Some(v) = config.get_settings_from_rom {
            cfg.bGetSettingsFromROM = v.into();
        }
        if let Some(v) = config.activate_source {
            cfg.bActivateSource = v.into();
        }
        if let Some(v) = config.power_off_on_standby {
            cfg.bPowerOffOnStandby = v.into();
        }
        if let Some(v) = config.device_language.clone() {
            cfg.strDeviceLanguage = first_n::<3>(&v);
        }
        if let Some(v) = config.monitor_only {
            cfg.bMonitorOnly = v.into();
        }
        if let Some(v) = config.adapter_type {
            cfg.adapterType = v.repr();
        }
        if let Some(v) = config.combo_key {
            cfg.comboKey = v.repr();
        }
        if let Some(v) = config.combo_key_timeout {
            cfg.iComboKeyTimeoutMs = v.as_millis().to_u32().unwrap();
        }
        if let Some(v) = config.button_repeat_rate {
            cfg.iButtonRepeatRateMs = v.as_millis().to_u32().unwrap();
        }
        if let Some(v) = config.button_release_delay {
            cfg.iButtonReleaseDelayMs = v.as_millis().to_u32().unwrap();
        }
        if let Some(v) = config.double_tap_timeout {
            cfg.iDoubleTapTimeoutMs = v.as_millis().to_u32().unwrap();
        }
        if let Some(v) = config.autowake_avr {
            cfg.bAutoWakeAVR = v.into();
        }
        cfg
    }
}
