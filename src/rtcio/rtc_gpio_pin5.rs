#[doc = "Reader of register RTC_GPIO_PIN5"]
pub type R = crate::R<u32, super::RTC_GPIO_PIN5>;
#[doc = "Writer for register RTC_GPIO_PIN5"]
pub type W = crate::W<u32, super::RTC_GPIO_PIN5>;
#[doc = "Register RTC_GPIO_PIN5 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_GPIO_PIN5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_GPIO_PIN5_WAKEUP_ENABLE`"]
pub type RTC_GPIO_PIN5_WAKEUP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_GPIO_PIN5_WAKEUP_ENABLE`"]
pub struct RTC_GPIO_PIN5_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN5_WAKEUP_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `RTC_GPIO_PIN5_INT_TYPE`"]
pub type RTC_GPIO_PIN5_INT_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_GPIO_PIN5_INT_TYPE`"]
pub struct RTC_GPIO_PIN5_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN5_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTC_GPIO_PIN5_PAD_DRIVER`"]
pub type RTC_GPIO_PIN5_PAD_DRIVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_GPIO_PIN5_PAD_DRIVER`"]
pub struct RTC_GPIO_PIN5_PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_GPIO_PIN5_PAD_DRIVER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - GPIO wake up enable only available in light sleep"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_wakeup_enable(&self) -> RTC_GPIO_PIN5_WAKEUP_ENABLE_R {
        RTC_GPIO_PIN5_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable if set to 1: rising edge trigger if set to 2: falling edge trigger if set to 3: any edge trigger if set to 4: low level trigger if set to 5: high level trigger"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_int_type(&self) -> RTC_GPIO_PIN5_INT_TYPE_R {
        RTC_GPIO_PIN5_INT_TYPE_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bit 2 - if set to 0: normal output if set to 1: open drain"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_pad_driver(&self) -> RTC_GPIO_PIN5_PAD_DRIVER_R {
        RTC_GPIO_PIN5_PAD_DRIVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - GPIO wake up enable only available in light sleep"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_wakeup_enable(&mut self) -> RTC_GPIO_PIN5_WAKEUP_ENABLE_W {
        RTC_GPIO_PIN5_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable if set to 1: rising edge trigger if set to 2: falling edge trigger if set to 3: any edge trigger if set to 4: low level trigger if set to 5: high level trigger"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_int_type(&mut self) -> RTC_GPIO_PIN5_INT_TYPE_W {
        RTC_GPIO_PIN5_INT_TYPE_W { w: self }
    }
    #[doc = "Bit 2 - if set to 0: normal output if set to 1: open drain"]
    #[inline(always)]
    pub fn rtc_gpio_pin5_pad_driver(&mut self) -> RTC_GPIO_PIN5_PAD_DRIVER_W {
        RTC_GPIO_PIN5_PAD_DRIVER_W { w: self }
    }
}
