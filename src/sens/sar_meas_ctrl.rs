#[doc = "Reader of register SAR_MEAS_CTRL"]
pub type R = crate::R<u32, super::SAR_MEAS_CTRL>;
#[doc = "Writer for register SAR_MEAS_CTRL"]
pub type W = crate::W<u32, super::SAR_MEAS_CTRL>;
#[doc = "Register SAR_MEAS_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_MEAS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR2_XPD_WAIT`"]
pub type SAR2_XPD_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR2_XPD_WAIT`"]
pub struct SAR2_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SAR_RSTB_FSM`"]
pub type SAR_RSTB_FSM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR_RSTB_FSM`"]
pub struct SAR_RSTB_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_RSTB_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `XPD_SAR_FSM`"]
pub type XPD_SAR_FSM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XPD_SAR_FSM`"]
pub struct XPD_SAR_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `AMP_SHORT_REF_GND_FSM`"]
pub type AMP_SHORT_REF_GND_FSM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMP_SHORT_REF_GND_FSM`"]
pub struct AMP_SHORT_REF_GND_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_GND_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `AMP_SHORT_REF_FSM`"]
pub type AMP_SHORT_REF_FSM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMP_SHORT_REF_FSM`"]
pub struct AMP_SHORT_REF_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_SHORT_REF_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AMP_RST_FB_FSM`"]
pub type AMP_RST_FB_FSM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMP_RST_FB_FSM`"]
pub struct AMP_RST_FB_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_RST_FB_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `XPD_SAR_AMP_FSM`"]
pub type XPD_SAR_AMP_FSM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XPD_SAR_AMP_FSM`"]
pub struct XPD_SAR_AMP_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_AMP_FSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&self) -> SAR2_XPD_WAIT_R {
        SAR2_XPD_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&self) -> SAR_RSTB_FSM_R {
        SAR_RSTB_FSM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn xpd_sar_fsm(&self) -> XPD_SAR_FSM_R {
        XPD_SAR_FSM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&self) -> AMP_SHORT_REF_GND_FSM_R {
        AMP_SHORT_REF_GND_FSM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amp_short_ref_fsm(&self) -> AMP_SHORT_REF_FSM_R {
        AMP_SHORT_REF_FSM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&self) -> AMP_RST_FB_FSM_R {
        AMP_RST_FB_FSM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&self) -> XPD_SAR_AMP_FSM_R {
        XPD_SAR_AMP_FSM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&mut self) -> SAR2_XPD_WAIT_W {
        SAR2_XPD_WAIT_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn sar_rstb_fsm(&mut self) -> SAR_RSTB_FSM_W {
        SAR_RSTB_FSM_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn xpd_sar_fsm(&mut self) -> XPD_SAR_FSM_W {
        XPD_SAR_FSM_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn amp_short_ref_gnd_fsm(&mut self) -> AMP_SHORT_REF_GND_FSM_W {
        AMP_SHORT_REF_GND_FSM_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amp_short_ref_fsm(&mut self) -> AMP_SHORT_REF_FSM_W {
        AMP_SHORT_REF_FSM_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn amp_rst_fb_fsm(&mut self) -> AMP_RST_FB_FSM_W {
        AMP_RST_FB_FSM_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn xpd_sar_amp_fsm(&mut self) -> XPD_SAR_AMP_FSM_W {
        XPD_SAR_AMP_FSM_W { w: self }
    }
}
