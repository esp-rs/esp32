#[doc = "Reader of register RTC_CNTL_TEST_MUX"]
pub type R = crate::R<u32, super::RTC_CNTL_TEST_MUX>;
#[doc = "Writer for register RTC_CNTL_TEST_MUX"]
pub type W = crate::W<u32, super::RTC_CNTL_TEST_MUX>;
#[doc = "Register RTC_CNTL_TEST_MUX `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_TEST_MUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_DTEST_RTC`"]
pub type RTC_CNTL_DTEST_RTC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_DTEST_RTC`"]
pub struct RTC_CNTL_DTEST_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DTEST_RTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_ENT_RTC`"]
pub type RTC_CNTL_ENT_RTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ENT_RTC`"]
pub struct RTC_CNTL_ENT_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ENT_RTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    pub fn rtc_cntl_dtest_rtc(&self) -> RTC_CNTL_DTEST_RTC_R {
        RTC_CNTL_DTEST_RTC_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    pub fn rtc_cntl_ent_rtc(&self) -> RTC_CNTL_ENT_RTC_R {
        RTC_CNTL_ENT_RTC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    pub fn rtc_cntl_dtest_rtc(&mut self) -> RTC_CNTL_DTEST_RTC_W {
        RTC_CNTL_DTEST_RTC_W { w: self }
    }
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    pub fn rtc_cntl_ent_rtc(&mut self) -> RTC_CNTL_ENT_RTC_W {
        RTC_CNTL_ENT_RTC_W { w: self }
    }
}
