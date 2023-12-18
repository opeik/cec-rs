mod convert;
mod types;

use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
    ffi::{c_int, CStr, CString},
    fmt,
    os::raw::c_void,
    pin::Pin,
    result,
    time::Duration,
};

use arrayvec::ArrayVec;
use cec_sys::*;
use derive_builder::Builder;
use log::trace;
use thiserror::Error;

pub use crate::types::*;

fn first_n<const N: usize>(string: &str) -> [::std::os::raw::c_char; N] {
    let mut data: [::std::os::raw::c_char; N] = [0; N];
    let bytes = string.as_bytes();
    for (dst, src) in data.iter_mut().zip(bytes) {
        // c_char is either u8 or i8. We use simple casting to convert u8 accordingly
        *dst = *src as _;
    }
    data
}

/// CecLogicalAddress which does not allow Unknown variant
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct KnownCecLogicalAddress(CecLogicalAddress);

impl KnownCecLogicalAddress {
    pub fn new(address: CecLogicalAddress) -> Option<Self> {
        match address {
            CecLogicalAddress::Unknown => None,
            valid_address => Some(Self(valid_address)),
        }
    }
}

/// CecLogicalAddress which does not allow Unknown and Unregistered variants
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct KnownAndRegisteredCecLogicalAddress(CecLogicalAddress);

impl KnownAndRegisteredCecLogicalAddress {
    pub fn new(address: CecLogicalAddress) -> Option<Self> {
        match address {
            CecLogicalAddress::Unknown | CecLogicalAddress::Unregistered => None,
            valid_address => Some(Self(valid_address)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UnregisteredCecLogicalAddress {}
impl TryFrom<KnownCecLogicalAddress> for KnownAndRegisteredCecLogicalAddress {
    type Error = UnregisteredCecLogicalAddress;

    fn try_from(address: KnownCecLogicalAddress) -> Result<Self, Self::Error> {
        let unchecked_address = address.0;
        Self::new(unchecked_address).ok_or(Self::Error {})
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecDatapacket(pub ArrayVec<u8, 64>);

#[derive(Debug, Clone)]
pub struct CecCommand {
    /// The logical address of the initiator of this message.
    pub initiator: CecLogicalAddress,
    /// The logical address of the destination of this message.
    pub destination: CecLogicalAddress,
    /// 1 when the ACK bit is set, 0 otherwise.
    pub ack: bool,
    /// 1 when the EOM bit is set, 0 otherwise.
    pub eom: bool,
    /// The opcode of this message.
    pub opcode: CecOpcode,
    /// The parameters attached to this message.
    pub parameters: CecDatapacket,
    /// 1 when an opcode is set, 0 otherwise (POLL message).
    pub opcode_set: bool,
    /// The timeout to use in ms.
    pub transmit_timeout: Duration,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Error)]
pub enum TryFromCecCommandError {
    #[error("unknown opcode")]
    UnknownOpcode,
    #[error("unknown initiator")]
    UnknownInitiator,
    #[error("unknown destination")]
    UnknownDestination,
}

impl core::convert::TryFrom<cec_command> for CecCommand {
    type Error = TryFromCecCommandError;

    fn try_from(command: cec_command) -> std::result::Result<Self, Self::Error> {
        let opcode =
            CecOpcode::from_repr(command.opcode).ok_or(TryFromCecCommandError::UnknownOpcode)?;
        let initiator = CecLogicalAddress::from_repr(command.initiator)
            .ok_or(TryFromCecCommandError::UnknownInitiator)?;
        let destination = CecLogicalAddress::from_repr(command.destination)
            .ok_or(TryFromCecCommandError::UnknownDestination)?;
        let parameters = command.parameters.into();
        let transmit_timeout = Duration::from_millis(if command.transmit_timeout < 0 {
            0
        } else {
            command.transmit_timeout.try_into().unwrap()
        });
        Ok(CecCommand {
            initiator,
            destination,
            ack: command.ack != 0,
            eom: command.eom != 0,
            opcode,
            parameters,
            opcode_set: command.opcode_set != 0,
            transmit_timeout,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Error)]
pub enum TryFromCecLogMessageError {
    #[error("message parse error")]
    MessageParseError,
    #[error("log level parse error")]
    LogLevelParseError,
    #[error("timestamp parse error")]
    TimestampParseError,
    #[error("unknown log level")]
    UnknownLogLevel,
}

#[derive(Debug, Clone)]
pub struct CecLogMessage {
    /// The actual message.
    pub message: String,
    /// Log level of the message.
    pub level: CecLogLevel,
    /// Duration since connection was established.
    pub time: Duration,
}

impl core::convert::TryFrom<cec_log_message> for CecLogMessage {
    type Error = TryFromCecLogMessageError;

    fn try_from(log_message: cec_log_message) -> std::result::Result<Self, Self::Error> {
        let c_str: &CStr = unsafe { CStr::from_ptr(log_message.message) };
        let message = c_str
            .to_str()
            .map_err(|_| TryFromCecLogMessageError::MessageParseError)?
            .to_owned();
        let level = CecLogLevel::from_repr(log_message.level)
            .ok_or(TryFromCecLogMessageError::LogLevelParseError)?;
        let time = log_message
            .time
            .try_into()
            .map_err(|_| TryFromCecLogMessageError::TimestampParseError)?;

        Ok(CecLogMessage {
            message,
            level,
            time: Duration::from_millis(time),
        })
    }
}

impl fmt::Display for CecLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CecLogLevel::Error => write!(f, "Error"),
            CecLogLevel::Warning => write!(f, "Warning"),
            CecLogLevel::Notice => write!(f, "Notice"),
            CecLogLevel::Traffic => write!(f, "Traffic"),
            CecLogLevel::Debug => write!(f, "Debug"),
            CecLogLevel::All => write!(f, "All"),
        }
    }
}

/// Collection of logical addresses, with one primary address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecLogicalAddresses {
    pub primary: KnownCecLogicalAddress,
    pub addresses: HashSet<KnownAndRegisteredCecLogicalAddress>,
}

impl CecLogicalAddresses {
    pub fn with_only_primary(primary: &KnownCecLogicalAddress) -> CecLogicalAddresses {
        CecLogicalAddresses {
            primary: *primary,
            addresses: HashSet::new(),
        }
    }
    /// Create CecLogicalAddresses from primary address and secondary addresses
    ///
    /// # Arguments
    ///
    /// * `primary` - Primary address to use
    /// * `addresses` - other addresses to use. Primary is added to the set if not yet present
    ///
    /// Returns `None` in the following cases
    /// * when primary is `Unregistered` and `addresses` is non-empty
    ///
    pub fn with_primary_and_addresses(
        primary: &KnownCecLogicalAddress,
        addresses: &HashSet<KnownAndRegisteredCecLogicalAddress>,
    ) -> Option<CecLogicalAddresses> {
        match (*primary).into() {
            // Invalid: Primary must be set if there are addresses
            CecLogicalAddress::Unregistered if !addresses.is_empty() => None,
            // Empty
            CecLogicalAddress::Unregistered => Some(CecLogicalAddresses::default()),
            // Non-empty
            _ => {
                let mut cloned_addresses = addresses.clone();
                // Following cannot panic since primary is not representing Unregistered
                let registered_address: KnownAndRegisteredCecLogicalAddress =
                    (*primary).try_into().unwrap();
                // We ensure that addresses always contains the primary
                cloned_addresses.insert(registered_address);
                Some(CecLogicalAddresses {
                    primary: *primary,
                    addresses: cloned_addresses,
                })
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Error)]
pub enum TryFromCecLogicalAddressesError {
    #[error("unknown primary address")]
    UnknownPrimaryAddress,
    #[error("invalid primary address")]
    InvalidPrimaryAddress,
}

impl TryFrom<cec_logical_addresses> for CecLogicalAddresses {
    type Error = TryFromCecLogicalAddressesError;
    fn try_from(addresses: cec_logical_addresses) -> Result<Self, Self::Error> {
        let primary = CecLogicalAddress::from_repr(addresses.primary)
            .ok_or(TryFromCecLogicalAddressesError::InvalidPrimaryAddress)?;
        let primary = KnownCecLogicalAddress::new(primary)
            .ok_or(TryFromCecLogicalAddressesError::UnknownPrimaryAddress)?;

        let addresses = HashSet::from_iter(addresses.addresses.into_iter().enumerate().filter_map(
            |(logical_addr, addr_mask)| {
                let logical_addr = logical_addr as c_int;
                // If logical address x is in use, addresses.addresses[x] != 0.
                if addr_mask != 0 {
                    KnownAndRegisteredCecLogicalAddress::new(
                        CecLogicalAddress::try_from(logical_addr).unwrap(),
                    )
                } else {
                    None
                }
            },
        ));

        Ok(Self { primary, addresses })
    }
}

impl Default for CecLogicalAddresses {
    fn default() -> Self {
        CecLogicalAddresses {
            primary: KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered).unwrap(),
            addresses: HashSet::new(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CecKeypress {
    /// The keycode.
    pub keycode: CecUserControlCode,
    /// The duration of the keypress.
    pub duration: Duration,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Error)]
pub enum TryFromCecKeyPressError {
    #[error("unknown keycode")]
    UnknownKeycode,
}

impl core::convert::TryFrom<cec_keypress> for CecKeypress {
    type Error = TryFromCecKeyPressError;
    fn try_from(keypress: cec_keypress) -> std::result::Result<Self, Self::Error> {
        let keycode = CecUserControlCode::from_repr(keypress.keycode)
            .ok_or(TryFromCecKeyPressError::UnknownKeycode)?;
        Ok(CecKeypress {
            keycode,
            duration: Duration::from_millis(keypress.duration.into()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecDeviceTypeVec(pub ArrayVec<CecDeviceType, 5>);

impl CecDeviceTypeVec {
    pub fn new(type1: CecDeviceType) -> CecDeviceTypeVec {
        let mut inner = ArrayVec::<_, 5>::new();
        inner.push(type1);
        CecDeviceTypeVec(inner)
    }
}

#[derive(derive_more::Debug)]
struct CecCallbacks {
    #[debug(skip)]
    pub key_press_callback: Option<Box<dyn FnMut(CecKeypress) + Send>>,
    #[debug(skip)]
    pub command_received_callback: Option<Box<dyn FnMut(CecCommand) + Send>>,
    #[debug(skip)]
    pub log_message_callbacks: Option<Box<dyn FnMut(CecLogMessage) + Send>>,
    // pub onSourceActivated: FnSourceActivated,
}

pub type FnKeyPress = dyn FnMut(CecKeypress) + Send;
pub type FnCommand = dyn FnMut(CecCommand) + Send;
pub type FnLogMessage = dyn FnMut(CecLogMessage) + Send;
pub type FnSourceActivated = dyn FnMut(CecLogicalAddress, bool);

extern "C" fn key_press_callback(rust_callbacks: *mut c_void, keypress_raw: *const cec_keypress) {
    trace!("key_press_callback");
    let rust_callbacks: *mut CecCallbacks = rust_callbacks.cast();
    if let Some(rust_callbacks) = unsafe { rust_callbacks.as_mut() } {
        if let Some(keypress) = unsafe { keypress_raw.as_ref() } {
            trace!("CecCallbacks: keypress.keycode {:?}", keypress.keycode);
            if let Some(rust_callback) = &mut rust_callbacks.key_press_callback {
                if let Ok(keypress) = (*keypress).try_into() {
                    rust_callback(keypress);
                }
            }
        }
    }
}

extern "C" fn command_received_callback(
    rust_callbacks: *mut c_void,
    command_raw: *const cec_command,
) {
    trace!("command_received_callback");
    let rust_callbacks: *mut CecCallbacks = rust_callbacks.cast();
    if let Some(rust_callbacks) = unsafe { rust_callbacks.as_mut() } {
        if let Some(command) = unsafe { command_raw.as_ref() } {
            trace!(
                "command_received_callback: command.opcode {:?}",
                command.opcode
            );
            if let Some(rust_callback) = &mut rust_callbacks.command_received_callback {
                if let Ok(command) = (*command).try_into() {
                    rust_callback(command);
                }
            }
        }
    }
}

extern "C" fn log_message_callback(
    rust_callbacks: *mut c_void,
    log_message_raw: *const cec_log_message,
) {
    trace!("log_message_callback");
    let rust_callbacks: *mut CecCallbacks = rust_callbacks.cast();
    if let Some(rust_callbacks) = unsafe { rust_callbacks.as_mut() } {
        if let Some(log_message) = unsafe { log_message_raw.as_ref() } {
            if let Some(rust_callback) = &mut rust_callbacks.log_message_callbacks {
                if let Ok(log_message) = (*log_message).try_into() {
                    rust_callback(log_message);
                }
            }
        }
    }
}

static mut CALLBACKS: ICECCallbacks = ICECCallbacks {
    logMessage: Option::Some(log_message_callback),
    keyPress: Option::Some(key_press_callback),
    commandReceived: Option::Some(command_received_callback),
    configurationChanged: Option::None,
    alert: Option::None,
    menuStateChanged: Option::None,
    sourceActivated: Option::None,
};

#[derive(Builder, derive_more::Debug)]
#[builder(pattern = "owned")]
pub struct CecConnectionCfg {
    #[debug(skip)]
    #[builder(default, setter(strip_option), pattern = "owned")]
    pub key_press_callback: Option<Box<FnKeyPress>>,
    #[debug(skip)]
    #[builder(default, setter(strip_option), pattern = "owned")]
    pub command_received_callback: Option<Box<FnCommand>>,
    #[debug(skip)]
    #[builder(default, setter(strip_option), pattern = "owned")]
    pub log_message_callback: Option<Box<FnLogMessage>>,

    #[builder(default)]
    pub port: Option<String>,

    #[builder(default, setter(strip_option))]
    pub autodetect: Option<bool>,

    #[builder(default = "Duration::from_secs(5)")]
    pub open_timeout: Duration,

    //
    // cec_configuration items follow up
    //
    pub device_name: String,

    ///< the device type(s) to use on the CEC bus for libCEC.
    pub device_types: CecDeviceTypeVec,

    // optional cec_configuration items follow
    ///< the physical address of the CEC adapter.
    #[builder(default, setter(strip_option))]
    pub physical_address: Option<u16>,

    ///< the logical address of the device to which the adapter is connected. only used when iPhysicalAddress = 0 or when the adapter doesn't support autodetection.
    #[builder(default, setter(strip_option))]
    pub base_device: Option<CecLogicalAddress>,

    ///< the HDMI port to which the adapter is connected. only used when iPhysicalAddress = 0 or when the adapter doesn't support autodetection.
    #[builder(default, setter(strip_option))]
    pub hdmi_port: Option<u8>,

    ///< override the vendor ID of the TV. leave this untouched to autodetect.
    #[builder(default, setter(strip_option))]
    pub tv_vendor: Option<u32>,

    ///< list of devices to wake when initialising libCEC or when calling PowerOnDevices() without any parameter..
    #[builder(default, setter(strip_option))]
    pub wake_devices: Option<CecLogicalAddresses>,

    /// List of devices to power off when calling StandbyDevices() without any parameter.
    #[builder(default, setter(strip_option))]
    pub power_off_devices: Option<CecLogicalAddresses>,

    /// True to get the settings from the ROM (if set, and a v2 ROM is present), false to use these settings.
    #[builder(default, setter(strip_option))]
    pub get_settings_from_rom: Option<bool>,

    /// Make libCEC the active source on the bus when starting the player application.
    #[builder(default, setter(strip_option))]
    pub activate_source: Option<bool>,

    /// Put this PC in standby mode when the TV is switched off.
    /// Only used when `bShutdownOnStandby` = 0.
    #[builder(default, setter(strip_option))]
    pub power_off_on_standby: Option<bool>,

    /// The menu language used by the client. 3 character ISO 639-2 country code. see http://http://www.loc.gov/standards/iso639-2/ added in 1.6.2.
    #[builder(default, setter(strip_option))]
    pub device_language: Option<String>,

    /// Won't allocate a CCECClient when starting the connection when set (same as monitor mode). added in 1.6.3.
    #[builder(default, setter(strip_option))]
    pub monitor_only: Option<bool>,

    /// Type of the CEC adapter that we're connected to. added in 1.8.2.
    #[builder(default, setter(strip_option))]
    pub adapter_type: Option<CecAdapterType>,

    /// key code that initiates combo keys. defaults to CEC_USER_CONTROL_CODE_F1_BLUE. CEC_USER_CONTROL_CODE_UNKNOWN to disable. added in 2.0.5.
    #[builder(default, setter(strip_option))]
    pub combo_key: Option<CecUserControlCode>,

    /// Timeout until the combo key is sent as normal keypress.
    #[builder(default, setter(strip_option))]
    pub combo_key_timeout: Option<Duration>,

    /// Rate at which buttons autorepeat. 0 means rely on CEC device.
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub button_repeat_rate: Option<Duration>,

    /// Duration after last update until a button is considered released.
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub button_release_delay: Option<Duration>,

    /// Prevent double taps within this timeout. defaults to 200ms. added in 4.0.0.
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub double_tap_timeout: Option<Duration>,

    /// Set to 1 to automatically waking an AVR when the source is activated. added in 4.0.0.
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub autowake_avr: Option<bool>,
}

pub type CecConnectionResult<T> = result::Result<T, CecConnectionResultError>;

#[derive(Error, Debug)]
pub enum CecConnectionResultError {
    #[error("libcec initialization failed")]
    LibInitFailed,
    #[error("no adapter found")]
    NoAdapterFound,
    #[error("failed to open adapter")]
    AdapterOpenFailed,
    #[error("callback registration failed")]
    CallbackRegistrationFailed,
    #[error("transmit failed")]
    TransmitFailed,
    #[error("port missing")]
    PortMissing,
    #[error("ffi error: {0}")]
    FfiError(#[from] std::ffi::NulError),
}

#[derive(Debug)]
pub struct CecConnection(
    pub CecConnectionCfg,
    pub libcec_connection_t,
    Pin<Box<CecCallbacks>>,
);

unsafe impl Send for CecConnection {}

impl CecConnection {
    pub fn transmit(&self, command: CecCommand) -> CecConnectionResult<()> {
        if unsafe { libcec_transmit(self.1, &command.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }
    pub fn send_power_on_devices(&self, address: CecLogicalAddress) -> CecConnectionResult<()> {
        if unsafe { libcec_power_on_devices(self.1, address.repr()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }
    pub fn send_standby_devices(&self, address: CecLogicalAddress) -> CecConnectionResult<()> {
        if unsafe { libcec_standby_devices(self.1, address.repr()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn set_active_source(&self, device_type: CecDeviceType) -> CecConnectionResult<()> {
        if unsafe { libcec_set_active_source(self.1, device_type.repr()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn get_active_source(&self) -> CecLogicalAddress {
        let active_raw: cec_logical_address = unsafe { libcec_get_active_source(self.1) };
        CecLogicalAddress::from_repr(active_raw).unwrap()
    }

    pub fn is_active_source(&self, address: CecLogicalAddress) -> CecConnectionResult<()> {
        if unsafe { libcec_is_active_source(self.1, address.repr()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn get_device_power_status(&self, address: CecLogicalAddress) -> CecPowerStatus {
        let status_raw: cec_power_status =
            unsafe { libcec_get_device_power_status(self.1, address.repr()) };

        CecPowerStatus::from_repr(status_raw).unwrap()
    }

    pub fn send_keypress(
        &self,
        address: CecLogicalAddress,
        key: CecUserControlCode,
        wait: bool,
    ) -> CecConnectionResult<()> {
        if unsafe { libcec_send_keypress(self.1, address.repr(), key.repr(), wait.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn send_key_release(
        &self,
        address: CecLogicalAddress,
        wait: bool,
    ) -> CecConnectionResult<()> {
        if unsafe { libcec_send_key_release(self.1, address.repr(), wait.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn volume_up(&self, send_release: bool) -> CecConnectionResult<()> {
        if unsafe { libcec_volume_up(self.1, send_release.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn volume_down(&self, send_release: bool) -> CecConnectionResult<()> {
        if unsafe { libcec_volume_down(self.1, send_release.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn mute_audio(&self, send_release: bool) -> CecConnectionResult<()> {
        if unsafe { libcec_mute_audio(self.1, send_release.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn audio_toggle_mute(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_audio_toggle_mute(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn audio_mute(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_audio_mute(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn audio_unmute(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_audio_unmute(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn audio_get_status(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_audio_get_status(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn set_inactive_view(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_set_inactive_view(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn set_logical_address(&self, address: CecLogicalAddress) -> CecConnectionResult<()> {
        if unsafe { libcec_set_logical_address(self.1, address.repr()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn switch_monitoring(&self, enable: bool) -> CecConnectionResult<()> {
        if unsafe { libcec_switch_monitoring(self.1, enable.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn get_logical_addresses(
        &self,
    ) -> Result<CecLogicalAddresses, TryFromCecLogicalAddressesError> {
        CecLogicalAddresses::try_from(unsafe { libcec_get_logical_addresses(self.1) })
    }

    // Unimplemented:
    // extern DECLSPEC int libcec_set_physical_address(libcec_connection_t connection, uint16_t iPhysicalAddress);
    // extern DECLSPEC int libcec_set_deck_control_mode(libcec_connection_t connection, CEC_NAMESPACE cec_deck_control_mode mode, int bSendUpdate);
    // extern DECLSPEC int libcec_set_deck_info(libcec_connection_t connection, CEC_NAMESPACE cec_deck_info info, int bSendUpdate);
    // extern DECLSPEC int libcec_set_menu_state(libcec_connection_t connection, CEC_NAMESPACE cec_menu_state state, int bSendUpdate);
    // extern DECLSPEC int libcec_set_osd_string(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress, CEC_NAMESPACE cec_display_control duration, const char* strMessage);
    // extern DECLSPEC CEC_NAMESPACE cec_version libcec_get_device_cec_version(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress);
    // extern DECLSPEC int libcec_get_device_menu_language(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress, CEC_NAMESPACE cec_menu_language language);
    // extern DECLSPEC uint32_t libcec_get_device_vendor_id(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress);
    // extern DECLSPEC uint16_t libcec_get_device_physical_address(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress);
    // extern DECLSPEC int libcec_poll_device(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress);
    // extern DECLSPEC CEC_NAMESPACE cec_logical_addresses libcec_get_active_devices(libcec_connection_t connection);
    // extern DECLSPEC int libcec_is_active_device(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address address);
    // extern DECLSPEC int libcec_is_active_device_type(libcec_connection_t connection, CEC_NAMESPACE cec_device_type type);
    // extern DECLSPEC int libcec_set_hdmi_port(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address baseDevice, uint8_t iPort);
    // extern DECLSPEC int libcec_get_device_osd_name(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iAddress, CEC_NAMESPACE cec_osd_name name);
    // extern DECLSPEC int libcec_set_stream_path_logical(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iAddress);
    // extern DECLSPEC int libcec_set_stream_path_physical(libcec_connection_t connection, uint16_t iPhysicalAddress);
    // extern DECLSPEC int libcec_get_current_configuration(libcec_connection_t connection, CEC_NAMESPACE libcec_configuration* configuration);
    // extern DECLSPEC int libcec_can_persist_configuration(libcec_connection_t connection);
    // extern DECLSPEC int libcec_persist_configuration(libcec_connection_t connection, CEC_NAMESPACE libcec_configuration* configuration);
    // extern DECLSPEC int libcec_set_configuration(libcec_connection_t connection, const CEC_NAMESPACE libcec_configuration* configuration);
    // extern DECLSPEC void libcec_rescan_devices(libcec_connection_t connection);
    // extern DECLSPEC int libcec_is_libcec_active_source(libcec_connection_t connection);
    // extern DECLSPEC int libcec_get_device_information(libcec_connection_t connection, const char* strPort, CEC_NAMESPACE libcec_configuration* config, uint32_t iTimeoutMs);
    // extern DECLSPEC const char* libcec_get_lib_info(libcec_connection_t connection);
    // extern DECLSPEC void libcec_init_video_standalone(libcec_connection_t connection);
    // extern DECLSPEC uint16_t libcec_get_adapter_vendor_id(libcec_connection_t connection);
    // extern DECLSPEC uint16_t libcec_get_adapter_product_id(libcec_connection_t connection);
    // extern DECLSPEC int8_t libcec_detect_adapters(libcec_connection_t connection, CEC_NAMESPACE cec_adapter_descriptor* deviceList, uint8_t iBufSize, const char* strDevicePath, int bQuickScan);
}

impl CecConnectionCfg {
    /// Open connection to configuration represented by this object
    ///
    ///
    /// # Errors
    ///
    /// Error is returned in following cases
    /// - LibInitFailed: cec_sys::libcec_initialise fails
    /// - AdapterOpenFailed: cec_sys::libcec_open fails
    /// - CallbackRegistrationFailed: cec_sys::libcec_enable_callbacks fails
    ///
    /// # Panics
    ///
    /// Panics if self.port contains internal 0 byte
    pub fn open(mut self) -> CecConnectionResult<CecConnection> {
        let mut cfg: libcec_configuration = (&self).into();
        // Consume self.*_callback and build CecCallbacks from those
        let pinned_callbacks = Box::pin(CecCallbacks {
            key_press_callback: self.key_press_callback.take(),
            command_received_callback: self.command_received_callback.take(),
            log_message_callbacks: self.log_message_callback.take(),
        });
        let rust_callbacks_as_void_ptr = &*pinned_callbacks as *const _ as *mut _;
        let autodetect = self.autodetect.unwrap_or(false);
        let port = self.port.clone();
        let open_timeout = self.open_timeout.as_millis() as u32;

        let connection = CecConnection(
            self,
            unsafe { libcec_initialise(&mut cfg) },
            pinned_callbacks,
        );

        if connection.1.is_null() {
            return Err(CecConnectionResultError::LibInitFailed);
        }

        let resolved_port = match autodetect {
            true => match Self::detect_port(&connection) {
                Ok(x) => x,
                Err(e) => return Err(e),
            },
            false => match port {
                Some(x) => CString::new(x)?,
                None => return Err(CecConnectionResultError::PortMissing),
            },
        };

        if unsafe { libcec_open(connection.1, resolved_port.as_ptr(), open_timeout) } == 0 {
            return Err(CecConnectionResultError::AdapterOpenFailed);
        }

        #[cfg(abi4)]
        let callback_ret = unsafe {
            cec_sys::libcec_enable_callbacks(
                connection.1,
                rust_callbacks_as_void_ptr,
                &mut CALLBACKS,
            )
        };
        #[cfg(not(abi4))]
        let callback_ret = unsafe {
            cec_sys::libcec_set_callbacks(connection.1, &mut CALLBACKS, rust_callbacks_as_void_ptr)
        };
        if callback_ret == 0 {
            return Err(CecConnectionResultError::CallbackRegistrationFailed);
        }

        Ok(connection)
    }

    fn detect_port(connection: &CecConnection) -> CecConnectionResult<CString> {
        let mut devices: [cec_sys::cec_adapter_descriptor; 10] = unsafe { std::mem::zeroed() };
        let num_devices = unsafe {
            cec_sys::libcec_detect_adapters(
                connection.1,
                &mut devices as _,
                10,
                std::ptr::null(),
                true as i32,
            )
        };

        if num_devices < 0 {
            Err(CecConnectionResultError::NoAdapterFound)
        } else {
            let port = devices[0]
                .strComName
                .into_iter()
                .flat_map(u8::try_from)
                .filter(|x| *x != 0)
                .collect::<Vec<u8>>();
            Ok(CString::new(port)?)
        }
    }
}

impl Drop for CecConnection {
    fn drop(&mut self) {
        unsafe {
            libcec_close(self.1);
            libcec_destroy(self.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_version() {
        assert_eq!(CEC_LIB_VERSION_MAJOR, 6);
    }

    mod utils {
        use super::*;

        #[allow(clippy::unnecessary_cast)]
        #[test]
        fn test_first_3() {
            assert_eq!(
                [b's' as _, b'a' as _, b'm' as _] as [::std::os::raw::c_char; 3],
                first_n::<3>("sample")
            );
            assert_eq!(
                [b's' as _, b'a' as _, 0 as _] as [::std::os::raw::c_char; 3],
                first_n::<3>("sa")
            );
            assert_eq!(
                [0 as _, 0 as _, 0 as _] as [::std::os::raw::c_char; 3],
                first_n::<3>("")
            );
        }

        #[allow(clippy::unnecessary_cast)]
        #[test]
        fn test_first_7() {
            assert_eq!(
                [b's' as _, b'a' as _, b'm' as _, b'p' as _, b'l' as _, b'e' as _, 0]
                    as [::std::os::raw::c_char; 7],
                first_n::<7>("sample")
            );
        }
        #[test]
        fn test_first_0() {
            assert_eq!([] as [::std::os::raw::c_char; 0], first_n::<0>("sample"));
        }
    }

    #[cfg(test)]
    mod address {
        use super::*;

        #[test]
        fn test_known_address() {
            assert_eq!(
                Some(KnownCecLogicalAddress(CecLogicalAddress::Audiosystem)),
                KnownCecLogicalAddress::new(CecLogicalAddress::Audiosystem)
            );
            assert_eq!(
                Some(KnownCecLogicalAddress(CecLogicalAddress::Unregistered)),
                KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered)
            );
            assert_eq!(
                None,
                KnownCecLogicalAddress::new(CecLogicalAddress::Unknown)
            );
        }

        #[test]
        fn test_known_and_registered_address() {
            assert_eq!(
                Some(KnownAndRegisteredCecLogicalAddress(
                    CecLogicalAddress::Audiosystem
                )),
                KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Audiosystem)
            );
            assert_eq!(
                None,
                KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Unregistered)
            );
            assert_eq!(
                None,
                KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Unknown)
            );
        }

        #[test]
        fn test_to_ffi_no_address() {
            let ffi_addresses: cec_logical_addresses = CecLogicalAddresses::default().into();
            assert_eq!(
                ffi_addresses.primary,
                CecLogicalAddress::Unregistered.repr()
            );
            assert_eq!(ffi_addresses.addresses, [0; 16]);

            // try converting back
            let rust_addresses = CecLogicalAddresses::try_from(ffi_addresses).unwrap();
            assert_eq!(
                rust_addresses.primary,
                KnownCecLogicalAddress(CecLogicalAddress::Unregistered)
            );
            assert!(rust_addresses.addresses.is_empty());
        }

        #[test]
        fn test_to_ffi_one_address() {
            let ffi_addresses: cec_logical_addresses = CecLogicalAddresses::with_only_primary(
                &KnownCecLogicalAddress::new(CecLogicalAddress::Playbackdevice1).unwrap(),
            )
            .into();
            assert_eq!(
                ffi_addresses.primary,
                CecLogicalAddress::Playbackdevice1.repr()
            );
            // addresses mask should be all zeros
            assert_eq!(ffi_addresses.addresses, [0; 16]);

            // try converting back
            let rust_addresses = CecLogicalAddresses::try_from(ffi_addresses).unwrap();
            assert_eq!(
                rust_addresses.primary,
                KnownCecLogicalAddress(CecLogicalAddress::Playbackdevice1)
            );
            assert!(rust_addresses.addresses.is_empty());
        }

        #[test]
        fn test_to_ffi_three_address() {
            let mut others = HashSet::new();
            others.insert(
                KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Playbackdevice2)
                    .unwrap(),
            );
            others.insert(
                KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Audiosystem).unwrap(),
            );

            let non_ffi = CecLogicalAddresses::with_primary_and_addresses(
                &KnownCecLogicalAddress::new(CecLogicalAddress::Playbackdevice1).unwrap(),
                &others,
            )
            .unwrap();

            let ffi_addresses: cec_logical_addresses = non_ffi.clone().into();

            assert_eq!(
                ffi_addresses.primary,
                CecLogicalAddress::Playbackdevice1.repr()
            );
            let ffi_secondary = ffi_addresses.addresses;
            const PRIMARY_INDEX: usize = CecLogicalAddress::Playbackdevice1 as usize;
            const PLAYBACKDEVICE2_INDEX: usize = CecLogicalAddress::Playbackdevice2 as usize;
            const AUDIOSYSTEM_INDEX: usize = CecLogicalAddress::Audiosystem as usize;
            for (mask_index, mask_value) in ffi_secondary.iter().enumerate() {
                match mask_index {
                    // Note: also the primary address is in the mask even though it was not provided originally
                    PLAYBACKDEVICE2_INDEX | AUDIOSYSTEM_INDEX | PRIMARY_INDEX => {
                        assert_eq!(
                            1, *mask_value,
                            "index {}, non-ffi addresses {:?}, ffi addresses {:?}",
                            mask_index, non_ffi, ffi_addresses
                        )
                    }
                    _ => assert_eq!(0, *mask_value),
                }
            }

            // try converting back
            let rust_addresses = CecLogicalAddresses::try_from(ffi_addresses).unwrap();
            assert_eq!(rust_addresses.primary, non_ffi.primary);
            assert_eq!(rust_addresses.addresses, non_ffi.addresses);
        }

        #[test]
        fn test_unregistered_primary_no_others() {
            let expected = Some(CecLogicalAddresses::with_only_primary(
                &KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered).unwrap(),
            ));
            assert_eq!(
                expected,
                CecLogicalAddresses::with_primary_and_addresses(
                    &KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered).unwrap(),
                    &HashSet::new(),
                )
            );
        }

        #[test]
        fn test_unregistered_primary_some_others() {
            let mut others = HashSet::new();
            others.insert(
                KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Audiosystem).unwrap(),
            );
            // If there are others, there should be also primary
            assert_eq!(
                None,
                CecLogicalAddresses::with_primary_and_addresses(
                    &KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered).unwrap(),
                    &others,
                )
            );
        }
    }

    #[cfg(test)]
    mod data_packet {
        use super::*;

        /// Assert that
        /// 1) sizes match
        /// 2) and that the elements of CecDatapacket match the first elements of packet2
        fn assert_eq_packet(packet: CecDatapacket, packet2: cec_datapacket) {
            assert_eq!(packet.0.len(), packet2.size.into());
            assert!(packet
                .0
                .as_slice()
                .iter()
                .eq(packet2.data[..(packet2.size as usize)].iter()));
        }

        fn assert_eq_ffi_packet(packet: cec_datapacket, packet2: cec_datapacket) {
            assert_eq!(packet.size, packet2.size);
            assert!(&packet.data.iter().eq(packet2.data.iter()));
        }

        #[test]
        fn test_from_ffi_full_size() {
            let mut data_buffer = [50; 64];
            data_buffer[0] = 5;
            data_buffer[1] = 7;
            data_buffer[3] = 99;
            let ffi_packet = cec_datapacket {
                data: data_buffer,
                size: 64,
            };
            let packet: CecDatapacket = ffi_packet.into();
            assert_eq_packet(packet, ffi_packet);
        }

        #[test]
        fn test_from_ffi_not_full() {
            let mut data_buffer = [50; 64];
            data_buffer[0] = 5;
            data_buffer[1] = 7;
            data_buffer[3] = 99;
            let ffi_packet = cec_datapacket {
                data: data_buffer,
                size: 3,
            };
            let packet: CecDatapacket = ffi_packet.into();
            assert_eq!(packet.0.as_slice(), &[5, 7, 50]);
        }

        #[test]
        fn test_to_ffi_not_full() {
            let mut a = ArrayVec::new();
            a.push(2);
            a.push(50);
            let packet = CecDatapacket(a);
            let ffi_packet: cec_datapacket = packet.into();
            let mut expected = cec_datapacket {
                size: 2,
                data: [0; 64],
            };
            expected.data[0] = 2;
            expected.data[1] = 50;
            assert_eq_ffi_packet(ffi_packet, expected);
        }

        #[test]
        fn test_to_ffi_full() {
            let mut a = ArrayVec::from([99; 64]);
            a.as_mut_slice()[1] = 50;
            let packet = CecDatapacket(a);
            let ffi_packet: cec_datapacket = packet.into();
            let mut expected = cec_datapacket {
                size: 64,
                data: [99; 64],
            };
            expected.data[1] = 50;
            assert_eq_ffi_packet(ffi_packet, expected);
        }
    }

    #[cfg(test)]
    mod command {
        use super::*;

        fn assert_eq_ffi_packet(packet: cec_datapacket, packet2: cec_datapacket) {
            assert_eq!(packet.size, packet2.size);
            assert!(&packet.data.iter().eq(packet2.data.iter()));
        }

        fn assert_eq_ffi_command(actual: cec_command, expected: cec_command) {
            assert_eq!(actual.ack, expected.ack);
            assert_eq!(actual.destination, expected.destination);
            assert_eq!(actual.eom, expected.eom);
            assert_eq!(actual.initiator, expected.initiator);
            assert_eq!(actual.opcode, expected.opcode);
            assert_eq!(actual.opcode_set, expected.opcode_set);
            assert_eq_ffi_packet(actual.parameters, expected.parameters);
            assert_eq!(actual.transmit_timeout, expected.transmit_timeout);
        }

        fn assert_eq_command(actual: CecCommand, expected: CecCommand) {
            assert_eq!(actual.ack, expected.ack);
            assert_eq!(actual.destination, expected.destination);
            assert_eq!(actual.eom, expected.eom);
            assert_eq!(actual.initiator, expected.initiator);
            assert_eq!(actual.opcode, expected.opcode);
            assert_eq!(actual.opcode_set, expected.opcode_set);
            assert_eq!(actual.parameters.0, expected.parameters.0);
            assert_eq!(actual.transmit_timeout, expected.transmit_timeout);
        }

        #[test]
        fn test_to_ffi() {
            let mut parameters = ArrayVec::new();
            parameters.push(2);
            parameters.push(3);
            let command = CecCommand {
                opcode: CecOpcode::ClearAnalogueTimer,
                initiator: CecLogicalAddress::Playbackdevice1,
                destination: CecLogicalAddress::Playbackdevice2,
                parameters: CecDatapacket(parameters.clone()),
                transmit_timeout: Duration::from_secs(65),
                ack: false,
                eom: true,
                opcode_set: true,
            };
            let ffi_command: cec_command = command.into();
            assert_eq_ffi_command(
                ffi_command,
                cec_command {
                    ack: 0,
                    destination: CecLogicalAddress::Playbackdevice2.repr(),
                    eom: 1,
                    initiator: CecLogicalAddress::Playbackdevice1.repr(),
                    opcode: CecOpcode::ClearAnalogueTimer.repr(),
                    opcode_set: 1,
                    parameters: CecDatapacket(parameters).into(), // OK to use here, verified in CecDatapacket unit tests
                    transmit_timeout: 65_000,
                },
            )
        }

        #[test]
        fn test_from_ffi() {
            let mut parameters = ArrayVec::new();
            parameters.push(2);
            parameters.push(3);
            let ffi_command = cec_command {
                ack: 0,
                destination: CecLogicalAddress::Playbackdevice2.repr(),
                eom: 1,
                initiator: CecLogicalAddress::Playbackdevice1.repr(),
                opcode: CecOpcode::ClearAnalogueTimer.repr(),
                opcode_set: 1,
                parameters: CecDatapacket(parameters.clone()).into(), // OK to use here, verified in CecDatapacket unit tests
                transmit_timeout: 65_000,
            };
            let command: CecCommand = ffi_command.try_into().unwrap();
            assert_eq_command(
                command,
                CecCommand {
                    ack: false,
                    destination: CecLogicalAddress::Playbackdevice2,
                    eom: true,
                    initiator: CecLogicalAddress::Playbackdevice1,
                    opcode: CecOpcode::ClearAnalogueTimer,
                    opcode_set: true,
                    parameters: CecDatapacket(parameters),
                    transmit_timeout: Duration::from_millis(65000),
                },
            )
        }
    }

    #[cfg(test)]
    mod device {
        use super::*;

        #[test]
        fn test_to_ffi_empty() {
            let devices = ArrayVec::new();
            let ffi_devices: cec_device_type_list = CecDeviceTypeVec(devices).into();
            assert_eq!(ffi_devices.types, [CecDeviceType::Reserved.repr(); 5]);
        }

        #[test]
        fn test_to_ffi_two_devices() {
            let mut devices = ArrayVec::new();
            devices.push(CecDeviceType::PlaybackDevice);
            devices.push(CecDeviceType::RecordingDevice);
            let ffi_devices: cec_device_type_list = CecDeviceTypeVec(devices).into();
            assert_eq!(ffi_devices.types[0], CecDeviceType::PlaybackDevice.repr());
            assert_eq!(ffi_devices.types[1], CecDeviceType::RecordingDevice.repr());
            assert_eq!(ffi_devices.types[2..], [CecDeviceType::Reserved.repr(); 3]);
        }
    }

    #[cfg(test)]
    mod keypress {
        use super::*;

        #[test]
        fn test_keypress_from_ffi_known_code() {
            let keypress: CecKeypress = cec_keypress {
                keycode: cec_user_control_code::CEC_USER_CONTROL_CODE_UP,
                duration: 300,
            }
            .try_into()
            .unwrap();
            assert_eq!(keypress.keycode, CecUserControlCode::Up);
            assert_eq!(keypress.duration, Duration::from_millis(300));
        }

        #[test]
        fn test_keypress_from_ffi_unknown_code() {
            let keypress: Result<CecKeypress, TryFromCecKeyPressError> = cec_keypress {
                keycode: unsafe { std::mem::transmute::<i32, cec_user_control_code>(666) },
                duration: 300,
            }
            .try_into();
            assert_eq!(keypress, Err(TryFromCecKeyPressError::UnknownKeycode));
        }
    }
}
