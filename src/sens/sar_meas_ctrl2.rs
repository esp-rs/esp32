#[doc = "Reader of register SAR_MEAS_CTRL2"]
pub type R = crate::R<u32, super::SAR_MEAS_CTRL2>;
#[doc = "Writer for register SAR_MEAS_CTRL2"]
pub type W = crate::W<u32, super::SAR_MEAS_CTRL2>;
#[doc = "Register SAR_MEAS_CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_MEAS_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AMP_SHORT_REF_GND_FORCE`"]
pub type AMP_SHORT_REF_GND_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMP_SHORT_REF_GND_FORCE`"]
pub struct AMP_SHORT_REF_GND_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_GND_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `AMP_SHORT_REF_FORCE`"]
pub type AMP_SHORT_REF_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMP_SHORT_REF_FORCE`"]
pub struct AMP_SHORT_REF_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `AMP_RST_FB_FORCE`"]
pub type AMP_RST_FB_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMP_RST_FB_FORCE`"]
pub struct AMP_RST_FB_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_RST_FB_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `SAR2_RSTB_FORCE`"]
pub type SAR2_RSTB_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR2_RSTB_FORCE`"]
pub struct SAR2_RSTB_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_RSTB_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `SAR_RSTB_FSM_IDLE`"]
pub type SAR_RSTB_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR_RSTB_FSM_IDLE`"]
pub struct SAR_RSTB_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_RSTB_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `XPD_SAR_FSM_IDLE`"]
pub type XPD_SAR_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XPD_SAR_FSM_IDLE`"]
pub struct XPD_SAR_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `AMP_SHORT_REF_GND_FSM_IDLE`"]
pub type AMP_SHORT_REF_GND_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMP_SHORT_REF_GND_FSM_IDLE`"]
pub struct AMP_SHORT_REF_GND_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_GND_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `AMP_SHORT_REF_FSM_IDLE`"]
pub type AMP_SHORT_REF_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMP_SHORT_REF_FSM_IDLE`"]
pub struct AMP_SHORT_REF_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `AMP_RST_FB_FSM_IDLE`"]
pub type AMP_RST_FB_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMP_RST_FB_FSM_IDLE`"]
pub struct AMP_RST_FB_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_RST_FB_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `XPD_SAR_AMP_FSM_IDLE`"]
pub type XPD_SAR_AMP_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XPD_SAR_AMP_FSM_IDLE`"]
pub struct XPD_SAR_AMP_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_AMP_FSM_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SAR1_DAC_XPD_FSM_IDLE`"]
pub type SAR1_DAC_XPD_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAR1_DAC_XPD_FSM_IDLE`"]
pub struct SAR1_DAC_XPD_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_DAC_XPD_FSM_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SAR1_DAC_XPD_FSM`"]
pub type SAR1_DAC_XPD_FSM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR1_DAC_XPD_FSM`"]
pub struct SAR1_DAC_XPD_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR1_DAC_XPD_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&self) -> AMP_SHORT_REF_GND_FORCE_R {
        AMP_SHORT_REF_GND_FORCE_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn amp_short_ref_force(&self) -> AMP_SHORT_REF_FORCE_R {
        AMP_SHORT_REF_FORCE_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&self) -> AMP_RST_FB_FORCE_R {
        AMP_RST_FB_FORCE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sar2_rstb_force(&self) -> SAR2_RSTB_FORCE_R {
        SAR2_RSTB_FORCE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&self) -> SAR_RSTB_FSM_IDLE_R {
        SAR_RSTB_FSM_IDLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&self) -> XPD_SAR_FSM_IDLE_R {
        XPD_SAR_FSM_IDLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&self) -> AMP_SHORT_REF_GND_FSM_IDLE_R {
        AMP_SHORT_REF_GND_FSM_IDLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&self) -> AMP_SHORT_REF_FSM_IDLE_R {
        AMP_SHORT_REF_FSM_IDLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&self) -> AMP_RST_FB_FSM_IDLE_R {
        AMP_RST_FB_FSM_IDLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&self) -> XPD_SAR_AMP_FSM_IDLE_R {
        XPD_SAR_AMP_FSM_IDLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&self) -> SAR1_DAC_XPD_FSM_IDLE_R {
        SAR1_DAC_XPD_FSM_IDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&self) -> SAR1_DAC_XPD_FSM_R {
        SAR1_DAC_XPD_FSM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_force(&mut self) -> AMP_SHORT_REF_GND_FORCE_W {
        AMP_SHORT_REF_GND_FORCE_W { w: self }
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn amp_short_ref_force(&mut self) -> AMP_SHORT_REF_FORCE_W {
        AMP_SHORT_REF_FORCE_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn amp_rst_fb_force(&mut self) -> AMP_RST_FB_FORCE_W {
        AMP_RST_FB_FORCE_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sar2_rstb_force(&mut self) -> SAR2_RSTB_FORCE_W {
        SAR2_RSTB_FORCE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sar_rstb_fsm_idle(&mut self) -> SAR_RSTB_FSM_IDLE_W {
        SAR_RSTB_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn xpd_sar_fsm_idle(&mut self) -> XPD_SAR_FSM_IDLE_W {
        XPD_SAR_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm_idle(&mut self) -> AMP_SHORT_REF_GND_FSM_IDLE_W {
        AMP_SHORT_REF_GND_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn amp_short_ref_fsm_idle(&mut self) -> AMP_SHORT_REF_FSM_IDLE_W {
        AMP_SHORT_REF_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm_idle(&mut self) -> AMP_RST_FB_FSM_IDLE_W {
        AMP_RST_FB_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm_idle(&mut self) -> XPD_SAR_AMP_FSM_IDLE_W {
        XPD_SAR_AMP_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm_idle(&mut self) -> SAR1_DAC_XPD_FSM_IDLE_W {
        SAR1_DAC_XPD_FSM_IDLE_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sar1_dac_xpd_fsm(&mut self) -> SAR1_DAC_XPD_FSM_W {
        SAR1_DAC_XPD_FSM_W { w: self }
    }
}
