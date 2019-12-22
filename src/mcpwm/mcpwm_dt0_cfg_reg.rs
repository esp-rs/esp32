#[doc = "Reader of register MCPWM_DT0_CFG_REG"]
pub type R = crate::R<u32, super::MCPWM_DT0_CFG_REG>;
#[doc = "Writer for register MCPWM_DT0_CFG_REG"]
pub type W = crate::W<u32, super::MCPWM_DT0_CFG_REG>;
#[doc = "Register MCPWM_DT0_CFG_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_DT0_CFG_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_DT0_CLK_SEL`"]
pub type MCPWM_DT0_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_CLK_SEL`"]
pub struct MCPWM_DT0_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_DT0_B_OUTBYPASS`"]
pub type MCPWM_DT0_B_OUTBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_B_OUTBYPASS`"]
pub struct MCPWM_DT0_B_OUTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_B_OUTBYPASS_W<'a> {
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
#[doc = "Reader of field `MCPWM_DT0_A_OUTBYPASS`"]
pub type MCPWM_DT0_A_OUTBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_A_OUTBYPASS`"]
pub struct MCPWM_DT0_A_OUTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_A_OUTBYPASS_W<'a> {
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
#[doc = "Reader of field `MCPWM_DT0_FED_OUTINVERT`"]
pub type MCPWM_DT0_FED_OUTINVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_FED_OUTINVERT`"]
pub struct MCPWM_DT0_FED_OUTINVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_FED_OUTINVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_DT0_RED_OUTINVERT`"]
pub type MCPWM_DT0_RED_OUTINVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_RED_OUTINVERT`"]
pub struct MCPWM_DT0_RED_OUTINVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_RED_OUTINVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_DT0_FED_INSEL`"]
pub type MCPWM_DT0_FED_INSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_FED_INSEL`"]
pub struct MCPWM_DT0_FED_INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_FED_INSEL_W<'a> {
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
#[doc = "Reader of field `MCPWM_DT0_RED_INSEL`"]
pub type MCPWM_DT0_RED_INSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_RED_INSEL`"]
pub struct MCPWM_DT0_RED_INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_RED_INSEL_W<'a> {
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
#[doc = "Reader of field `MCPWM_DT0_B_OUTSWAP`"]
pub type MCPWM_DT0_B_OUTSWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_B_OUTSWAP`"]
pub struct MCPWM_DT0_B_OUTSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_B_OUTSWAP_W<'a> {
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
#[doc = "Reader of field `MCPWM_DT0_A_OUTSWAP`"]
pub type MCPWM_DT0_A_OUTSWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_A_OUTSWAP`"]
pub struct MCPWM_DT0_A_OUTSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_A_OUTSWAP_W<'a> {
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
#[doc = "Reader of field `MCPWM_DT0_DEB_MODE`"]
pub type MCPWM_DT0_DEB_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_DT0_DEB_MODE`"]
pub struct MCPWM_DT0_DEB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_DEB_MODE_W<'a> {
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
#[doc = "Reader of field `MCPWM_DT0_RED_UPMETHOD`"]
pub type MCPWM_DT0_RED_UPMETHOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_DT0_RED_UPMETHOD`"]
pub struct MCPWM_DT0_RED_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_RED_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_DT0_FED_UPMETHOD`"]
pub type MCPWM_DT0_FED_UPMETHOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_DT0_FED_UPMETHOD`"]
pub struct MCPWM_DT0_FED_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DT0_FED_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Dead time generator 0 clock selection. 0: PWM_clk 1: PT_clk"]
    #[inline(always)]
    pub fn mcpwm_dt0_clk_sel(&self) -> MCPWM_DT0_CLK_SEL_R {
        MCPWM_DT0_CLK_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_b_outbypass(&self) -> MCPWM_DT0_B_OUTBYPASS_R {
        MCPWM_DT0_B_OUTBYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_a_outbypass(&self) -> MCPWM_DT0_A_OUTBYPASS_R {
        MCPWM_DT0_A_OUTBYPASS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_fed_outinvert(&self) -> MCPWM_DT0_FED_OUTINVERT_R {
        MCPWM_DT0_FED_OUTINVERT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_red_outinvert(&self) -> MCPWM_DT0_RED_OUTINVERT_R {
        MCPWM_DT0_RED_OUTINVERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_fed_insel(&self) -> MCPWM_DT0_FED_INSEL_R {
        MCPWM_DT0_FED_INSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_red_insel(&self) -> MCPWM_DT0_RED_INSEL_R {
        MCPWM_DT0_RED_INSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_b_outswap(&self) -> MCPWM_DT0_B_OUTSWAP_R {
        MCPWM_DT0_B_OUTSWAP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_a_outswap(&self) -> MCPWM_DT0_A_OUTSWAP_R {
        MCPWM_DT0_A_OUTSWAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - S8 in documentation dual-edge B mode 0: FED/RED take effect on different path separately 1: FED/RED take effect on B path A out is in bypass or normal operation mode"]
    #[inline(always)]
    pub fn mcpwm_dt0_deb_mode(&self) -> MCPWM_DT0_DEB_MODE_R {
        MCPWM_DT0_DEB_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn mcpwm_dt0_red_upmethod(&self) -> MCPWM_DT0_RED_UPMETHOD_R {
        MCPWM_DT0_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn mcpwm_dt0_fed_upmethod(&self) -> MCPWM_DT0_FED_UPMETHOD_R {
        MCPWM_DT0_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 17 - Dead time generator 0 clock selection. 0: PWM_clk 1: PT_clk"]
    #[inline(always)]
    pub fn mcpwm_dt0_clk_sel(&mut self) -> MCPWM_DT0_CLK_SEL_W {
        MCPWM_DT0_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_b_outbypass(&mut self) -> MCPWM_DT0_B_OUTBYPASS_W {
        MCPWM_DT0_B_OUTBYPASS_W { w: self }
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_a_outbypass(&mut self) -> MCPWM_DT0_A_OUTBYPASS_W {
        MCPWM_DT0_A_OUTBYPASS_W { w: self }
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_fed_outinvert(&mut self) -> MCPWM_DT0_FED_OUTINVERT_W {
        MCPWM_DT0_FED_OUTINVERT_W { w: self }
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_red_outinvert(&mut self) -> MCPWM_DT0_RED_OUTINVERT_W {
        MCPWM_DT0_RED_OUTINVERT_W { w: self }
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_fed_insel(&mut self) -> MCPWM_DT0_FED_INSEL_W {
        MCPWM_DT0_FED_INSEL_W { w: self }
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_red_insel(&mut self) -> MCPWM_DT0_RED_INSEL_W {
        MCPWM_DT0_RED_INSEL_W { w: self }
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_b_outswap(&mut self) -> MCPWM_DT0_B_OUTSWAP_W {
        MCPWM_DT0_B_OUTSWAP_W { w: self }
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    pub fn mcpwm_dt0_a_outswap(&mut self) -> MCPWM_DT0_A_OUTSWAP_W {
        MCPWM_DT0_A_OUTSWAP_W { w: self }
    }
    #[doc = "Bit 8 - S8 in documentation dual-edge B mode 0: FED/RED take effect on different path separately 1: FED/RED take effect on B path A out is in bypass or normal operation mode"]
    #[inline(always)]
    pub fn mcpwm_dt0_deb_mode(&mut self) -> MCPWM_DT0_DEB_MODE_W {
        MCPWM_DT0_DEB_MODE_W { w: self }
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn mcpwm_dt0_red_upmethod(&mut self) -> MCPWM_DT0_RED_UPMETHOD_W {
        MCPWM_DT0_RED_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn mcpwm_dt0_fed_upmethod(&mut self) -> MCPWM_DT0_FED_UPMETHOD_W {
        MCPWM_DT0_FED_UPMETHOD_W { w: self }
    }
}
