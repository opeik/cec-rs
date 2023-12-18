use std::ffi::c_int;

use cec_sys::*;
use enum_repr::EnumRepr;

use crate::TryFromLogicalAddressesError;

#[EnumRepr(type = "cec_abort_reason")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AbortReason {
    UnrecognizedOpcode = cec_abort_reason::CEC_ABORT_REASON_UNRECOGNIZED_OPCODE,
    NotInCorrectModeToRespond = cec_abort_reason::CEC_ABORT_REASON_NOT_IN_CORRECT_MODE_TO_RESPOND,
    CannotProvideSource = cec_abort_reason::CEC_ABORT_REASON_CANNOT_PROVIDE_SOURCE,
    InvalidOperand = cec_abort_reason::CEC_ABORT_REASON_INVALID_OPERAND,
    Refused = cec_abort_reason::CEC_ABORT_REASON_REFUSED,
}

#[EnumRepr(type = "cec_analogue_broadcast_type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AnalogueBroadcastType {
    Cable = cec_analogue_broadcast_type::CEC_ANALOGUE_BROADCAST_TYPE_CABLE,
    Satellite = cec_analogue_broadcast_type::CEC_ANALOGUE_BROADCAST_TYPE_SATELLITE,
    Terrestial = cec_analogue_broadcast_type::CEC_ANALOGUE_BROADCAST_TYPE_TERRESTIAL,
}

#[EnumRepr(type = "cec_audio_rate")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AudioRate {
    RateControlOff = cec_audio_rate::CEC_AUDIO_RATE_RATE_CONTROL_OFF,
    StandardRate100 = cec_audio_rate::CEC_AUDIO_RATE_STANDARD_RATE_100,
    FastRateMax101 = cec_audio_rate::CEC_AUDIO_RATE_FAST_RATE_MAX_101,
    SlowRateMin99 = cec_audio_rate::CEC_AUDIO_RATE_SLOW_RATE_MIN_99,
    StandardRate1000 = cec_audio_rate::CEC_AUDIO_RATE_STANDARD_RATE_100_0,
    FastRateMax1001 = cec_audio_rate::CEC_AUDIO_RATE_FAST_RATE_MAX_100_1,
    SlowRateMin999 = cec_audio_rate::CEC_AUDIO_RATE_SLOW_RATE_MIN_99_9,
}

#[EnumRepr(type = "cec_audio_status")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AudioStatus {
    MuteStatusMask = cec_audio_status::CEC_AUDIO_MUTE_STATUS_MASK,
    VolumeStatusMask = cec_audio_status::CEC_AUDIO_VOLUME_STATUS_MASK,
    VolumeMin = cec_audio_status::CEC_AUDIO_VOLUME_MIN,
    VolumeMax = cec_audio_status::CEC_AUDIO_VOLUME_MAX,
}

#[EnumRepr(type = "cec_version")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Version {
    VersionUnknown = cec_version::CEC_VERSION_UNKNOWN,
    Version12 = cec_version::CEC_VERSION_1_2,
    Version12a = cec_version::CEC_VERSION_1_2A,
    Version13 = cec_version::CEC_VERSION_1_3,
    Version13a = cec_version::CEC_VERSION_1_3A,
    Version14 = cec_version::CEC_VERSION_1_4,
    Version20 = cec_version::CEC_VERSION_2_0,
}

#[EnumRepr(type = "cec_channel_identifier")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ChannelIdentifier {
    CecChannelNumberFormatMask = cec_channel_identifier::CEC_CHANNEL_NUMBER_FORMAT_MASK,
    Cec1PartChannelNumber = cec_channel_identifier::CEC_1_PART_CHANNEL_NUMBER,
    Cec2PartChannelNumber = cec_channel_identifier::CEC_2_PART_CHANNEL_NUMBER,
    CecMajorChannelNumberMask = cec_channel_identifier::CEC_MAJOR_CHANNEL_NUMBER_MASK,
    CecMinorChannelNumberMask = cec_channel_identifier::CEC_MINOR_CHANNEL_NUMBER_MASK,
}

#[EnumRepr(type = "cec_deck_control_mode")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeckControlMode {
    SkipForwardWind = cec_deck_control_mode::CEC_DECK_CONTROL_MODE_SKIP_FORWARD_WIND,
    SkipReverseRewind = cec_deck_control_mode::CEC_DECK_CONTROL_MODE_SKIP_REVERSE_REWIND,
    Stop = cec_deck_control_mode::CEC_DECK_CONTROL_MODE_STOP,
    Eject = cec_deck_control_mode::CEC_DECK_CONTROL_MODE_EJECT,
}

#[EnumRepr(type = "cec_deck_info")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeckInfo {
    Play = cec_deck_info::CEC_DECK_INFO_PLAY,
    Record = cec_deck_info::CEC_DECK_INFO_RECORD,
    PlayReverse = cec_deck_info::CEC_DECK_INFO_PLAY_REVERSE,
    Still = cec_deck_info::CEC_DECK_INFO_STILL,
    Slow = cec_deck_info::CEC_DECK_INFO_SLOW,
    SlowReverse = cec_deck_info::CEC_DECK_INFO_SLOW_REVERSE,
    FastForward = cec_deck_info::CEC_DECK_INFO_FAST_FORWARD,
    FastReverse = cec_deck_info::CEC_DECK_INFO_FAST_REVERSE,
    NoMedia = cec_deck_info::CEC_DECK_INFO_NO_MEDIA,
    Stop = cec_deck_info::CEC_DECK_INFO_STOP,
    SkipForwardWind = cec_deck_info::CEC_DECK_INFO_SKIP_FORWARD_WIND,
    SkipReverseRewind = cec_deck_info::CEC_DECK_INFO_SKIP_REVERSE_REWIND,
    IndexSearchForward = cec_deck_info::CEC_DECK_INFO_INDEX_SEARCH_FORWARD,
    IndexSearchReverse = cec_deck_info::CEC_DECK_INFO_INDEX_SEARCH_REVERSE,
    OtherStatus = cec_deck_info::CEC_DECK_INFO_OTHER_STATUS,
    OtherStatusLg = cec_deck_info::CEC_DECK_INFO_OTHER_STATUS_LG,
}

#[EnumRepr(type = "cec_device_type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Tv = cec_device_type::CEC_DEVICE_TYPE_TV,
    RecordingDevice = cec_device_type::CEC_DEVICE_TYPE_RECORDING_DEVICE,
    Reserved = cec_device_type::CEC_DEVICE_TYPE_RESERVED,
    Tuner = cec_device_type::CEC_DEVICE_TYPE_TUNER,
    PlaybackDevice = cec_device_type::CEC_DEVICE_TYPE_PLAYBACK_DEVICE,
    AudioSystem = cec_device_type::CEC_DEVICE_TYPE_AUDIO_SYSTEM,
}

#[EnumRepr(type = "cec_display_control")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DisplayControl {
    DisplayForDefaultTime = cec_display_control::CEC_DISPLAY_CONTROL_DISPLAY_FOR_DEFAULT_TIME,
    DisplayUntilCleared = cec_display_control::CEC_DISPLAY_CONTROL_DISPLAY_UNTIL_CLEARED,
    ClearPreviousMessage = cec_display_control::CEC_DISPLAY_CONTROL_CLEAR_PREVIOUS_MESSAGE,
    ReservedForFutureUse = cec_display_control::CEC_DISPLAY_CONTROL_RESERVED_FOR_FUTURE_USE,
}

#[EnumRepr(type = "cec_external_source_specifier")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExternalSourceSpecifier {
    Plug = cec_external_source_specifier::CEC_EXTERNAL_SOURCE_SPECIFIER_EXTERNAL_PLUG,
    PhysicalAddress =
        cec_external_source_specifier::CEC_EXTERNAL_SOURCE_SPECIFIER_EXTERNAL_PHYSICAL_ADDRESS,
}

#[EnumRepr(type = "cec_menu_request_type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MenuRequestType {
    Activate = cec_menu_request_type::CEC_MENU_REQUEST_TYPE_ACTIVATE,
    Deactivate = cec_menu_request_type::CEC_MENU_REQUEST_TYPE_DEACTIVATE,
    Query = cec_menu_request_type::CEC_MENU_REQUEST_TYPE_QUERY,
}

#[EnumRepr(type = "cec_menu_state")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    Activated = cec_menu_state::CEC_MENU_STATE_ACTIVATED,
    Deactivated = cec_menu_state::CEC_MENU_STATE_DEACTIVATED,
}

#[EnumRepr(type = "cec_play_mode")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PlayMode {
    PlayForward = cec_play_mode::CEC_PLAY_MODE_PLAY_FORWARD,
    PlayReverse = cec_play_mode::CEC_PLAY_MODE_PLAY_REVERSE,
    PlayStill = cec_play_mode::CEC_PLAY_MODE_PLAY_STILL,
    FastForwardMinSpeed = cec_play_mode::CEC_PLAY_MODE_FAST_FORWARD_MIN_SPEED,
    FastForwardMediumSpeed = cec_play_mode::CEC_PLAY_MODE_FAST_FORWARD_MEDIUM_SPEED,
    FastForwardMaxSpeed = cec_play_mode::CEC_PLAY_MODE_FAST_FORWARD_MAX_SPEED,
    FastReverseMinSpeed = cec_play_mode::CEC_PLAY_MODE_FAST_REVERSE_MIN_SPEED,
    FastReverseMediumSpeed = cec_play_mode::CEC_PLAY_MODE_FAST_REVERSE_MEDIUM_SPEED,
    FastReverseMaxSpeed = cec_play_mode::CEC_PLAY_MODE_FAST_REVERSE_MAX_SPEED,
    SlowForwardMinSpeed = cec_play_mode::CEC_PLAY_MODE_SLOW_FORWARD_MIN_SPEED,
    SlowForwardMediumSpeed = cec_play_mode::CEC_PLAY_MODE_SLOW_FORWARD_MEDIUM_SPEED,
    SlowForwardMaxSpeed = cec_play_mode::CEC_PLAY_MODE_SLOW_FORWARD_MAX_SPEED,
    SlowReverseMinSpeed = cec_play_mode::CEC_PLAY_MODE_SLOW_REVERSE_MIN_SPEED,
    SlowReverseMediumSpeed = cec_play_mode::CEC_PLAY_MODE_SLOW_REVERSE_MEDIUM_SPEED,
    SlowReverseMaxSpeed = cec_play_mode::CEC_PLAY_MODE_SLOW_REVERSE_MAX_SPEED,
}

#[EnumRepr(type = "cec_power_status")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PowerStatus {
    On = cec_power_status::CEC_POWER_STATUS_ON,
    Standby = cec_power_status::CEC_POWER_STATUS_STANDBY,
    InTransitionStandbyToOn = cec_power_status::CEC_POWER_STATUS_IN_TRANSITION_STANDBY_TO_ON,
    InTransitionOnToStandby = cec_power_status::CEC_POWER_STATUS_IN_TRANSITION_ON_TO_STANDBY,
    Unknown = cec_power_status::CEC_POWER_STATUS_UNKNOWN,
}

#[EnumRepr(type = "cec_record_source_type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RecordSourceType {
    OwnSource = cec_record_source_type::CEC_RECORD_SOURCE_TYPE_OWN_SOURCE,
    DigitalService = cec_record_source_type::CEC_RECORD_SOURCE_TYPE_DIGITAL_SERVICE,
    AnalogueService = cec_record_source_type::CEC_RECORD_SOURCE_TYPE_ANALOGUE_SERVICE,
    ExternalPlus = cec_record_source_type::CEC_RECORD_SOURCE_TYPE_EXTERNAL_PLUS,
    ExternalPhysicalAddress =
        cec_record_source_type::CEC_RECORD_SOURCE_TYPE_EXTERNAL_PHYSICAL_ADDRESS,
}

#[EnumRepr(type = "cec_record_status_info")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RecordStatusInfo {
    RecordingCurrentlySelectedSource =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_RECORDING_CURRENTLY_SELECTED_SOURCE,
    RecordingDigitalService =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_RECORDING_DIGITAL_SERVICE,
    RecordingAnalogueService =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_RECORDING_ANALOGUE_SERVICE,
    RecordingExternalInput =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_RECORDING_EXTERNAL_INPUT,
    NoRecordingUnableToRecordDigitalService = cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_UNABLE_TO_RECORD_DIGITAL_SERVICE,
    NoRecordingUnableToRecordAnalogueService = cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_UNABLE_TO_RECORD_ANALOGUE_SERVICE,
    NoRecordingUnableToSelectRequiredService = cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_UNABLE_TO_SELECT_REQUIRED_SERVICE,
    NoRecordingInvalidExternalPlugNumber =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_INVALID_EXTERNAL_PLUG_NUMBER,
    NoRecordingInvalidExternalAddress =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_INVALID_EXTERNAL_ADDRESS,
    NoRecordingCaSystemNotSupported =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_CA_SYSTEM_NOT_SUPPORTED,
    NoRecordingNoOrInsufficientEntitlements =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_NO_OR_INSUFFICIENT_ENTITLEMENTS,
    NoRecordingNotAllowedToCopySource =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_NOT_ALLOWED_TO_COPY_SOURCE,
    NoRecordingNoFurtherCopiesAllowed =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_NO_FURTHER_COPIES_ALLOWED,
    NoRecordingNoMedia = cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_NO_MEDIA,
    NoRecordingPlaying = cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_PLAYING,
    NoRecordingAlreadyRecording =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_ALREADY_RECORDING,
    NoRecordingMediaProtected =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_MEDIA_PROTECTED,
    NoRecordingNoSourceSignal =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_NO_SOURCE_SIGNAL,
    NoRecordingMediaProblem =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_MEDIA_PROBLEM,
    NoRecordingNotEnoughSpaceAvailable =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_NOT_ENOUGH_SPACE_AVAILABLE,
    NoRecordingParentalLockOn =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_PARENTAL_LOCK_ON,
    RecordingTerminatedNormally =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_RECORDING_TERMINATED_NORMALLY,
    RecordingHasAlreadyTerminated =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_RECORDING_HAS_ALREADY_TERMINATED,
    NoRecordingOtherReason =
        cec_record_status_info::CEC_RECORD_STATUS_INFO_NO_RECORDING_OTHER_REASON,
}

#[EnumRepr(type = "cec_recording_sequence")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RecordingSequence {
    Sunday = cec_recording_sequence::CEC_RECORDING_SEQUENCE_SUNDAY,
    Monday = cec_recording_sequence::CEC_RECORDING_SEQUENCE_MONDAY,
    Tuesday = cec_recording_sequence::CEC_RECORDING_SEQUENCE_TUESDAY,
    Wednesday = cec_recording_sequence::CEC_RECORDING_SEQUENCE_WEDNESDAY,
    Thursday = cec_recording_sequence::CEC_RECORDING_SEQUENCE_THURSDAY,
    Friday = cec_recording_sequence::CEC_RECORDING_SEQUENCE_FRIDAY,
    Saturday = cec_recording_sequence::CEC_RECORDING_SEQUENCE_SATURDAY,
    OnceOnly = cec_recording_sequence::CEC_RECORDING_SEQUENCE_ONCE_ONLY,
}

#[EnumRepr(type = "cec_status_request")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StatusRequest {
    On = cec_status_request::CEC_STATUS_REQUEST_ON,
    Off = cec_status_request::CEC_STATUS_REQUEST_OFF,
    Once = cec_status_request::CEC_STATUS_REQUEST_ONCE,
}

#[EnumRepr(type = "cec_system_audio_status")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SystemAudioStatus {
    Off = cec_system_audio_status::CEC_SYSTEM_AUDIO_STATUS_OFF,
    On = cec_system_audio_status::CEC_SYSTEM_AUDIO_STATUS_ON,
}

#[EnumRepr(type = "cec_timer_cleared_status_data")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TimerClearedStatusData {
    NotClearedRecording =
        cec_timer_cleared_status_data::CEC_TIMER_CLEARED_STATUS_DATA_TIMER_NOT_CLEARED_RECORDING,
    NotClearedNoMatching =
        cec_timer_cleared_status_data::CEC_TIMER_CLEARED_STATUS_DATA_TIMER_NOT_CLEARED_NO_MATCHING,
    NotClearedNoInf0Available = cec_timer_cleared_status_data::CEC_TIMER_CLEARED_STATUS_DATA_TIMER_NOT_CLEARED_NO_INF0_AVAILABLE,
    Cleared = cec_timer_cleared_status_data::CEC_TIMER_CLEARED_STATUS_DATA_TIMER_CLEARED,
}

#[EnumRepr(type = "cec_timer_overlap_warning")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TimerOverlapWarning {
    NoOverlap = cec_timer_overlap_warning::CEC_TIMER_OVERLAP_WARNING_NO_OVERLAP,
    TimerBlocksOverlap = cec_timer_overlap_warning::CEC_TIMER_OVERLAP_WARNING_TIMER_BLOCKS_OVERLAP,
}

#[EnumRepr(type = "cec_media_info")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MediaInfo {
    MediaPresentAndNotProtected = cec_media_info::CEC_MEDIA_INFO_MEDIA_PRESENT_AND_NOT_PROTECTED,
    MediaPresentButProtected = cec_media_info::CEC_MEDIA_INFO_MEDIA_PRESENT_BUT_PROTECTED,
    MediaNotPresent = cec_media_info::CEC_MEDIA_INFO_MEDIA_NOT_PRESENT,
    FutureUse = cec_media_info::CEC_MEDIA_INFO_FUTURE_USE,
}

#[EnumRepr(type = "cec_programmed_indicator")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ProgrammedIndicator {
    NotProgrammed = cec_programmed_indicator::CEC_PROGRAMMED_INDICATOR_NOT_PROGRAMMED,
    Programmed = cec_programmed_indicator::CEC_PROGRAMMED_INDICATOR_PROGRAMMED,
}

#[EnumRepr(type = "cec_programmed_info")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ProgrammedInfo {
    FutureUse = cec_programmed_info::CEC_PROGRAMMED_INFO_FUTURE_USE,
    EnoughSpaceAvailableForRecording =
        cec_programmed_info::CEC_PROGRAMMED_INFO_ENOUGH_SPACE_AVAILABLE_FOR_RECORDING,
    NotEnoughSpaceAvailableForRecording =
        cec_programmed_info::CEC_PROGRAMMED_INFO_NOT_ENOUGH_SPACE_AVAILABLE_FOR_RECORDING,
    MayNotBeEnoughSpaceAvailable =
        cec_programmed_info::CEC_PROGRAMMED_INFO_MAY_NOT_BE_ENOUGH_SPACE_AVAILABLE,
    NoMediaInfoAvailable = cec_programmed_info::CEC_PROGRAMMED_INFO_NO_MEDIA_INFO_AVAILABLE,
}

#[EnumRepr(type = "cec_not_programmed_error_info")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NotProgrammedErrorInfo {
    FutureUse = cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_FUTURE_USE,
    NoFreeTimerAvailable =
        cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_NO_FREE_TIMER_AVAILABLE,
    DateOutOfRange = cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_DATE_OUT_OF_RANGE,
    RecordingSequenceError =
        cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_RECORDING_SEQUENCE_ERROR,
    InvalidExternalPlugNumber =
        cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_INVALID_EXTERNAL_PLUG_NUMBER,
    InvalidExternalPhysicalAddress = cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_INVALID_EXTERNAL_PHYSICAL_ADDRESS,
    CaSystemNotSupported =
        cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_CA_SYSTEM_NOT_SUPPORTED,
    NoOrInsufficientCaEntitlements = cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_NO_OR_INSUFFICIENT_CA_ENTITLEMENTS,
    DoesNotSupportResolution =
        cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_DOES_NOT_SUPPORT_RESOLUTION,
    ParentalLockOn = cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_PARENTAL_LOCK_ON,
    ClockFailure = cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_CLOCK_FAILURE,
    ReservedForFutureUseStart =
        cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_RESERVED_FOR_FUTURE_USE_START,
    ReservedForFutureUseEnd =
        cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_RESERVED_FOR_FUTURE_USE_END,
    DuplicateAlreadyProgrammed =
        cec_not_programmed_error_info::CEC_NOT_PROGRAMMED_ERROR_INFO_DUPLICATE_ALREADY_PROGRAMMED,
}

#[EnumRepr(type = "cec_recording_flag")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RecordingFlag {
    NotBeingUsedForRecording = cec_recording_flag::CEC_RECORDING_FLAG_NOT_BEING_USED_FOR_RECORDING,
    BeingUsedForRecording = cec_recording_flag::CEC_RECORDING_FLAG_BEING_USED_FOR_RECORDING,
}

#[EnumRepr(type = "cec_tuner_display_info")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TunerDisplayInfo {
    DisplayingDigitalTuner =
        cec_tuner_display_info::CEC_TUNER_DISPLAY_INFO_DISPLAYING_DIGITAL_TUNER,
    NotDisplayingTuner = cec_tuner_display_info::CEC_TUNER_DISPLAY_INFO_NOT_DISPLAYING_TUNER,
    DisplayingAnalogueTuner =
        cec_tuner_display_info::CEC_TUNER_DISPLAY_INFO_DISPLAYING_ANALOGUE_TUNER,
}

#[EnumRepr(type = "cec_broadcast_system")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BroadcastSystem {
    PalBG = cec_broadcast_system::CEC_BROADCAST_SYSTEM_PAL_B_G,
    SecamL1 = cec_broadcast_system::CEC_BROADCAST_SYSTEM_SECAM_L1,
    PalM = cec_broadcast_system::CEC_BROADCAST_SYSTEM_PAL_M,
    NtscM = cec_broadcast_system::CEC_BROADCAST_SYSTEM_NTSC_M,
    PalI = cec_broadcast_system::CEC_BROADCAST_SYSTEM_PAL_I,
    SecamDk = cec_broadcast_system::CEC_BROADCAST_SYSTEM_SECAM_DK,
    SecamBG = cec_broadcast_system::CEC_BROADCAST_SYSTEM_SECAM_B_G,
    SecamL2 = cec_broadcast_system::CEC_BROADCAST_SYSTEM_SECAM_L2,
    PalDk = cec_broadcast_system::CEC_BROADCAST_SYSTEM_PAL_DK,
    OtherSystem = cec_broadcast_system::CEC_BROADCAST_SYSTEM_OTHER_SYSTEM,
}

#[EnumRepr(type = "cec_user_control_code")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UserControlCode {
    Select = cec_user_control_code::CEC_USER_CONTROL_CODE_SELECT,
    Up = cec_user_control_code::CEC_USER_CONTROL_CODE_UP,
    Down = cec_user_control_code::CEC_USER_CONTROL_CODE_DOWN,
    Left = cec_user_control_code::CEC_USER_CONTROL_CODE_LEFT,
    Right = cec_user_control_code::CEC_USER_CONTROL_CODE_RIGHT,
    RightUp = cec_user_control_code::CEC_USER_CONTROL_CODE_RIGHT_UP,
    RightDown = cec_user_control_code::CEC_USER_CONTROL_CODE_RIGHT_DOWN,
    LeftUp = cec_user_control_code::CEC_USER_CONTROL_CODE_LEFT_UP,
    LeftDown = cec_user_control_code::CEC_USER_CONTROL_CODE_LEFT_DOWN,
    RootMenu = cec_user_control_code::CEC_USER_CONTROL_CODE_ROOT_MENU,
    SetupMenu = cec_user_control_code::CEC_USER_CONTROL_CODE_SETUP_MENU,
    ContentsMenu = cec_user_control_code::CEC_USER_CONTROL_CODE_CONTENTS_MENU,
    FavoriteMenu = cec_user_control_code::CEC_USER_CONTROL_CODE_FAVORITE_MENU,
    Exit = cec_user_control_code::CEC_USER_CONTROL_CODE_EXIT,
    TopMenu = cec_user_control_code::CEC_USER_CONTROL_CODE_TOP_MENU,
    DvdMenu = cec_user_control_code::CEC_USER_CONTROL_CODE_DVD_MENU,
    NumberEntryMode = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER_ENTRY_MODE,
    Number11 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER11,
    Number12 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER12,
    Number0 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER0,
    Number1 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER1,
    Number2 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER2,
    Number3 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER3,
    Number4 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER4,
    Number5 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER5,
    Number6 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER6,
    Number7 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER7,
    Number8 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER8,
    Number9 = cec_user_control_code::CEC_USER_CONTROL_CODE_NUMBER9,
    Dot = cec_user_control_code::CEC_USER_CONTROL_CODE_DOT,
    Enter = cec_user_control_code::CEC_USER_CONTROL_CODE_ENTER,
    Clear = cec_user_control_code::CEC_USER_CONTROL_CODE_CLEAR,
    NextFavorite = cec_user_control_code::CEC_USER_CONTROL_CODE_NEXT_FAVORITE,
    ChannelUp = cec_user_control_code::CEC_USER_CONTROL_CODE_CHANNEL_UP,
    ChannelDown = cec_user_control_code::CEC_USER_CONTROL_CODE_CHANNEL_DOWN,
    PreviousChannel = cec_user_control_code::CEC_USER_CONTROL_CODE_PREVIOUS_CHANNEL,
    SoundSelect = cec_user_control_code::CEC_USER_CONTROL_CODE_SOUND_SELECT,
    InputSelect = cec_user_control_code::CEC_USER_CONTROL_CODE_INPUT_SELECT,
    DisplayInformation = cec_user_control_code::CEC_USER_CONTROL_CODE_DISPLAY_INFORMATION,
    Help = cec_user_control_code::CEC_USER_CONTROL_CODE_HELP,
    PageUp = cec_user_control_code::CEC_USER_CONTROL_CODE_PAGE_UP,
    PageDown = cec_user_control_code::CEC_USER_CONTROL_CODE_PAGE_DOWN,
    Power = cec_user_control_code::CEC_USER_CONTROL_CODE_POWER,
    VolumeUp = cec_user_control_code::CEC_USER_CONTROL_CODE_VOLUME_UP,
    VolumeDown = cec_user_control_code::CEC_USER_CONTROL_CODE_VOLUME_DOWN,
    Mute = cec_user_control_code::CEC_USER_CONTROL_CODE_MUTE,
    Play = cec_user_control_code::CEC_USER_CONTROL_CODE_PLAY,
    Stop = cec_user_control_code::CEC_USER_CONTROL_CODE_STOP,
    Pause = cec_user_control_code::CEC_USER_CONTROL_CODE_PAUSE,
    Record = cec_user_control_code::CEC_USER_CONTROL_CODE_RECORD,
    Rewind = cec_user_control_code::CEC_USER_CONTROL_CODE_REWIND,
    FastForward = cec_user_control_code::CEC_USER_CONTROL_CODE_FAST_FORWARD,
    Eject = cec_user_control_code::CEC_USER_CONTROL_CODE_EJECT,
    Forward = cec_user_control_code::CEC_USER_CONTROL_CODE_FORWARD,
    Backward = cec_user_control_code::CEC_USER_CONTROL_CODE_BACKWARD,
    StopRecord = cec_user_control_code::CEC_USER_CONTROL_CODE_STOP_RECORD,
    PauseRecord = cec_user_control_code::CEC_USER_CONTROL_CODE_PAUSE_RECORD,
    Angle = cec_user_control_code::CEC_USER_CONTROL_CODE_ANGLE,
    SubPicture = cec_user_control_code::CEC_USER_CONTROL_CODE_SUB_PICTURE,
    VideoOnDemand = cec_user_control_code::CEC_USER_CONTROL_CODE_VIDEO_ON_DEMAND,
    ElectronicProgramGuide = cec_user_control_code::CEC_USER_CONTROL_CODE_ELECTRONIC_PROGRAM_GUIDE,
    TimerProgramming = cec_user_control_code::CEC_USER_CONTROL_CODE_TIMER_PROGRAMMING,
    InitialConfiguration = cec_user_control_code::CEC_USER_CONTROL_CODE_INITIAL_CONFIGURATION,
    SelectBroadcastType = cec_user_control_code::CEC_USER_CONTROL_CODE_SELECT_BROADCAST_TYPE,
    SelectSoundPresentation =
        cec_user_control_code::CEC_USER_CONTROL_CODE_SELECT_SOUND_PRESENTATION,
    PlayFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_PLAY_FUNCTION,
    PausePlayFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_PAUSE_PLAY_FUNCTION,
    RecordFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_RECORD_FUNCTION,
    PauseRecordFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_PAUSE_RECORD_FUNCTION,
    StopFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_STOP_FUNCTION,
    MuteFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_MUTE_FUNCTION,
    RestoreVolumeFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_RESTORE_VOLUME_FUNCTION,
    TuneFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_TUNE_FUNCTION,
    SelectMediaFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_SELECT_MEDIA_FUNCTION,
    SelectAvInputFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_SELECT_AV_INPUT_FUNCTION,
    SelectAudioInputFunction =
        cec_user_control_code::CEC_USER_CONTROL_CODE_SELECT_AUDIO_INPUT_FUNCTION,
    PowerToggleFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_POWER_TOGGLE_FUNCTION,
    PowerOffFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_POWER_OFF_FUNCTION,
    PowerOnFunction = cec_user_control_code::CEC_USER_CONTROL_CODE_POWER_ON_FUNCTION,
    F1Blue = cec_user_control_code::CEC_USER_CONTROL_CODE_F1_BLUE,
    F2Red = cec_user_control_code::CEC_USER_CONTROL_CODE_F2_RED,
    F3Green = cec_user_control_code::CEC_USER_CONTROL_CODE_F3_GREEN,
    F4Yellow = cec_user_control_code::CEC_USER_CONTROL_CODE_F4_YELLOW,
    F5 = cec_user_control_code::CEC_USER_CONTROL_CODE_F5,
    Data = cec_user_control_code::CEC_USER_CONTROL_CODE_DATA,
    AnReturn = cec_user_control_code::CEC_USER_CONTROL_CODE_AN_RETURN,
    AnChannelsList = cec_user_control_code::CEC_USER_CONTROL_CODE_AN_CHANNELS_LIST,
    Unknown = cec_user_control_code::CEC_USER_CONTROL_CODE_UNKNOWN,
}

#[EnumRepr(type = "cec_logical_address")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogicalAddress {
    Unknown = cec_logical_address::CECDEVICE_UNKNOWN,
    Tv = cec_logical_address::CECDEVICE_TV,
    Recordingdevice1 = cec_logical_address::CECDEVICE_RECORDINGDEVICE1,
    Recordingdevice2 = cec_logical_address::CECDEVICE_RECORDINGDEVICE2,
    Tuner1 = cec_logical_address::CECDEVICE_TUNER1,
    Playbackdevice1 = cec_logical_address::CECDEVICE_PLAYBACKDEVICE1,
    Audiosystem = cec_logical_address::CECDEVICE_AUDIOSYSTEM,
    Tuner2 = cec_logical_address::CECDEVICE_TUNER2,
    Tuner3 = cec_logical_address::CECDEVICE_TUNER3,
    Playbackdevice2 = cec_logical_address::CECDEVICE_PLAYBACKDEVICE2,
    Recordingdevice3 = cec_logical_address::CECDEVICE_RECORDINGDEVICE3,
    Tuner4 = cec_logical_address::CECDEVICE_TUNER4,
    Playbackdevice3 = cec_logical_address::CECDEVICE_PLAYBACKDEVICE3,
    Reserved1 = cec_logical_address::CECDEVICE_RESERVED1,
    Reserved2 = cec_logical_address::CECDEVICE_RESERVED2,
    Freeuse = cec_logical_address::CECDEVICE_FREEUSE,
    Unregistered = cec_logical_address::CECDEVICE_UNREGISTERED,
}

#[EnumRepr(type = "cec_opcode")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    ActiveSource = cec_opcode::CEC_OPCODE_ACTIVE_SOURCE,
    ImageViewOn = cec_opcode::CEC_OPCODE_IMAGE_VIEW_ON,
    TextViewOn = cec_opcode::CEC_OPCODE_TEXT_VIEW_ON,
    InactiveSource = cec_opcode::CEC_OPCODE_INACTIVE_SOURCE,
    RequestActiveSource = cec_opcode::CEC_OPCODE_REQUEST_ACTIVE_SOURCE,
    RoutingChange = cec_opcode::CEC_OPCODE_ROUTING_CHANGE,
    RoutingInformation = cec_opcode::CEC_OPCODE_ROUTING_INFORMATION,
    SetStreamPath = cec_opcode::CEC_OPCODE_SET_STREAM_PATH,
    Standby = cec_opcode::CEC_OPCODE_STANDBY,
    RecordOff = cec_opcode::CEC_OPCODE_RECORD_OFF,
    RecordOn = cec_opcode::CEC_OPCODE_RECORD_ON,
    RecordStatus = cec_opcode::CEC_OPCODE_RECORD_STATUS,
    RecordTvScreen = cec_opcode::CEC_OPCODE_RECORD_TV_SCREEN,
    ClearAnalogueTimer = cec_opcode::CEC_OPCODE_CLEAR_ANALOGUE_TIMER,
    ClearDigitalTimer = cec_opcode::CEC_OPCODE_CLEAR_DIGITAL_TIMER,
    ClearExternalTimer = cec_opcode::CEC_OPCODE_CLEAR_EXTERNAL_TIMER,
    SetAnalogueTimer = cec_opcode::CEC_OPCODE_SET_ANALOGUE_TIMER,
    SetDigitalTimer = cec_opcode::CEC_OPCODE_SET_DIGITAL_TIMER,
    SetExternalTimer = cec_opcode::CEC_OPCODE_SET_EXTERNAL_TIMER,
    SetTimerProgramTitle = cec_opcode::CEC_OPCODE_SET_TIMER_PROGRAM_TITLE,
    TimerClearedStatus = cec_opcode::CEC_OPCODE_TIMER_CLEARED_STATUS,
    TimerStatus = cec_opcode::CEC_OPCODE_TIMER_STATUS,
    CecVersion = cec_opcode::CEC_OPCODE_CEC_VERSION,
    GetCecVersion = cec_opcode::CEC_OPCODE_GET_CEC_VERSION,
    GivePhysicalAddress = cec_opcode::CEC_OPCODE_GIVE_PHYSICAL_ADDRESS,
    GetMenuLanguage = cec_opcode::CEC_OPCODE_GET_MENU_LANGUAGE,
    ReportPhysicalAddress = cec_opcode::CEC_OPCODE_REPORT_PHYSICAL_ADDRESS,
    SetMenuLanguage = cec_opcode::CEC_OPCODE_SET_MENU_LANGUAGE,
    DeckControl = cec_opcode::CEC_OPCODE_DECK_CONTROL,
    DeckStatus = cec_opcode::CEC_OPCODE_DECK_STATUS,
    GiveDeckStatus = cec_opcode::CEC_OPCODE_GIVE_DECK_STATUS,
    Play = cec_opcode::CEC_OPCODE_PLAY,
    GiveTunerDeviceStatus = cec_opcode::CEC_OPCODE_GIVE_TUNER_DEVICE_STATUS,
    SelectAnalogueService = cec_opcode::CEC_OPCODE_SELECT_ANALOGUE_SERVICE,
    SelectDigitalService = cec_opcode::CEC_OPCODE_SELECT_DIGITAL_SERVICE,
    TunerDeviceStatus = cec_opcode::CEC_OPCODE_TUNER_DEVICE_STATUS,
    TunerStepDecrement = cec_opcode::CEC_OPCODE_TUNER_STEP_DECREMENT,
    TunerStepIncrement = cec_opcode::CEC_OPCODE_TUNER_STEP_INCREMENT,
    DeviceVendorId = cec_opcode::CEC_OPCODE_DEVICE_VENDOR_ID,
    GiveDeviceVendorId = cec_opcode::CEC_OPCODE_GIVE_DEVICE_VENDOR_ID,
    VendorCommand = cec_opcode::CEC_OPCODE_VENDOR_COMMAND,
    VendorCommandWithId = cec_opcode::CEC_OPCODE_VENDOR_COMMAND_WITH_ID,
    VendorRemoteButtonDown = cec_opcode::CEC_OPCODE_VENDOR_REMOTE_BUTTON_DOWN,
    VendorRemoteButtonUp = cec_opcode::CEC_OPCODE_VENDOR_REMOTE_BUTTON_UP,
    SetOsdString = cec_opcode::CEC_OPCODE_SET_OSD_STRING,
    GiveOsdName = cec_opcode::CEC_OPCODE_GIVE_OSD_NAME,
    SetOsdName = cec_opcode::CEC_OPCODE_SET_OSD_NAME,
    MenuRequest = cec_opcode::CEC_OPCODE_MENU_REQUEST,
    MenuStatus = cec_opcode::CEC_OPCODE_MENU_STATUS,
    UserControlPressed = cec_opcode::CEC_OPCODE_USER_CONTROL_PRESSED,
    UserControlRelease = cec_opcode::CEC_OPCODE_USER_CONTROL_RELEASE,
    GiveDevicePowerStatus = cec_opcode::CEC_OPCODE_GIVE_DEVICE_POWER_STATUS,
    ReportPowerStatus = cec_opcode::CEC_OPCODE_REPORT_POWER_STATUS,
    FeatureAbort = cec_opcode::CEC_OPCODE_FEATURE_ABORT,
    Abort = cec_opcode::CEC_OPCODE_ABORT,
    GiveAudioStatus = cec_opcode::CEC_OPCODE_GIVE_AUDIO_STATUS,
    GiveSystemAudioModeStatus = cec_opcode::CEC_OPCODE_GIVE_SYSTEM_AUDIO_MODE_STATUS,
    ReportAudioStatus = cec_opcode::CEC_OPCODE_REPORT_AUDIO_STATUS,
    SetSystemAudioMode = cec_opcode::CEC_OPCODE_SET_SYSTEM_AUDIO_MODE,
    SystemAudioModeRequest = cec_opcode::CEC_OPCODE_SYSTEM_AUDIO_MODE_REQUEST,
    SystemAudioModeStatus = cec_opcode::CEC_OPCODE_SYSTEM_AUDIO_MODE_STATUS,
    SetAudioRate = cec_opcode::CEC_OPCODE_SET_AUDIO_RATE,
    ReportShortAudioDescriptors = cec_opcode::CEC_OPCODE_REPORT_SHORT_AUDIO_DESCRIPTORS,
    RequestShortAudioDescriptors = cec_opcode::CEC_OPCODE_REQUEST_SHORT_AUDIO_DESCRIPTORS,
    StartArc = cec_opcode::CEC_OPCODE_START_ARC,
    ReportArcStarted = cec_opcode::CEC_OPCODE_REPORT_ARC_STARTED,
    ReportArcEnded = cec_opcode::CEC_OPCODE_REPORT_ARC_ENDED,
    RequestArcStart = cec_opcode::CEC_OPCODE_REQUEST_ARC_START,
    RequestArcEnd = cec_opcode::CEC_OPCODE_REQUEST_ARC_END,
    EndArc = cec_opcode::CEC_OPCODE_END_ARC,
    Cdc = cec_opcode::CEC_OPCODE_CDC,
    None = cec_opcode::CEC_OPCODE_NONE,
}

#[EnumRepr(type = "cec_log_level")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Error = cec_log_level::CEC_LOG_ERROR,
    Warning = cec_log_level::CEC_LOG_WARNING,
    Notice = cec_log_level::CEC_LOG_NOTICE,
    Traffic = cec_log_level::CEC_LOG_TRAFFIC,
    Debug = cec_log_level::CEC_LOG_DEBUG,
    All = cec_log_level::CEC_LOG_ALL,
}

#[EnumRepr(type = "cec_bus_device_status")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BusDeviceStatus {
    Unknown = cec_bus_device_status::CEC_DEVICE_STATUS_UNKNOWN,
    Present = cec_bus_device_status::CEC_DEVICE_STATUS_PRESENT,
    NotPresent = cec_bus_device_status::CEC_DEVICE_STATUS_NOT_PRESENT,
    HandledByLibcec = cec_bus_device_status::CEC_DEVICE_STATUS_HANDLED_BY_LIBCEC,
}

#[EnumRepr(type = "cec_vendor_id")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VendorId {
    Toshiba = cec_vendor_id::CEC_VENDOR_TOSHIBA,
    Samsung = cec_vendor_id::CEC_VENDOR_SAMSUNG,
    Denon = cec_vendor_id::CEC_VENDOR_DENON,
    Marantz = cec_vendor_id::CEC_VENDOR_MARANTZ,
    Loewe = cec_vendor_id::CEC_VENDOR_LOEWE,
    Onkyo = cec_vendor_id::CEC_VENDOR_ONKYO,
    Medion = cec_vendor_id::CEC_VENDOR_MEDION,
    Toshiba2 = cec_vendor_id::CEC_VENDOR_TOSHIBA2,
    Apple = cec_vendor_id::CEC_VENDOR_APPLE,
    PulseEight = cec_vendor_id::CEC_VENDOR_PULSE_EIGHT,
    HarmanKardon2 = cec_vendor_id::CEC_VENDOR_HARMAN_KARDON2,
    Google = cec_vendor_id::CEC_VENDOR_GOOGLE,
    Akai = cec_vendor_id::CEC_VENDOR_AKAI,
    Aoc = cec_vendor_id::CEC_VENDOR_AOC,
    Panasonic = cec_vendor_id::CEC_VENDOR_PANASONIC,
    Philips = cec_vendor_id::CEC_VENDOR_PHILIPS,
    Daewoo = cec_vendor_id::CEC_VENDOR_DAEWOO,
    Yamaha = cec_vendor_id::CEC_VENDOR_YAMAHA,
    Grundig = cec_vendor_id::CEC_VENDOR_GRUNDIG,
    Pioneer = cec_vendor_id::CEC_VENDOR_PIONEER,
    Lg = cec_vendor_id::CEC_VENDOR_LG,
    Sharp = cec_vendor_id::CEC_VENDOR_SHARP,
    Sony = cec_vendor_id::CEC_VENDOR_SONY,
    Broadcom = cec_vendor_id::CEC_VENDOR_BROADCOM,
    Sharp2 = cec_vendor_id::CEC_VENDOR_SHARP2,
    Vizio = cec_vendor_id::CEC_VENDOR_VIZIO,
    Benq = cec_vendor_id::CEC_VENDOR_BENQ,
    HarmanKardon = cec_vendor_id::CEC_VENDOR_HARMAN_KARDON,
    Unknown = cec_vendor_id::CEC_VENDOR_UNKNOWN,
}

#[EnumRepr(type = "cec_adapter_type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AdapterType {
    Unknown = cec_adapter_type::ADAPTERTYPE_UNKNOWN,
    P8External = cec_adapter_type::ADAPTERTYPE_P8_EXTERNAL,
    P8Daughterboard = cec_adapter_type::ADAPTERTYPE_P8_DAUGHTERBOARD,
    Rpi = cec_adapter_type::ADAPTERTYPE_RPI,
    Tda995x = cec_adapter_type::ADAPTERTYPE_TDA995x,
    Exynos = cec_adapter_type::ADAPTERTYPE_EXYNOS,
    Linux = cec_adapter_type::ADAPTERTYPE_LINUX,
    Aocec = cec_adapter_type::ADAPTERTYPE_AOCEC,
    Imx = cec_adapter_type::ADAPTERTYPE_IMX,
}

#[EnumRepr(type = "libcec_version")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LibraryVersion {
    Current = libcec_version::LIBCEC_VERSION_CURRENT,
}

#[EnumRepr(type = "libcec_alert")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Alert {
    ServiceDevice = libcec_alert::CEC_ALERT_SERVICE_DEVICE,
    ConnectionLost = libcec_alert::CEC_ALERT_CONNECTION_LOST,
    PermissionError = libcec_alert::CEC_ALERT_PERMISSION_ERROR,
    PortBusy = libcec_alert::CEC_ALERT_PORT_BUSY,
    PhysicalAddressError = libcec_alert::CEC_ALERT_PHYSICAL_ADDRESS_ERROR,
    TvPollFailed = libcec_alert::CEC_ALERT_TV_POLL_FAILED,
}

#[EnumRepr(type = "libcec_parameter_type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ParameterType {
    String = libcec_parameter_type::CEC_PARAMETER_TYPE_STRING,
    Unknown = libcec_parameter_type::CEC_PARAMETER_TYPE_UNKOWN,
}

impl TryFrom<c_int> for LogicalAddress {
    type Error = TryFromLogicalAddressesError;

    fn try_from(value: c_int) -> Result<Self, Self::Error> {
        let x = match value {
            -1 => LogicalAddress::Unknown,
            0 => LogicalAddress::Tv,
            1 => LogicalAddress::Recordingdevice1,
            2 => LogicalAddress::Recordingdevice2,
            3 => LogicalAddress::Tuner1,
            4 => LogicalAddress::Playbackdevice1,
            5 => LogicalAddress::Audiosystem,
            6 => LogicalAddress::Tuner2,
            7 => LogicalAddress::Tuner3,
            8 => LogicalAddress::Playbackdevice2,
            9 => LogicalAddress::Recordingdevice3,
            10 => LogicalAddress::Tuner4,
            11 => LogicalAddress::Playbackdevice3,
            12 => LogicalAddress::Reserved1,
            13 => LogicalAddress::Reserved2,
            14 => LogicalAddress::Freeuse,
            15 => LogicalAddress::Unregistered,
            _ => return Err(TryFromLogicalAddressesError::InvalidPrimaryAddress),
        };

        Ok(x)
    }
}
