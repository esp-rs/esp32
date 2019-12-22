#[doc = "Reader of register RTC_CNTL_CLK_CONF_REG"]
pub type R = crate::R<u32, super::RTC_CNTL_CLK_CONF_REG>;
#[doc = "Writer for register RTC_CNTL_CLK_CONF_REG"]
pub type W = crate::W<u32, super::RTC_CNTL_CLK_CONF_REG>;
#[doc = "Register RTC_CNTL_CLK_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNTL_CLK_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_CNTL_ANA_CLK_RTC_SEL`"]
pub type RTC_CNTL_ANA_CLK_RTC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_ANA_CLK_RTC_SEL`"]
pub struct RTC_CNTL_ANA_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ANA_CLK_RTC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_FAST_CLK_RTC_SEL`"]
pub type RTC_CNTL_FAST_CLK_RTC_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_FAST_CLK_RTC_SEL`"]
pub struct RTC_CNTL_FAST_CLK_RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_FAST_CLK_RTC_SEL_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_SOC_CLK_SEL`"]
pub type RTC_CNTL_SOC_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_SOC_CLK_SEL`"]
pub struct RTC_CNTL_SOC_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_SOC_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_CK8M_FORCE_PU`"]
pub type RTC_CNTL_CK8M_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CK8M_FORCE_PU`"]
pub struct RTC_CNTL_CK8M_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CK8M_FORCE_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_CK8M_FORCE_PD`"]
pub type RTC_CNTL_CK8M_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CK8M_FORCE_PD`"]
pub struct RTC_CNTL_CK8M_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CK8M_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_CK8M_DFREQ`"]
pub type RTC_CNTL_CK8M_DFREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_CK8M_DFREQ`"]
pub struct RTC_CNTL_CK8M_DFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CK8M_DFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 17)) | (((value as u32) & 0xff) << 17);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_CK8M_FORCE_NOGATING`"]
pub type RTC_CNTL_CK8M_FORCE_NOGATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CK8M_FORCE_NOGATING`"]
pub struct RTC_CNTL_CK8M_FORCE_NOGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CK8M_FORCE_NOGATING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_XTAL_FORCE_NOGATING`"]
pub type RTC_CNTL_XTAL_FORCE_NOGATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_XTAL_FORCE_NOGATING`"]
pub struct RTC_CNTL_XTAL_FORCE_NOGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_XTAL_FORCE_NOGATING_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_CK8M_DIV_SEL`"]
pub type RTC_CNTL_CK8M_DIV_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_CK8M_DIV_SEL`"]
pub struct RTC_CNTL_CK8M_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CK8M_DIV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_CK8M_DFREQ_FORCE`"]
pub type RTC_CNTL_CK8M_DFREQ_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_CK8M_DFREQ_FORCE`"]
pub struct RTC_CNTL_CK8M_DFREQ_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CK8M_DFREQ_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DIG_CLK8M_EN`"]
pub type RTC_CNTL_DIG_CLK8M_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DIG_CLK8M_EN`"]
pub struct RTC_CNTL_DIG_CLK8M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DIG_CLK8M_EN_W<'a> {
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
#[doc = "Reader of field `RTC_CNTL_DIG_CLK8M_D256_EN`"]
pub type RTC_CNTL_DIG_CLK8M_D256_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DIG_CLK8M_D256_EN`"]
pub struct RTC_CNTL_DIG_CLK8M_D256_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DIG_CLK8M_D256_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_DIG_XTAL32K_EN`"]
pub type RTC_CNTL_DIG_XTAL32K_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_DIG_XTAL32K_EN`"]
pub struct RTC_CNTL_DIG_XTAL32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_DIG_XTAL32K_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_ENB_CK8M_DIV`"]
pub type RTC_CNTL_ENB_CK8M_DIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ENB_CK8M_DIV`"]
pub struct RTC_CNTL_ENB_CK8M_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ENB_CK8M_DIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_ENB_CK8M`"]
pub type RTC_CNTL_ENB_CK8M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_CNTL_ENB_CK8M`"]
pub struct RTC_CNTL_ENB_CK8M_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_ENB_CK8M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RTC_CNTL_CK8M_DIV`"]
pub type RTC_CNTL_CK8M_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CNTL_CK8M_DIV`"]
pub struct RTC_CNTL_CK8M_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CNTL_CK8M_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
    #[inline(always)]
    pub fn rtc_cntl_ana_clk_rtc_sel(&self) -> RTC_CNTL_ANA_CLK_RTC_SEL_R {
        RTC_CNTL_ANA_CLK_RTC_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
    #[inline(always)]
    pub fn rtc_cntl_fast_clk_rtc_sel(&self) -> RTC_CNTL_FAST_CLK_RTC_SEL_R {
        RTC_CNTL_FAST_CLK_RTC_SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
    #[inline(always)]
    pub fn rtc_cntl_soc_clk_sel(&self) -> RTC_CNTL_SOC_CLK_SEL_R {
        RTC_CNTL_SOC_CLK_SEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_force_pu(&self) -> RTC_CNTL_CK8M_FORCE_PU_R {
        RTC_CNTL_CK8M_FORCE_PU_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_force_pd(&self) -> RTC_CNTL_CK8M_FORCE_PD_R {
        RTC_CNTL_CK8M_FORCE_PD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_dfreq(&self) -> RTC_CNTL_CK8M_DFREQ_R {
        RTC_CNTL_CK8M_DFREQ_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_force_nogating(&self) -> RTC_CNTL_CK8M_FORCE_NOGATING_R {
        RTC_CNTL_CK8M_FORCE_NOGATING_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    pub fn rtc_cntl_xtal_force_nogating(&self) -> RTC_CNTL_XTAL_FORCE_NOGATING_R {
        RTC_CNTL_XTAL_FORCE_NOGATING_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_div_sel(&self) -> RTC_CNTL_CK8M_DIV_SEL_R {
        RTC_CNTL_CK8M_DIV_SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_dfreq_force(&self) -> RTC_CNTL_CK8M_DFREQ_FORCE_R {
        RTC_CNTL_CK8M_DFREQ_FORCE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn rtc_cntl_dig_clk8m_en(&self) -> RTC_CNTL_DIG_CLK8M_EN_R {
        RTC_CNTL_DIG_CLK8M_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn rtc_cntl_dig_clk8m_d256_en(&self) -> RTC_CNTL_DIG_CLK8M_D256_EN_R {
        RTC_CNTL_DIG_CLK8M_D256_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn rtc_cntl_dig_xtal32k_en(&self) -> RTC_CNTL_DIG_XTAL32K_EN_R {
        RTC_CNTL_DIG_XTAL32K_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
    #[inline(always)]
    pub fn rtc_cntl_enb_ck8m_div(&self) -> RTC_CNTL_ENB_CK8M_DIV_R {
        RTC_CNTL_ENB_CK8M_DIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    pub fn rtc_cntl_enb_ck8m(&self) -> RTC_CNTL_ENB_CK8M_R {
        RTC_CNTL_ENB_CK8M_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_div(&self) -> RTC_CNTL_CK8M_DIV_R {
        RTC_CNTL_CK8M_DIV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - slow_clk_rtc sel. 0: SLOW_CK 1: CK_XTAL_32K 2: CK8M_D256_OUT"]
    #[inline(always)]
    pub fn rtc_cntl_ana_clk_rtc_sel(&mut self) -> RTC_CNTL_ANA_CLK_RTC_SEL_W {
        RTC_CNTL_ANA_CLK_RTC_SEL_W { w: self }
    }
    #[doc = "Bit 29 - fast_clk_rtc sel. 0: XTAL div 4 1: CK8M"]
    #[inline(always)]
    pub fn rtc_cntl_fast_clk_rtc_sel(&mut self) -> RTC_CNTL_FAST_CLK_RTC_SEL_W {
        RTC_CNTL_FAST_CLK_RTC_SEL_W { w: self }
    }
    #[doc = "Bits 27:28 - SOC clock sel. 0: XTAL 1: PLL 2: CK8M 3: APLL"]
    #[inline(always)]
    pub fn rtc_cntl_soc_clk_sel(&mut self) -> RTC_CNTL_SOC_CLK_SEL_W {
        RTC_CNTL_SOC_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 26 - CK8M force power up"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_force_pu(&mut self) -> RTC_CNTL_CK8M_FORCE_PU_W {
        RTC_CNTL_CK8M_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 25 - CK8M force power down"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_force_pd(&mut self) -> RTC_CNTL_CK8M_FORCE_PD_W {
        RTC_CNTL_CK8M_FORCE_PD_W { w: self }
    }
    #[doc = "Bits 17:24 - CK8M_DFREQ"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_dfreq(&mut self) -> RTC_CNTL_CK8M_DFREQ_W {
        RTC_CNTL_CK8M_DFREQ_W { w: self }
    }
    #[doc = "Bit 16 - CK8M force no gating during sleep"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_force_nogating(&mut self) -> RTC_CNTL_CK8M_FORCE_NOGATING_W {
        RTC_CNTL_CK8M_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bit 15 - XTAL force no gating during sleep"]
    #[inline(always)]
    pub fn rtc_cntl_xtal_force_nogating(&mut self) -> RTC_CNTL_XTAL_FORCE_NOGATING_W {
        RTC_CNTL_XTAL_FORCE_NOGATING_W { w: self }
    }
    #[doc = "Bits 12:14 - divider = reg_ck8m_div_sel + 1"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_div_sel(&mut self) -> RTC_CNTL_CK8M_DIV_SEL_W {
        RTC_CNTL_CK8M_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_dfreq_force(&mut self) -> RTC_CNTL_CK8M_DFREQ_FORCE_W {
        RTC_CNTL_CK8M_DFREQ_FORCE_W { w: self }
    }
    #[doc = "Bit 10 - enable CK8M for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn rtc_cntl_dig_clk8m_en(&mut self) -> RTC_CNTL_DIG_CLK8M_EN_W {
        RTC_CNTL_DIG_CLK8M_EN_W { w: self }
    }
    #[doc = "Bit 9 - enable CK8M_D256_OUT for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn rtc_cntl_dig_clk8m_d256_en(&mut self) -> RTC_CNTL_DIG_CLK8M_D256_EN_W {
        RTC_CNTL_DIG_CLK8M_D256_EN_W { w: self }
    }
    #[doc = "Bit 8 - enable CK_XTAL_32K for digital core (no relationship with RTC core)"]
    #[inline(always)]
    pub fn rtc_cntl_dig_xtal32k_en(&mut self) -> RTC_CNTL_DIG_XTAL32K_EN_W {
        RTC_CNTL_DIG_XTAL32K_EN_W { w: self }
    }
    #[doc = "Bit 7 - 1: CK8M_D256_OUT is actually CK8M 0: CK8M_D256_OUT is CK8M divided by 256"]
    #[inline(always)]
    pub fn rtc_cntl_enb_ck8m_div(&mut self) -> RTC_CNTL_ENB_CK8M_DIV_W {
        RTC_CNTL_ENB_CK8M_DIV_W { w: self }
    }
    #[doc = "Bit 6 - disable CK8M and CK8M_D256_OUT"]
    #[inline(always)]
    pub fn rtc_cntl_enb_ck8m(&mut self) -> RTC_CNTL_ENB_CK8M_W {
        RTC_CNTL_ENB_CK8M_W { w: self }
    }
    #[doc = "Bits 4:5 - CK8M_D256_OUT divider. 00: div128 01: div256 10: div512 11: div1024."]
    #[inline(always)]
    pub fn rtc_cntl_ck8m_div(&mut self) -> RTC_CNTL_CK8M_DIV_W {
        RTC_CNTL_CK8M_DIV_W { w: self }
    }
}
