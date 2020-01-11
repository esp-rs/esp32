#[doc = "Reader of register TIME_UPDATE"]
pub type R = crate::R<u32, super::TIME_UPDATE>;
#[doc = "Writer for register TIME_UPDATE"]
pub type W = crate::W<u32, super::TIME_UPDATE>;
#[doc = "Register TIME_UPDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::TIME_UPDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_TIME_UPDATE`"]
pub type RTC_CNTL_TIME_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TIME_UPDATE`"]
pub struct RTC_CNTL_TIME_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TIME_UPDATE_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_TIME_VALID`"]
pub type RTC_CNTL_TIME_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_TIME_VALID`"]
pub struct RTC_CNTL_TIME_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_TIME_VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Set 1: to update register with RTC timer"]
    #[inline(always)]
    pub fn rtc_cntl_time_update(&self) -> RTC_CNTL_TIME_UPDATE_R {
        RTC_CNTL_TIME_UPDATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - To indicate the register is updated"]
    #[inline(always)]
    pub fn rtc_cntl_time_valid(&self) -> RTC_CNTL_TIME_VALID_R {
        RTC_CNTL_TIME_VALID_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Set 1: to update register with RTC timer"]
    #[inline(always)]
    pub fn rtc_cntl_time_update(&mut self) -> RTC_CNTL_TIME_UPDATE_W {
        RTC_CNTL_TIME_UPDATE_W { w: self }
    }
    #[doc = "Bit 30 - To indicate the register is updated"]
    #[inline(always)]
    pub fn rtc_cntl_time_valid(&mut self) -> RTC_CNTL_TIME_VALID_W {
        RTC_CNTL_TIME_VALID_W { w: self }
    }
}
