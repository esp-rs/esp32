#[doc = "Reader of register TIMG_RTCCALICFG_REG"]
pub type R = crate::R<u32, super::TIMG_RTCCALICFG_REG>;
#[doc = "Writer for register TIMG_RTCCALICFG_REG"]
pub type W = crate::W<u32, super::TIMG_RTCCALICFG_REG>;
#[doc = "Register TIMG_RTCCALICFG_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_RTCCALICFG_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_RTC_CALI_START`"]
pub type TIMG_RTC_CALI_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_RTC_CALI_START`"]
pub struct TIMG_RTC_CALI_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_RTC_CALI_START_W<'a> {
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
#[doc = "Reader of field `TIMG_RTC_CALI_MAX`"]
pub type TIMG_RTC_CALI_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMG_RTC_CALI_MAX`"]
pub struct TIMG_RTC_CALI_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_RTC_CALI_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TIMG_RTC_CALI_RDY`"]
pub type TIMG_RTC_CALI_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_RTC_CALI_RDY`"]
pub struct TIMG_RTC_CALI_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_RTC_CALI_RDY_W<'a> {
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
#[doc = "Reader of field `TIMG_RTC_CALI_CLK_SEL`"]
pub type TIMG_RTC_CALI_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMG_RTC_CALI_CLK_SEL`"]
pub struct TIMG_RTC_CALI_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_RTC_CALI_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `TIMG_RTC_CALI_START_CYCLING`"]
pub type TIMG_RTC_CALI_START_CYCLING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_RTC_CALI_START_CYCLING`"]
pub struct TIMG_RTC_CALI_START_CYCLING_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_RTC_CALI_START_CYCLING_W<'a> {
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
    pub fn timg_rtc_cali_start(&self) -> TIMG_RTC_CALI_START_R {
        TIMG_RTC_CALI_START_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn timg_rtc_cali_max(&self) -> TIMG_RTC_CALI_MAX_R {
        TIMG_RTC_CALI_MAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timg_rtc_cali_rdy(&self) -> TIMG_RTC_CALI_RDY_R {
        TIMG_RTC_CALI_RDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn timg_rtc_cali_clk_sel(&self) -> TIMG_RTC_CALI_CLK_SEL_R {
        TIMG_RTC_CALI_CLK_SEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn timg_rtc_cali_start_cycling(&self) -> TIMG_RTC_CALI_START_CYCLING_R {
        TIMG_RTC_CALI_START_CYCLING_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn timg_rtc_cali_start(&mut self) -> TIMG_RTC_CALI_START_W {
        TIMG_RTC_CALI_START_W { w: self }
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn timg_rtc_cali_max(&mut self) -> TIMG_RTC_CALI_MAX_W {
        TIMG_RTC_CALI_MAX_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timg_rtc_cali_rdy(&mut self) -> TIMG_RTC_CALI_RDY_W {
        TIMG_RTC_CALI_RDY_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn timg_rtc_cali_clk_sel(&mut self) -> TIMG_RTC_CALI_CLK_SEL_W {
        TIMG_RTC_CALI_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn timg_rtc_cali_start_cycling(&mut self) -> TIMG_RTC_CALI_START_CYCLING_W {
        TIMG_RTC_CALI_START_CYCLING_W { w: self }
    }
}
