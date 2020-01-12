#[doc = "Reader of register RTCCALICFG"]
pub type R = crate::R<u32, super::RTCCALICFG>;
#[doc = "Writer for register RTCCALICFG"]
pub type W = crate::W<u32, super::RTCCALICFG>;
#[doc = "Register RTCCALICFG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCALICFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CALI_START`"]
pub type RTC_CALI_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CALI_START`"]
pub struct RTC_CALI_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_START_W<'a> {
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
#[doc = "Reader of field `RTC_CALI_MAX`"]
pub type RTC_CALI_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CALI_MAX`"]
pub struct RTC_CALI_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTC_CALI_RDY`"]
pub type RTC_CALI_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CALI_RDY`"]
pub struct RTC_CALI_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RTC_CALI_CLK_SEL`"]
pub type RTC_CALI_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CALI_CLK_SEL`"]
pub struct RTC_CALI_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `RTC_CALI_START_CYCLING`"]
pub type RTC_CALI_START_CYCLING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CALI_START_CYCLING`"]
pub struct RTC_CALI_START_CYCLING_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CALI_START_CYCLING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cali_start(&self) -> RTC_CALI_START_R {
        RTC_CALI_START_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn rtc_cali_max(&self) -> RTC_CALI_MAX_R {
        RTC_CALI_MAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cali_rdy(&self) -> RTC_CALI_RDY_R {
        RTC_CALI_RDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&self) -> RTC_CALI_CLK_SEL_R {
        RTC_CALI_CLK_SEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&self) -> RTC_CALI_START_CYCLING_R {
        RTC_CALI_START_CYCLING_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_cali_start(&mut self) -> RTC_CALI_START_W {
        RTC_CALI_START_W { w: self }
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn rtc_cali_max(&mut self) -> RTC_CALI_MAX_W {
        RTC_CALI_MAX_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rtc_cali_rdy(&mut self) -> RTC_CALI_RDY_W {
        RTC_CALI_RDY_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rtc_cali_clk_sel(&mut self) -> RTC_CALI_CLK_SEL_W {
        RTC_CALI_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtc_cali_start_cycling(&mut self) -> RTC_CALI_START_CYCLING_W {
        RTC_CALI_START_CYCLING_W { w: self }
    }
}
