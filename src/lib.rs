#![doc = "Peripheral access API for ESPRESSIF microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "EFUSE"]
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE {}
impl EFUSE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse::RegisterBlock {
        0x3ff5_a000 as *const _
    }
}
impl Deref for EFUSE {
    type Target = efuse::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFUSE::ptr() }
    }
}
#[doc = "EFUSE"]
pub mod efuse;
#[doc = "RTC_I2C"]
pub struct RTC_I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_I2C {}
impl RTC_I2C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_i2c::RegisterBlock {
        0x3ff4_8c00 as *const _
    }
}
impl Deref for RTC_I2C {
    type Target = rtc_i2c::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC_I2C::ptr() }
    }
}
#[doc = "RTC_I2C"]
pub mod rtc_i2c;
#[doc = "SLC"]
pub struct SLC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SLC {}
impl SLC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const slc::RegisterBlock {
        0x3ff5_8000 as *const _
    }
}
impl Deref for SLC {
    type Target = slc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SLC::ptr() }
    }
}
#[doc = "SLC"]
pub mod slc;
#[doc = "DPORT"]
pub struct DPORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DPORT {}
impl DPORT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dport::RegisterBlock {
        0x3ff0_0000 as *const _
    }
}
impl Deref for DPORT {
    type Target = dport::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DPORT::ptr() }
    }
}
#[doc = "DPORT"]
pub mod dport;
#[doc = "RMT"]
pub struct RMT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMT {}
impl RMT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rmt::RegisterBlock {
        0x3ff5_6000 as *const _
    }
}
impl Deref for RMT {
    type Target = rmt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RMT::ptr() }
    }
}
#[doc = "RMT"]
pub mod rmt;
#[doc = "PCNT"]
pub struct PCNT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT {}
impl PCNT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcnt::RegisterBlock {
        0x3ff5_7000 as *const _
    }
}
impl Deref for PCNT {
    type Target = pcnt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PCNT::ptr() }
    }
}
#[doc = "PCNT"]
pub mod pcnt;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x3ff4_4000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "RTCIO"]
pub struct RTCIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCIO {}
impl RTCIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtcio::RegisterBlock {
        0x3ff4_8400 as *const _
    }
}
impl Deref for RTCIO {
    type Target = rtcio::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTCIO::ptr() }
    }
}
#[doc = "RTCIO"]
pub mod rtcio;
#[doc = "LEDC"]
pub struct LEDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEDC {}
impl LEDC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ledc::RegisterBlock {
        0x3ff5_9000 as *const _
    }
}
impl Deref for LEDC {
    type Target = ledc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LEDC::ptr() }
    }
}
#[doc = "LEDC"]
pub mod ledc;
#[doc = "APB_CTRL"]
pub struct APB_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APB_CTRL {}
impl APB_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apb_ctrl::RegisterBlock {
        0x3ff6_6000 as *const _
    }
}
impl Deref for APB_CTRL {
    type Target = apb_ctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*APB_CTRL::ptr() }
    }
}
#[doc = "APB_CTRL"]
pub mod apb_ctrl;
#[doc = "SYSCON"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscon::RegisterBlock {
        0x3ff6_6000 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "SYSCON"]
pub mod syscon;
#[doc = "GPIO_SD"]
pub struct GPIO_SD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_SD {}
impl GPIO_SD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_sd::RegisterBlock {
        0x3ff4_4f00 as *const _
    }
}
impl Deref for GPIO_SD {
    type Target = gpio_sd::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_SD::ptr() }
    }
}
#[doc = "GPIO_SD"]
pub mod gpio_sd;
#[doc = "I2S"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        0x3ff4_f000 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "I2S"]
pub mod i2s;
#[doc = "SLCHOST"]
pub struct SLCHOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SLCHOST {}
impl SLCHOST {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const slchost::RegisterBlock {
        0x3ff5_5000 as *const _
    }
}
impl Deref for SLCHOST {
    type Target = slchost::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SLCHOST::ptr() }
    }
}
#[doc = "SLCHOST"]
pub mod slchost;
#[doc = "HINF"]
pub struct HINF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HINF {}
impl HINF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hinf::RegisterBlock {
        0x3ff4_b000 as *const _
    }
}
impl Deref for HINF {
    type Target = hinf::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*HINF::ptr() }
    }
}
#[doc = "HINF"]
pub mod hinf;
#[doc = "UART"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        0x3ff4_0000 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "UART"]
pub mod uart;
#[doc = "RTCCNTL"]
pub struct RTCCNTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCCNTL {}
impl RTCCNTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtccntl::RegisterBlock {
        0x3ff4_8000 as *const _
    }
}
impl Deref for RTCCNTL {
    type Target = rtccntl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTCCNTL::ptr() }
    }
}
#[doc = "RTCCNTL"]
pub mod rtccntl;
#[doc = "SENS"]
pub struct SENS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SENS {}
impl SENS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sens::RegisterBlock {
        0x3ff4_8800 as *const _
    }
}
impl Deref for SENS {
    type Target = sens::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SENS::ptr() }
    }
}
#[doc = "SENS"]
pub mod sens;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "RTC_I2C"]
    pub RTC_I2C: RTC_I2C,
    #[doc = "SLC"]
    pub SLC: SLC,
    #[doc = "DPORT"]
    pub DPORT: DPORT,
    #[doc = "RMT"]
    pub RMT: RMT,
    #[doc = "PCNT"]
    pub PCNT: PCNT,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "RTCIO"]
    pub RTCIO: RTCIO,
    #[doc = "LEDC"]
    pub LEDC: LEDC,
    #[doc = "APB_CTRL"]
    pub APB_CTRL: APB_CTRL,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "GPIO_SD"]
    pub GPIO_SD: GPIO_SD,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "SLCHOST"]
    pub SLCHOST: SLCHOST,
    #[doc = "HINF"]
    pub HINF: HINF,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "RTCCNTL"]
    pub RTCCNTL: RTCCNTL,
    #[doc = "SENS"]
    pub SENS: SENS,
}
impl Peripherals {
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            RTC_I2C: RTC_I2C {
                _marker: PhantomData,
            },
            SLC: SLC {
                _marker: PhantomData,
            },
            DPORT: DPORT {
                _marker: PhantomData,
            },
            RMT: RMT {
                _marker: PhantomData,
            },
            PCNT: PCNT {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            RTCIO: RTCIO {
                _marker: PhantomData,
            },
            LEDC: LEDC {
                _marker: PhantomData,
            },
            APB_CTRL: APB_CTRL {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            GPIO_SD: GPIO_SD {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            SLCHOST: SLCHOST {
                _marker: PhantomData,
            },
            HINF: HINF {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            RTCCNTL: RTCCNTL {
                _marker: PhantomData,
            },
            SENS: SENS {
                _marker: PhantomData,
            },
        }
    }
}
