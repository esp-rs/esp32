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
#[doc = "Reader of field `SENS_AMP_SHORT_REF_GND_FORCE`"]
pub type SENS_AMP_SHORT_REF_GND_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_AMP_SHORT_REF_GND_FORCE`"]
pub struct SENS_AMP_SHORT_REF_GND_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_AMP_SHORT_REF_GND_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `SENS_AMP_SHORT_REF_FORCE`"]
pub type SENS_AMP_SHORT_REF_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_AMP_SHORT_REF_FORCE`"]
pub struct SENS_AMP_SHORT_REF_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_AMP_SHORT_REF_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `SENS_AMP_RST_FB_FORCE`"]
pub type SENS_AMP_RST_FB_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_AMP_RST_FB_FORCE`"]
pub struct SENS_AMP_RST_FB_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_AMP_RST_FB_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `SENS_SAR2_RSTB_FORCE`"]
pub type SENS_SAR2_RSTB_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR2_RSTB_FORCE`"]
pub struct SENS_SAR2_RSTB_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR2_RSTB_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `SENS_SAR_RSTB_FSM_IDLE`"]
pub type SENS_SAR_RSTB_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR_RSTB_FSM_IDLE`"]
pub struct SENS_SAR_RSTB_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR_RSTB_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `SENS_XPD_SAR_FSM_IDLE`"]
pub type SENS_XPD_SAR_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_XPD_SAR_FSM_IDLE`"]
pub struct SENS_XPD_SAR_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_XPD_SAR_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `SENS_AMP_SHORT_REF_GND_FSM_IDLE`"]
pub type SENS_AMP_SHORT_REF_GND_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_AMP_SHORT_REF_GND_FSM_IDLE`"]
pub struct SENS_AMP_SHORT_REF_GND_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_AMP_SHORT_REF_GND_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `SENS_AMP_SHORT_REF_FSM_IDLE`"]
pub type SENS_AMP_SHORT_REF_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_AMP_SHORT_REF_FSM_IDLE`"]
pub struct SENS_AMP_SHORT_REF_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_AMP_SHORT_REF_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `SENS_AMP_RST_FB_FSM_IDLE`"]
pub type SENS_AMP_RST_FB_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_AMP_RST_FB_FSM_IDLE`"]
pub struct SENS_AMP_RST_FB_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_AMP_RST_FB_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `SENS_XPD_SAR_AMP_FSM_IDLE`"]
pub type SENS_XPD_SAR_AMP_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_XPD_SAR_AMP_FSM_IDLE`"]
pub struct SENS_XPD_SAR_AMP_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_XPD_SAR_AMP_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `SENS_SAR1_DAC_XPD_FSM_IDLE`"]
pub type SENS_SAR1_DAC_XPD_FSM_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENS_SAR1_DAC_XPD_FSM_IDLE`"]
pub struct SENS_SAR1_DAC_XPD_FSM_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_DAC_XPD_FSM_IDLE_W<'a> {
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
#[doc = "Reader of field `SENS_SAR1_DAC_XPD_FSM`"]
pub type SENS_SAR1_DAC_XPD_FSM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR1_DAC_XPD_FSM`"]
pub struct SENS_SAR1_DAC_XPD_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR1_DAC_XPD_FSM_W<'a> {
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
    pub fn sens_amp_short_ref_gnd_force(&self) -> SENS_AMP_SHORT_REF_GND_FORCE_R {
        SENS_AMP_SHORT_REF_GND_FORCE_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn sens_amp_short_ref_force(&self) -> SENS_AMP_SHORT_REF_FORCE_R {
        SENS_AMP_SHORT_REF_FORCE_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn sens_amp_rst_fb_force(&self) -> SENS_AMP_RST_FB_FORCE_R {
        SENS_AMP_RST_FB_FORCE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sens_sar2_rstb_force(&self) -> SENS_SAR2_RSTB_FORCE_R {
        SENS_SAR2_RSTB_FORCE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sens_sar_rstb_fsm_idle(&self) -> SENS_SAR_RSTB_FSM_IDLE_R {
        SENS_SAR_RSTB_FSM_IDLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sens_xpd_sar_fsm_idle(&self) -> SENS_XPD_SAR_FSM_IDLE_R {
        SENS_XPD_SAR_FSM_IDLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sens_amp_short_ref_gnd_fsm_idle(&self) -> SENS_AMP_SHORT_REF_GND_FSM_IDLE_R {
        SENS_AMP_SHORT_REF_GND_FSM_IDLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sens_amp_short_ref_fsm_idle(&self) -> SENS_AMP_SHORT_REF_FSM_IDLE_R {
        SENS_AMP_SHORT_REF_FSM_IDLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sens_amp_rst_fb_fsm_idle(&self) -> SENS_AMP_RST_FB_FSM_IDLE_R {
        SENS_AMP_RST_FB_FSM_IDLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sens_xpd_sar_amp_fsm_idle(&self) -> SENS_XPD_SAR_AMP_FSM_IDLE_R {
        SENS_XPD_SAR_AMP_FSM_IDLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sens_sar1_dac_xpd_fsm_idle(&self) -> SENS_SAR1_DAC_XPD_FSM_IDLE_R {
        SENS_SAR1_DAC_XPD_FSM_IDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sens_sar1_dac_xpd_fsm(&self) -> SENS_SAR1_DAC_XPD_FSM_R {
        SENS_SAR1_DAC_XPD_FSM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn sens_amp_short_ref_gnd_force(&mut self) -> SENS_AMP_SHORT_REF_GND_FORCE_W {
        SENS_AMP_SHORT_REF_GND_FORCE_W { w: self }
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn sens_amp_short_ref_force(&mut self) -> SENS_AMP_SHORT_REF_FORCE_W {
        SENS_AMP_SHORT_REF_FORCE_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn sens_amp_rst_fb_force(&mut self) -> SENS_AMP_RST_FB_FORCE_W {
        SENS_AMP_RST_FB_FORCE_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sens_sar2_rstb_force(&mut self) -> SENS_SAR2_RSTB_FORCE_W {
        SENS_SAR2_RSTB_FORCE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sens_sar_rstb_fsm_idle(&mut self) -> SENS_SAR_RSTB_FSM_IDLE_W {
        SENS_SAR_RSTB_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sens_xpd_sar_fsm_idle(&mut self) -> SENS_XPD_SAR_FSM_IDLE_W {
        SENS_XPD_SAR_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sens_amp_short_ref_gnd_fsm_idle(&mut self) -> SENS_AMP_SHORT_REF_GND_FSM_IDLE_W {
        SENS_AMP_SHORT_REF_GND_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sens_amp_short_ref_fsm_idle(&mut self) -> SENS_AMP_SHORT_REF_FSM_IDLE_W {
        SENS_AMP_SHORT_REF_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sens_amp_rst_fb_fsm_idle(&mut self) -> SENS_AMP_RST_FB_FSM_IDLE_W {
        SENS_AMP_RST_FB_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sens_xpd_sar_amp_fsm_idle(&mut self) -> SENS_XPD_SAR_AMP_FSM_IDLE_W {
        SENS_XPD_SAR_AMP_FSM_IDLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sens_sar1_dac_xpd_fsm_idle(&mut self) -> SENS_SAR1_DAC_XPD_FSM_IDLE_W {
        SENS_SAR1_DAC_XPD_FSM_IDLE_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sens_sar1_dac_xpd_fsm(&mut self) -> SENS_SAR1_DAC_XPD_FSM_W {
        SENS_SAR1_DAC_XPD_FSM_W { w: self }
    }
}
