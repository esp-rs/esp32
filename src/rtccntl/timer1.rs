#[doc = "Reader of register TIMER1"]
pub type R = crate::R<u32, super::TIMER1>;
#[doc = "Writer for register TIMER1"]
pub type W = crate::W<u32, super::TIMER1>;
#[doc = "Register TIMER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLL_BUF_WAIT`"]
pub type PLL_BUF_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLL_BUF_WAIT`"]
pub struct PLL_BUF_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BUF_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `XTL_BUF_WAIT`"]
pub type XTL_BUF_WAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XTL_BUF_WAIT`"]
pub struct XTL_BUF_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_BUF_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 14)) | (((value as u32) & 0x03ff) << 14);
        self.w
    }
}
#[doc = "Reader of field `CK8M_WAIT`"]
pub type CK8M_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CK8M_WAIT`"]
pub struct CK8M_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CK8M_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | (((value as u32) & 0xff) << 6);
        self.w
    }
}
#[doc = "Reader of field `CPU_STALL_WAIT`"]
pub type CPU_STALL_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPU_STALL_WAIT`"]
pub struct CPU_STALL_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_STALL_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Reader of field `CPU_STALL_EN`"]
pub type CPU_STALL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_STALL_EN`"]
pub struct CPU_STALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_STALL_EN_W<'a> {
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
    pub fn pll_buf_wait(&self) -> PLL_BUF_WAIT_R {
        PLL_BUF_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - XTAL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn xtl_buf_wait(&self) -> XTL_BUF_WAIT_R {
        XTL_BUF_WAIT_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 6:13 - CK8M wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn ck8m_wait(&self) -> CK8M_WAIT_R {
        CK8M_WAIT_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 1:5 - CPU stall wait cycles in fast_clk_rtc"]
    #[inline(always)]
    pub fn cpu_stall_wait(&self) -> CPU_STALL_WAIT_R {
        CPU_STALL_WAIT_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - CPU stall enable bit"]
    #[inline(always)]
    pub fn cpu_stall_en(&self) -> CPU_STALL_EN_R {
        CPU_STALL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - PLL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn pll_buf_wait(&mut self) -> PLL_BUF_WAIT_W {
        PLL_BUF_WAIT_W { w: self }
    }
    #[doc = "Bits 14:23 - XTAL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn xtl_buf_wait(&mut self) -> XTL_BUF_WAIT_W {
        XTL_BUF_WAIT_W { w: self }
    }
    #[doc = "Bits 6:13 - CK8M wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn ck8m_wait(&mut self) -> CK8M_WAIT_W {
        CK8M_WAIT_W { w: self }
    }
    #[doc = "Bits 1:5 - CPU stall wait cycles in fast_clk_rtc"]
    #[inline(always)]
    pub fn cpu_stall_wait(&mut self) -> CPU_STALL_WAIT_W {
        CPU_STALL_WAIT_W { w: self }
    }
    #[doc = "Bit 0 - CPU stall enable bit"]
    #[inline(always)]
    pub fn cpu_stall_en(&mut self) -> CPU_STALL_EN_W {
        CPU_STALL_EN_W { w: self }
    }
}
