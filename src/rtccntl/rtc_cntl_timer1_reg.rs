#[doc = "Reader of register RTC_CNTL_TIMER1_REG"]
pub type R = crate::R<u32, super::RTC_CNTL_TIMER1_REG>;
#[doc = "Writer for register RTC_CNTL_TIMER1_REG"]
pub type W = crate::W<u32, super::RTC_CNTL_TIMER1_REG>;
#[doc = "Register RTC_CNTL_TIMER1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_TIMER1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_PLL_BUF_WAIT`"]
pub type RTC_CNTL_PLL_BUF_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_PLL_BUF_WAIT`"]
pub struct RTC_CNTL_PLL_BUF_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_PLL_BUF_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_XTL_BUF_WAIT`"]
pub type RTC_CNTL_XTL_BUF_WAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_CNTL_XTL_BUF_WAIT`"]
pub struct RTC_CNTL_XTL_BUF_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTL_BUF_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 14)) | (((value as u32) & 0x03ff) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_CK8M_WAIT`"]
pub type RTC_CNTL_CK8M_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_CK8M_WAIT`"]
pub struct RTC_CNTL_CK8M_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CK8M_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | (((value as u32) & 0xff) << 6);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_CPU_STALL_WAIT`"]
pub type RTC_CNTL_CPU_STALL_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_CPU_STALL_WAIT`"]
pub struct RTC_CNTL_CPU_STALL_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CPU_STALL_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_CPU_STALL_EN`"]
pub type RTC_CNTL_CPU_STALL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CPU_STALL_EN`"]
pub struct RTC_CNTL_CPU_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CPU_STALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - PLL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_pll_buf_wait(&self) -> RTC_CNTL_PLL_BUF_WAIT_R {
        RTC_CNTL_PLL_BUF_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - XTAL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_buf_wait(&self) -> RTC_CNTL_XTL_BUF_WAIT_R {
        RTC_CNTL_XTL_BUF_WAIT_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 6:13 - CK8M wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_wait(&self) -> RTC_CNTL_CK8M_WAIT_R {
        RTC_CNTL_CK8M_WAIT_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 1:5 - CPU stall wait cycles in fast_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_stall_wait(&self) -> RTC_CNTL_CPU_STALL_WAIT_R {
        RTC_CNTL_CPU_STALL_WAIT_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - CPU stall enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_stall_en(&self) -> RTC_CNTL_CPU_STALL_EN_R {
        RTC_CNTL_CPU_STALL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - PLL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_pll_buf_wait(&mut self) -> RTC_CNTL_PLL_BUF_WAIT_W {
        RTC_CNTL_PLL_BUF_WAIT_W { w: self }
    }
    #[doc = "Bits 14:23 - XTAL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_xtl_buf_wait(&mut self) -> RTC_CNTL_XTL_BUF_WAIT_W {
        RTC_CNTL_XTL_BUF_WAIT_W { w: self }
    }
    #[doc = "Bits 6:13 - CK8M wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_wait(&mut self) -> RTC_CNTL_CK8M_WAIT_W {
        RTC_CNTL_CK8M_WAIT_W { w: self }
    }
    #[doc = "Bits 1:5 - CPU stall wait cycles in fast_clk_rtc"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_stall_wait(&mut self) -> RTC_CNTL_CPU_STALL_WAIT_W {
        RTC_CNTL_CPU_STALL_WAIT_W { w: self }
    }
    #[doc = "Bit 0 - CPU stall enable bit"]
    #[inline(always)]
    pub fn rtc_cntl_cpu_stall_en(&mut self) -> RTC_CNTL_CPU_STALL_EN_W {
        RTC_CNTL_CPU_STALL_EN_W { w: self }
    }
}
