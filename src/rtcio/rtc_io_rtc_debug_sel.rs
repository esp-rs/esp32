#[doc = "Reader of register RTC_IO_RTC_DEBUG_SEL"]
pub type R = crate::R<u32, super::RTC_IO_RTC_DEBUG_SEL>;
#[doc = "Writer for register RTC_IO_RTC_DEBUG_SEL"]
pub type W = crate::W<u32, super::RTC_IO_RTC_DEBUG_SEL>;
#[doc = "Register RTC_IO_RTC_DEBUG_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IO_RTC_DEBUG_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_IO_DEBUG_12M_NO_GATING`"]
pub type RTC_IO_DEBUG_12M_NO_GATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_IO_DEBUG_12M_NO_GATING`"]
pub struct RTC_IO_DEBUG_12M_NO_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_DEBUG_12M_NO_GATING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_DEBUG_SEL4`"]
pub type RTC_IO_DEBUG_SEL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_DEBUG_SEL4`"]
pub struct RTC_IO_DEBUG_SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_DEBUG_SEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_DEBUG_SEL3`"]
pub type RTC_IO_DEBUG_SEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_DEBUG_SEL3`"]
pub struct RTC_IO_DEBUG_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_DEBUG_SEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_DEBUG_SEL2`"]
pub type RTC_IO_DEBUG_SEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_DEBUG_SEL2`"]
pub struct RTC_IO_DEBUG_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_DEBUG_SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_DEBUG_SEL1`"]
pub type RTC_IO_DEBUG_SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_DEBUG_SEL1`"]
pub struct RTC_IO_DEBUG_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_DEBUG_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `RTC_IO_DEBUG_SEL0`"]
pub type RTC_IO_DEBUG_SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_IO_DEBUG_SEL0`"]
pub struct RTC_IO_DEBUG_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IO_DEBUG_SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_io_debug_12m_no_gating(&self) -> RTC_IO_DEBUG_12M_NO_GATING_R {
        RTC_IO_DEBUG_12M_NO_GATING_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn rtc_io_debug_sel4(&self) -> RTC_IO_DEBUG_SEL4_R {
        RTC_IO_DEBUG_SEL4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn rtc_io_debug_sel3(&self) -> RTC_IO_DEBUG_SEL3_R {
        RTC_IO_DEBUG_SEL3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn rtc_io_debug_sel2(&self) -> RTC_IO_DEBUG_SEL2_R {
        RTC_IO_DEBUG_SEL2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rtc_io_debug_sel1(&self) -> RTC_IO_DEBUG_SEL1_R {
        RTC_IO_DEBUG_SEL1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rtc_io_debug_sel0(&self) -> RTC_IO_DEBUG_SEL0_R {
        RTC_IO_DEBUG_SEL0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rtc_io_debug_12m_no_gating(&mut self) -> RTC_IO_DEBUG_12M_NO_GATING_W {
        RTC_IO_DEBUG_12M_NO_GATING_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn rtc_io_debug_sel4(&mut self) -> RTC_IO_DEBUG_SEL4_W {
        RTC_IO_DEBUG_SEL4_W { w: self }
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn rtc_io_debug_sel3(&mut self) -> RTC_IO_DEBUG_SEL3_W {
        RTC_IO_DEBUG_SEL3_W { w: self }
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn rtc_io_debug_sel2(&mut self) -> RTC_IO_DEBUG_SEL2_W {
        RTC_IO_DEBUG_SEL2_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rtc_io_debug_sel1(&mut self) -> RTC_IO_DEBUG_SEL1_W {
        RTC_IO_DEBUG_SEL1_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rtc_io_debug_sel0(&mut self) -> RTC_IO_DEBUG_SEL0_W {
        RTC_IO_DEBUG_SEL0_W { w: self }
    }
}
