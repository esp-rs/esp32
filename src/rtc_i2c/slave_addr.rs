#[doc = "Reader of register SLAVE_ADDR"]
pub type R = crate::R<u32, super::SLAVE_ADDR>;
#[doc = "Writer for register SLAVE_ADDR"]
pub type W = crate::W<u32, super::SLAVE_ADDR>;
#[doc = "Register SLAVE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SLAVE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_SLAVE_ADDR_10BIT`"]
pub type RTC_I2C_SLAVE_ADDR_10BIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SLAVE_ADDR_10BIT`"]
pub struct RTC_I2C_SLAVE_ADDR_10BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SLAVE_ADDR_10BIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_SLAVE_ADDR`"]
pub type RTC_I2C_SLAVE_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_I2C_SLAVE_ADDR`"]
pub struct RTC_I2C_SLAVE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SLAVE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Set if local slave address is 10-bit"]
    #[inline(always)]
    pub fn rtc_i2c_slave_addr_10bit(&self) -> RTC_I2C_SLAVE_ADDR_10BIT_R {
        RTC_I2C_SLAVE_ADDR_10BIT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:14 - local slave address"]
    #[inline(always)]
    pub fn rtc_i2c_slave_addr(&self) -> RTC_I2C_SLAVE_ADDR_R {
        RTC_I2C_SLAVE_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Set if local slave address is 10-bit"]
    #[inline(always)]
    pub fn rtc_i2c_slave_addr_10bit(&mut self) -> RTC_I2C_SLAVE_ADDR_10BIT_W {
        RTC_I2C_SLAVE_ADDR_10BIT_W { w: self }
    }
    #[doc = "Bits 0:14 - local slave address"]
    #[inline(always)]
    pub fn rtc_i2c_slave_addr(&mut self) -> RTC_I2C_SLAVE_ADDR_W {
        RTC_I2C_SLAVE_ADDR_W { w: self }
    }
}
