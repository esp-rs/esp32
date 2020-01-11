#[doc = "Reader of register CPU_PER_CONF"]
pub type R = crate::R<u32, super::CPU_PER_CONF>;
#[doc = "Writer for register CPU_PER_CONF"]
pub type W = crate::W<u32, super::CPU_PER_CONF>;
#[doc = "Register CPU_PER_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CPU_PER_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_FAST_CLK_RTC_SEL`"]
pub type DPORT_FAST_CLK_RTC_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_FAST_CLK_RTC_SEL`"]
pub struct DPORT_FAST_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_FAST_CLK_RTC_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DPORT_LOWSPEED_CLK_SEL`"]
pub type DPORT_LOWSPEED_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_LOWSPEED_CLK_SEL`"]
pub struct DPORT_LOWSPEED_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_LOWSPEED_CLK_SEL_W<'a> {
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
#[doc = "Reader of field `DPORT_CPUPERIOD_SEL`"]
pub type DPORT_CPUPERIOD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_CPUPERIOD_SEL`"]
pub struct DPORT_CPUPERIOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_CPUPERIOD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_fast_clk_rtc_sel(&self) -> DPORT_FAST_CLK_RTC_SEL_R {
        DPORT_FAST_CLK_RTC_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_lowspeed_clk_sel(&self) -> DPORT_LOWSPEED_CLK_SEL_R {
        DPORT_LOWSPEED_CLK_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_cpuperiod_sel(&self) -> DPORT_CPUPERIOD_SEL_R {
        DPORT_CPUPERIOD_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_fast_clk_rtc_sel(&mut self) -> DPORT_FAST_CLK_RTC_SEL_W {
        DPORT_FAST_CLK_RTC_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_lowspeed_clk_sel(&mut self) -> DPORT_LOWSPEED_CLK_SEL_W {
        DPORT_LOWSPEED_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_cpuperiod_sel(&mut self) -> DPORT_CPUPERIOD_SEL_W {
        DPORT_CPUPERIOD_SEL_W { w: self }
    }
}
