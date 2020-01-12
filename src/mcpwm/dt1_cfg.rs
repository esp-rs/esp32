#[doc = "Reader of register DT1_CFG"]
pub type R = crate::R<u32, super::DT1_CFG>;
#[doc = "Writer for register DT1_CFG"]
pub type W = crate::W<u32, super::DT1_CFG>;
#[doc = "Register DT1_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DT1_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DT1_CLK_SEL`"]
pub type DT1_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_CLK_SEL`"]
pub struct DT1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_CLK_SEL_W<'a> {
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
#[doc = "Reader of field `DT1_B_OUTBYPASS`"]
pub type DT1_B_OUTBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_B_OUTBYPASS`"]
pub struct DT1_B_OUTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_B_OUTBYPASS_W<'a> {
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
#[doc = "Reader of field `DT1_A_OUTBYPASS`"]
pub type DT1_A_OUTBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_A_OUTBYPASS`"]
pub struct DT1_A_OUTBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_A_OUTBYPASS_W<'a> {
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
#[doc = "Reader of field `DT1_FED_OUTINVERT`"]
pub type DT1_FED_OUTINVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_FED_OUTINVERT`"]
pub struct DT1_FED_OUTINVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_FED_OUTINVERT_W<'a> {
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
#[doc = "Reader of field `DT1_RED_OUTINVERT`"]
pub type DT1_RED_OUTINVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_RED_OUTINVERT`"]
pub struct DT1_RED_OUTINVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_RED_OUTINVERT_W<'a> {
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
#[doc = "Reader of field `DT1_FED_INSEL`"]
pub type DT1_FED_INSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_FED_INSEL`"]
pub struct DT1_FED_INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_FED_INSEL_W<'a> {
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
#[doc = "Reader of field `DT1_RED_INSEL`"]
pub type DT1_RED_INSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_RED_INSEL`"]
pub struct DT1_RED_INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_RED_INSEL_W<'a> {
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
#[doc = "Reader of field `DT1_B_OUTSWAP`"]
pub type DT1_B_OUTSWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_B_OUTSWAP`"]
pub struct DT1_B_OUTSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_B_OUTSWAP_W<'a> {
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
#[doc = "Reader of field `DT1_A_OUTSWAP`"]
pub type DT1_A_OUTSWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_A_OUTSWAP`"]
pub struct DT1_A_OUTSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_A_OUTSWAP_W<'a> {
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
#[doc = "Reader of field `DT1_DEB_MODE`"]
pub type DT1_DEB_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT1_DEB_MODE`"]
pub struct DT1_DEB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_DEB_MODE_W<'a> {
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
#[doc = "Reader of field `DT1_RED_UPMETHOD`"]
pub type DT1_RED_UPMETHOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT1_RED_UPMETHOD`"]
pub struct DT1_RED_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_RED_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DT1_FED_UPMETHOD`"]
pub type DT1_FED_UPMETHOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT1_FED_UPMETHOD`"]
pub struct DT1_FED_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_FED_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Dead time generator 1 clock selection. 0: PWM_clk 1: PT_clk"]
    #[inline(always)]
    pub fn dt1_clk_sel(&self) -> DT1_CLK_SEL_R {
        DT1_CLK_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    pub fn dt1_b_outbypass(&self) -> DT1_B_OUTBYPASS_R {
        DT1_B_OUTBYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    pub fn dt1_a_outbypass(&self) -> DT1_A_OUTBYPASS_R {
        DT1_A_OUTBYPASS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    pub fn dt1_fed_outinvert(&self) -> DT1_FED_OUTINVERT_R {
        DT1_FED_OUTINVERT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    pub fn dt1_red_outinvert(&self) -> DT1_RED_OUTINVERT_R {
        DT1_RED_OUTINVERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    pub fn dt1_fed_insel(&self) -> DT1_FED_INSEL_R {
        DT1_FED_INSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    pub fn dt1_red_insel(&self) -> DT1_RED_INSEL_R {
        DT1_RED_INSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    pub fn dt1_b_outswap(&self) -> DT1_B_OUTSWAP_R {
        DT1_B_OUTSWAP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    pub fn dt1_a_outswap(&self) -> DT1_A_OUTSWAP_R {
        DT1_A_OUTSWAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - S8 in documentation dual-edge B mode 0: FED/RED take effect on different path separately 1: FED/RED take effect on B path A out is in bypass or normal operation mode"]
    #[inline(always)]
    pub fn dt1_deb_mode(&self) -> DT1_DEB_MODE_R {
        DT1_DEB_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn dt1_red_upmethod(&self) -> DT1_RED_UPMETHOD_R {
        DT1_RED_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn dt1_fed_upmethod(&self) -> DT1_FED_UPMETHOD_R {
        DT1_FED_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 17 - Dead time generator 1 clock selection. 0: PWM_clk 1: PT_clk"]
    #[inline(always)]
    pub fn dt1_clk_sel(&mut self) -> DT1_CLK_SEL_W {
        DT1_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 16 - S0 in documentation"]
    #[inline(always)]
    pub fn dt1_b_outbypass(&mut self) -> DT1_B_OUTBYPASS_W {
        DT1_B_OUTBYPASS_W { w: self }
    }
    #[doc = "Bit 15 - S1 in documentation"]
    #[inline(always)]
    pub fn dt1_a_outbypass(&mut self) -> DT1_A_OUTBYPASS_W {
        DT1_A_OUTBYPASS_W { w: self }
    }
    #[doc = "Bit 14 - S3 in documentation"]
    #[inline(always)]
    pub fn dt1_fed_outinvert(&mut self) -> DT1_FED_OUTINVERT_W {
        DT1_FED_OUTINVERT_W { w: self }
    }
    #[doc = "Bit 13 - S2 in documentation"]
    #[inline(always)]
    pub fn dt1_red_outinvert(&mut self) -> DT1_RED_OUTINVERT_W {
        DT1_RED_OUTINVERT_W { w: self }
    }
    #[doc = "Bit 12 - S5 in documentation"]
    #[inline(always)]
    pub fn dt1_fed_insel(&mut self) -> DT1_FED_INSEL_W {
        DT1_FED_INSEL_W { w: self }
    }
    #[doc = "Bit 11 - S4 in documentation"]
    #[inline(always)]
    pub fn dt1_red_insel(&mut self) -> DT1_RED_INSEL_W {
        DT1_RED_INSEL_W { w: self }
    }
    #[doc = "Bit 10 - S7 in documentation"]
    #[inline(always)]
    pub fn dt1_b_outswap(&mut self) -> DT1_B_OUTSWAP_W {
        DT1_B_OUTSWAP_W { w: self }
    }
    #[doc = "Bit 9 - S6 in documentation"]
    #[inline(always)]
    pub fn dt1_a_outswap(&mut self) -> DT1_A_OUTSWAP_W {
        DT1_A_OUTSWAP_W { w: self }
    }
    #[doc = "Bit 8 - S8 in documentation dual-edge B mode 0: FED/RED take effect on different path separately 1: FED/RED take effect on B path A out is in bypass or normal operation mode"]
    #[inline(always)]
    pub fn dt1_deb_mode(&mut self) -> DT1_DEB_MODE_W {
        DT1_DEB_MODE_W { w: self }
    }
    #[doc = "Bits 4:7 - Update method for RED (rising edge delay) active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn dt1_red_upmethod(&mut self) -> DT1_RED_UPMETHOD_W {
        DT1_RED_UPMETHOD_W { w: self }
    }
    #[doc = "Bits 0:3 - Update method for FED (falling edge delay) active reg. 0: immediate bit0: TEZ bit1: TEP bit2: sync bit3: disable update"]
    #[inline(always)]
    pub fn dt1_fed_upmethod(&mut self) -> DT1_FED_UPMETHOD_W {
        DT1_FED_UPMETHOD_W { w: self }
    }
}
