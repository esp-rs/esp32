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
#[doc = "PWM2"]
pub struct PWM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2 {}
impl PWM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcpwm::RegisterBlock {
        0x3ff6_f000 as *const _
    }
}
impl Deref for PWM2 {
    type Target = mcpwm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM2::ptr() }
    }
}
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
#[doc = "SPI"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "SPI"]
pub mod spi;
#[doc = "PWM3"]
pub struct PWM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM3 {}
impl PWM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcpwm::RegisterBlock {
        0x3ff7_0000 as *const _
    }
}
impl Deref for PWM3 {
    type Target = mcpwm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM3::ptr() }
    }
}
#[doc = "I2C"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        0 as *const _
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C::ptr() }
    }
}
#[doc = "I2C"]
pub mod i2c;
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
#[doc = "SPI2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0x3ff6_4000 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "UHCI1"]
pub struct UHCI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UHCI1 {}
impl UHCI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uhci::RegisterBlock {
        0x3ff4_c000 as *const _
    }
}
impl Deref for UHCI1 {
    type Target = uhci::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UHCI1::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        0x3ff6_7000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "PWM0"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcpwm::RegisterBlock {
        0x3ff5_e000 as *const _
    }
}
impl Deref for PWM0 {
    type Target = mcpwm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "TIMG"]
pub struct TIMG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMG {}
impl TIMG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timg::RegisterBlock {
        0 as *const _
    }
}
impl Deref for TIMG {
    type Target = timg::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMG::ptr() }
    }
}
#[doc = "TIMG"]
pub mod timg;
#[doc = "PWM1"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcpwm::RegisterBlock {
        0x3ff6_c000 as *const _
    }
}
impl Deref for PWM1 {
    type Target = mcpwm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM1::ptr() }
    }
}
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
#[doc = "UHCI0"]
pub struct UHCI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UHCI0 {}
impl UHCI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uhci::RegisterBlock {
        0x3ff5_4000 as *const _
    }
}
impl Deref for UHCI0 {
    type Target = uhci::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UHCI0::ptr() }
    }
}
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0x3ff4_2000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        0x3ff5_3000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
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
#[doc = "SPI0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0x3ff4_3000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "SPI3"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0x3ff6_5000 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
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
#[doc = "MCPWM"]
pub struct MCPWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCPWM {}
impl MCPWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcpwm::RegisterBlock {
        0 as *const _
    }
}
impl Deref for MCPWM {
    type Target = mcpwm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCPWM::ptr() }
    }
}
#[doc = "MCPWM"]
pub mod mcpwm;
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
#[doc = "UHCI"]
pub struct UHCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UHCI {}
impl UHCI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uhci::RegisterBlock {
        0 as *const _
    }
}
impl Deref for UHCI {
    type Target = uhci::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UHCI::ptr() }
    }
}
#[doc = "UHCI"]
pub mod uhci;
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
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PWM2"]
    pub PWM2: PWM2,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "SENS"]
    pub SENS: SENS,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "PWM3"]
    pub PWM3: PWM3,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "GPIO_SD"]
    pub GPIO_SD: GPIO_SD,
    #[doc = "PCNT"]
    pub PCNT: PCNT,
    #[doc = "SLC"]
    pub SLC: SLC,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "UHCI1"]
    pub UHCI1: UHCI1,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "TIMG"]
    pub TIMG: TIMG,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "RTCIO"]
    pub RTCIO: RTCIO,
    #[doc = "RTCCNTL"]
    pub RTCCNTL: RTCCNTL,
    #[doc = "RTC_I2C"]
    pub RTC_I2C: RTC_I2C,
    #[doc = "UHCI0"]
    pub UHCI0: UHCI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "APB_CTRL"]
    pub APB_CTRL: APB_CTRL,
    #[doc = "RMT"]
    pub RMT: RMT,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "LEDC"]
    pub LEDC: LEDC,
    #[doc = "MCPWM"]
    pub MCPWM: MCPWM,
    #[doc = "SLCHOST"]
    pub SLCHOST: SLCHOST,
    #[doc = "DPORT"]
    pub DPORT: DPORT,
    #[doc = "UHCI"]
    pub UHCI: UHCI,
    #[doc = "HINF"]
    pub HINF: HINF,
}
impl Peripherals {
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PWM2: PWM2 {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            SENS: SENS {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            PWM3: PWM3 {
                _marker: PhantomData,
            },
            I2C: I2C {
                _marker: PhantomData,
            },
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            GPIO_SD: GPIO_SD {
                _marker: PhantomData,
            },
            PCNT: PCNT {
                _marker: PhantomData,
            },
            SLC: SLC {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            UHCI1: UHCI1 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            TIMG: TIMG {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            RTCIO: RTCIO {
                _marker: PhantomData,
            },
            RTCCNTL: RTCCNTL {
                _marker: PhantomData,
            },
            RTC_I2C: RTC_I2C {
                _marker: PhantomData,
            },
            UHCI0: UHCI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            APB_CTRL: APB_CTRL {
                _marker: PhantomData,
            },
            RMT: RMT {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            LEDC: LEDC {
                _marker: PhantomData,
            },
            MCPWM: MCPWM {
                _marker: PhantomData,
            },
            SLCHOST: SLCHOST {
                _marker: PhantomData,
            },
            DPORT: DPORT {
                _marker: PhantomData,
            },
            UHCI: UHCI {
                _marker: PhantomData,
            },
            HINF: HINF {
                _marker: PhantomData,
            },
        }
    }
}
