/* automatically generated by rust-bindgen 0.64.0 */

#![allow(dead_code, non_camel_case_types, non_snake_case, trivial_numeric_casts)]
#![allow(clippy::unreadable_literal, clippy::upper_case_acronyms)]

pub type ULONG = ::std::os::raw::c_ulong;
pub type PULONG = *mut ULONG;
pub type USHORT = ::std::os::raw::c_ushort;
pub type UCHAR = ::std::os::raw::c_uchar;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type SHORT = ::std::os::raw::c_short;
pub type BOOLEAN = BYTE;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _VIGEM_TARGET_TYPE {
    Xbox360Wired = 0,
    DualShock4Wired = 2,
}
pub use self::_VIGEM_TARGET_TYPE as VIGEM_TARGET_TYPE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _XUSB_REPORT {
    pub wButtons: USHORT,
    pub bLeftTrigger: BYTE,
    pub bRightTrigger: BYTE,
    pub sThumbLX: SHORT,
    pub sThumbLY: SHORT,
    pub sThumbRX: SHORT,
    pub sThumbRY: SHORT,
}
pub type XUSB_REPORT = _XUSB_REPORT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DS4_LIGHTBAR_COLOR {
    pub Red: UCHAR,
    pub Green: UCHAR,
    pub Blue: UCHAR,
}
pub type DS4_LIGHTBAR_COLOR = _DS4_LIGHTBAR_COLOR;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DS4_REPORT {
    pub bThumbLX: BYTE,
    pub bThumbLY: BYTE,
    pub bThumbRX: BYTE,
    pub bThumbRY: BYTE,
    pub wButtons: USHORT,
    pub bSpecial: BYTE,
    pub bTriggerL: BYTE,
    pub bTriggerR: BYTE,
}
pub type DS4_REPORT = _DS4_REPORT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DS4_TOUCH {
    pub bPacketCounter: BYTE,
    pub bIsUpTrackingNum1: BYTE,
    pub bTouchData1: [BYTE; 3usize],
    pub bIsUpTrackingNum2: BYTE,
    pub bTouchData2: [BYTE; 3usize],
}
pub type DS4_TOUCH = _DS4_TOUCH;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _DS4_REPORT_EX {
    pub __bindgen_anon_1: _DS4_REPORT_EX__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _DS4_REPORT_EX__bindgen_ty_1 {
    pub Report: _DS4_REPORT_EX__bindgen_ty_1__bindgen_ty_1,
    pub ReportBuffer: [UCHAR; 63usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _DS4_REPORT_EX__bindgen_ty_1__bindgen_ty_1 {
    pub bThumbLX: BYTE,
    pub bThumbLY: BYTE,
    pub bThumbRX: BYTE,
    pub bThumbRY: BYTE,
    pub wButtons: USHORT,
    pub bSpecial: BYTE,
    pub bTriggerL: BYTE,
    pub bTriggerR: BYTE,
    pub wTimestamp: USHORT,
    pub bBatteryLvl: BYTE,
    pub wGyroX: SHORT,
    pub wGyroY: SHORT,
    pub wGyroZ: SHORT,
    pub wAccelX: SHORT,
    pub wAccelY: SHORT,
    pub wAccelZ: SHORT,
    pub _bUnknown1: [BYTE; 5usize],
    pub bBatteryLvlSpecial: BYTE,
    pub _bUnknown2: [BYTE; 2usize],
    pub bTouchPacketsN: BYTE,
    pub sCurrentTouch: DS4_TOUCH,
    pub sPreviousTouch: [DS4_TOUCH; 2usize],
}
pub type DS4_REPORT_EX = _DS4_REPORT_EX;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DS4_OUTPUT_BUFFER {
    pub Buffer: [UCHAR; 64usize],
}
pub type PDS4_OUTPUT_BUFFER = *mut _DS4_OUTPUT_BUFFER;
#[repr(i32)]
#[doc = " Values that represent ViGEm errors"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _VIGEM_ERRORS {
    VIGEM_ERROR_NONE = 536870912,
    VIGEM_ERROR_BUS_NOT_FOUND = -536870911,
    VIGEM_ERROR_NO_FREE_SLOT = -536870910,
    VIGEM_ERROR_INVALID_TARGET = -536870909,
    VIGEM_ERROR_REMOVAL_FAILED = -536870908,
    VIGEM_ERROR_ALREADY_CONNECTED = -536870907,
    VIGEM_ERROR_TARGET_UNINITIALIZED = -536870906,
    VIGEM_ERROR_TARGET_NOT_PLUGGED_IN = -536870905,
    VIGEM_ERROR_BUS_VERSION_MISMATCH = -536870904,
    VIGEM_ERROR_BUS_ACCESS_FAILED = -536870903,
    VIGEM_ERROR_CALLBACK_ALREADY_REGISTERED = -536870896,
    VIGEM_ERROR_CALLBACK_NOT_FOUND = -536870895,
    VIGEM_ERROR_BUS_ALREADY_CONNECTED = -536870894,
    VIGEM_ERROR_BUS_INVALID_HANDLE = -536870893,
    VIGEM_ERROR_XUSB_USERINDEX_OUT_OF_RANGE = -536870892,
    VIGEM_ERROR_INVALID_PARAMETER = -536870891,
    VIGEM_ERROR_NOT_SUPPORTED = -536870890,
    VIGEM_ERROR_WINAPI = -536870889,
    VIGEM_ERROR_TIMED_OUT = -536870888,
    VIGEM_ERROR_IS_DISPOSING = -536870887,
}
#[doc = " Values that represent ViGEm errors"]
pub use self::_VIGEM_ERRORS as VIGEM_ERROR;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VIGEM_CLIENT_T {
    _unused: [u8; 0],
}
#[doc = " Defines an alias representing a driver connection object"]
pub type PVIGEM_CLIENT = *mut _VIGEM_CLIENT_T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VIGEM_TARGET_T {
    _unused: [u8; 0],
}
#[doc = " Defines an alias representing a target device object"]
pub type PVIGEM_TARGET = *mut _VIGEM_TARGET_T;
pub type PFN_VIGEM_TARGET_ADD_RESULT = ::std::option::Option<
    unsafe extern "C" fn(arg1: PVIGEM_CLIENT, arg2: PVIGEM_TARGET, arg3: VIGEM_ERROR),
>;
pub type PFN_VIGEM_X360_NOTIFICATION = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: PVIGEM_CLIENT,
        arg2: PVIGEM_TARGET,
        arg3: UCHAR,
        arg4: UCHAR,
        arg5: UCHAR,
        arg6: LPVOID,
    ),
>;
pub type PFN_VIGEM_DS4_NOTIFICATION = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: PVIGEM_CLIENT,
        arg2: PVIGEM_TARGET,
        arg3: UCHAR,
        arg4: UCHAR,
        arg5: DS4_LIGHTBAR_COLOR,
        arg6: LPVOID,
    ),
>;
extern "C" {
    #[doc = "  Allocates an object representing a driver connection\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t28.08.2017\n\n @returns\tA PVIGEM_CLIENT object."]
    pub fn vigem_alloc() -> PVIGEM_CLIENT;
}
extern "C" {
    #[doc = " Frees up memory used by the driver connection object\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t28.08.2017\n\n @param \tvigem\tThe PVIGEM_CLIENT object."]
    pub fn vigem_free(vigem: PVIGEM_CLIENT);
}
extern "C" {
    #[doc = " Initializes the driver object and establishes a connection to the emulation bus\n          driver. Returns an error if no compatible bus device has been found.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t28.08.2017\n\n @param \tvigem\tThe PVIGEM_CLIENT object.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_connect(vigem: PVIGEM_CLIENT) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Disconnects from the bus device and resets the driver object state. The driver object\n           may be reused again after calling this function. When called, all targets which may\n           still be connected will be destroyed automatically. Be aware, that allocated target\n           objects won't be automatically freed, this has to be taken care of by the caller.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t28.08.2017\n\n @param \tvigem\tThe PVIGEM_CLIENT object."]
    pub fn vigem_disconnect(vigem: PVIGEM_CLIENT);
}
extern "C" {
    #[doc = " A useful utility function to check if pre 1.17 driver, meant to be replaced in the future by\n          more robust version checks, only able to be checked after at least one device has been\n          successfully plugged in.\n\n @author\tJason \"megadrago88\" Hart\n @date\t17.08.2021\n\n @param   target  The PVIGEM_TARGET to check against.\n\n @returns\tA BOOLEAN, true if the device wait ready ioctl is supported (1.17+) or false if not ( =< 1.16)"]
    pub fn vigem_target_is_waitable_add_supported(target: PVIGEM_TARGET) -> BOOLEAN;
}
extern "C" {
    #[doc = " Allocates an object representing an Xbox 360 Controller device.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t28.08.2017\n\n @returns\tA PVIGEM_TARGET representing an Xbox 360 Controller device."]
    pub fn vigem_target_x360_alloc() -> PVIGEM_TARGET;
}
extern "C" {
    #[doc = " Allocates an object representing a DualShock 4 Controller device.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t28.08.2017\n\n @returns\tA PVIGEM_TARGET representing a DualShock 4 Controller device."]
    pub fn vigem_target_ds4_alloc() -> PVIGEM_TARGET;
}
extern "C" {
    #[doc = " Frees up memory used by the target device object. This does not automatically remove\n          the associated device from the bus, if present. If the target device doesn't get\n          removed before this call, the device becomes orphaned until the owning process is\n          terminated.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t28.08.2017\n\n @param \ttarget\tThe target device object."]
    pub fn vigem_target_free(target: PVIGEM_TARGET);
}
extern "C" {
    #[doc = " Adds a provided target device to the bus driver, which is equal to a device plug-in\n          event of a physical hardware device. This function blocks until the target device is\n          in full operational mode.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t28.08.2017\n\n @param \tvigem \tThe driver connection object.\n @param \ttarget\tThe target device object.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_add(vigem: PVIGEM_CLIENT, target: PVIGEM_TARGET) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Adds a provided target device to the bus driver, which is equal to a device plug-in\n          event of a physical hardware device. This function immediately returns. An optional\n          callback may be registered which gets called on error or if the target device has\n          become fully operational.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t28.08.2017\n\n @param \tvigem \tThe driver connection object.\n @param \ttarget\tThe target device object.\n @param \tresult\tAn optional function getting called when the target device becomes available.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_add_async(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        result: PFN_VIGEM_TARGET_ADD_RESULT,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Removes a provided target device from the bus driver, which is equal to a device\n           unplug event of a physical hardware device. The target device object may be reused\n           after this function is called. If this function is never called on target device\n           objects, they will be removed from the bus when the owning process terminates.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \tvigem \tThe driver connection object.\n @param \ttarget\tThe target device object.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_remove(vigem: PVIGEM_CLIENT, target: PVIGEM_TARGET) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Registers a function which gets called, when LED index or vibration state changes\n                 occur on the provided target device. This function fails if the provided\n                 target device isn't fully operational or in an erroneous state.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \tvigem\t\t\tThe driver connection object.\n @param \ttarget\t\t\tThe target device object.\n @param \tnotification\tThe notification callback.\n @param \tuserData\t\tThe user data passed to the notification callback.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_x360_register_notification(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        notification: PFN_VIGEM_X360_NOTIFICATION,
        userData: LPVOID,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " This function is deprecated, use vigem_target_ds4_await_output_report or\n vigem_target_ds4_await_output_report_timeout instead. Registers a function which gets called,\n when LightBar or vibration state changes occur on the provided target device. This function\n fails if the provided target device isn't fully operational or in an erroneous state.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \tvigem\t\t\tThe driver connection object.\n @param \ttarget\t\t\tThe target device object.\n @param \tnotification\tThe notification callback.\n @param \tuserData\t\tThe user data passed to the notification callback.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_ds4_register_notification(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        notification: PFN_VIGEM_DS4_NOTIFICATION,
        userData: LPVOID,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Removes a previously registered callback function from the provided target object.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \ttarget\tThe target device object."]
    pub fn vigem_target_x360_unregister_notification(target: PVIGEM_TARGET);
}
extern "C" {
    #[doc = " Removes a previously registered callback function from the provided target object.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \ttarget\tThe target device object."]
    pub fn vigem_target_ds4_unregister_notification(target: PVIGEM_TARGET);
}
extern "C" {
    #[doc = " Overrides the default Vendor ID value with the provided one.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \ttarget\tThe target device object.\n @param \tvid   \tThe Vendor ID to set."]
    pub fn vigem_target_set_vid(target: PVIGEM_TARGET, vid: USHORT);
}
extern "C" {
    #[doc = " Overrides the default Product ID value with the provided one.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \ttarget\tThe target device object.\n @param \tpid   \tThe Product ID to set."]
    pub fn vigem_target_set_pid(target: PVIGEM_TARGET, pid: USHORT);
}
extern "C" {
    #[doc = " Returns the Vendor ID of the provided target device object.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \ttarget\tThe target device object.\n\n @returns\tThe Vendor ID."]
    pub fn vigem_target_get_vid(target: PVIGEM_TARGET) -> USHORT;
}
extern "C" {
    #[doc = " Returns the Product ID of the provided target device object.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \ttarget\tThe target device object.\n\n @returns\tThe Product ID."]
    pub fn vigem_target_get_pid(target: PVIGEM_TARGET) -> USHORT;
}
extern "C" {
    #[doc = " Sends a state report to the provided target device.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \tvigem \tThe driver connection object.\n @param \ttarget\tThe target device object.\n @param \treport\tThe report to send to the target device.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_x360_update(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        report: XUSB_REPORT,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " DEPRECATED. Sends a state report to the provided target device. It's recommended to use\n vigem_target_ds4_update_ex instead to utilize all DS4 features like touch, gyro etc.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \tvigem \tThe driver connection object.\n @param \ttarget\tThe target device object.\n @param \treport\tThe report to send to the target device.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_ds4_update(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        report: DS4_REPORT,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Sends a full size state report to the provided target device. It's recommended to use this\n function over vigem_target_ds4_update.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t07.09.2020\n\n @param \tvigem \tThe driver connection object.\n @param \ttarget\tThe target device object.\n @param \treport\tThe report buffer.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_ds4_update_ex(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        report: DS4_REPORT_EX,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Returns the internal index (serial number) the bus driver assigned to the provided\n               target device object. Note that this value is specific to the inner workings of\n               the bus driver, it does not reflect related values like player index or device\n               arrival order experienced by other APIs. It may be used to identify the target\n               device object for its lifetime. This value becomes invalid once the target\n               device is removed from the bus and may change on the next addition of the\n               device.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \ttarget\tThe target device object.\n\n @returns\tThe internally used index of the target device."]
    pub fn vigem_target_get_index(target: PVIGEM_TARGET) -> ULONG;
}
extern "C" {
    #[doc = " Returns the type of the provided target device object.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t28.08.2017\n\n @param \ttarget\tThe target device object.\n\n @returns\tA VIGEM_TARGET_TYPE."]
    pub fn vigem_target_get_type(target: PVIGEM_TARGET) -> VIGEM_TARGET_TYPE;
}
extern "C" {
    #[doc = " Returns TRUE if the provided target device object is currently attached to the bus,\n              FALSE otherwise.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t30.08.2017\n\n @param \ttarget\tThe target device object.\n\n @returns\tTRUE if device is attached to the bus, FALSE otherwise."]
    pub fn vigem_target_is_attached(target: PVIGEM_TARGET) -> BOOL;
}
extern "C" {
    #[doc = " Returns the user index of the emulated Xenon device. This value correspondents to the\n                (zero-based) index number representing the player number via LED present on a\n                physical controller and is compatible to the dwUserIndex property of the\n                XInput* APIs.\n\n @author\tBenjamin \"Nefarius\" H�glinger\n @date\t10.05.2018\n\n @param \tvigem \tThe driver connection object.\n @param \ttarget\tThe target device object.\n @param \tindex \tThe (zero-based) user index of the Xenon device.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_x360_get_user_index(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        index: PULONG,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Waits until there's one or more pending raw output reports available to consume. This\n function blocks until data becomes available or the device gets disconnected. The waiting is\n event-based, meaning that as soon as a data packet is pending, this call returns a copy of\n the entire buffer. Each call returns a packet in the exact order it arrived in the driver. It\n is recommended to repeatedly call this function in a thread. The call aborts with an error\n code if the target gets unplugged in parallel.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t06.08.2022\n\n @param \tvigem \tThe driver connection object.\n @param \ttarget\tThe target device object.\n @param \tbuffer\tThe fixed-size 64-bytes output report buffer that gets written to.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_ds4_await_output_report(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        buffer: PDS4_OUTPUT_BUFFER,
    ) -> VIGEM_ERROR;
}
extern "C" {
    #[doc = " Waits until there's one or more pending raw output reports available to consume. This\n function blocks until data becomes available, the provided timeout has been reached or the\n device gets disconnected. The waiting is event-based, meaning that as soon as a data packet\n is pending, this call returns a copy of the entire buffer. Each call returns a packet in the\n exact order it arrived in the driver. It is recommended to repeatedly call this function in a\n thread. A timeout of a few hundred milliseconds can be used to break out of the loop without\n excessive CPU consumption. The call aborts with an error code if the target gets unplugged in\n parallel. If a timeout of INFINITE is specified, the function basically behaves identical to\n vigem_target_ds4_await_output_report.\n\n @author\tBenjamin \"Nefarius\" H�glinger-Stelzer\n @date\t12.08.2022\n\n @param \tvigem\t\t\tThe driver connection object.\n @param \ttarget\t\t\tThe target device object.\n @param \tmilliseconds\tThe timeout in milliseconds.\n @param \tbuffer\t\t\tThe fixed-size 64-bytes output report buffer that gets written to.\n\n @returns\tA VIGEM_ERROR."]
    pub fn vigem_target_ds4_await_output_report_timeout(
        vigem: PVIGEM_CLIENT,
        target: PVIGEM_TARGET,
        milliseconds: DWORD,
        buffer: PDS4_OUTPUT_BUFFER,
    ) -> VIGEM_ERROR;
}
