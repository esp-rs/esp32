#[doc = "Reader of register SAR_MEAS_WAIT2"]
pub type R = crate::R<u32, super::SAR_MEAS_WAIT2>;
#[doc = "Writer for register SAR_MEAS_WAIT2"]
pub type W = crate::W<u32, super::SAR_MEAS_WAIT2>;
#[doc = "Register SAR_MEAS_WAIT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_MEAS_WAIT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR2_RSTB_WAIT`"]
pub type SAR2_RSTB_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAR2_RSTB_WAIT`"]
pub struct SAR2_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `FORCE_XPD_SAR`"]
pub type FORCE_XPD_SAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FORCE_XPD_SAR`"]
pub struct FORCE_XPD_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XPD_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `FORCE_XPD_AMP`"]
pub type FORCE_XPD_AMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FORCE_XPD_AMP`"]
pub struct FORCE_XPD_AMP_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XPD_AMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SAR_AMP_WAIT3`"]
pub type SAR_AMP_WAIT3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAR_AMP_WAIT3`"]
pub struct SAR_AMP_WAIT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_AMP_WAIT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&self) -> SAR2_RSTB_WAIT_R {
        SAR2_RSTB_WAIT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn force_xpd_amp(&self) -> FORCE_XPD_AMP_R {
        FORCE_XPD_AMP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SAR_AMP_WAIT3_R {
        SAR_AMP_WAIT3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&mut self) -> SAR2_RSTB_WAIT_W {
        SAR2_RSTB_WAIT_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W {
        FORCE_XPD_SAR_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn force_xpd_amp(&mut self) -> FORCE_XPD_AMP_W {
        FORCE_XPD_AMP_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sar_amp_wait3(&mut self) -> SAR_AMP_WAIT3_W {
        SAR_AMP_WAIT3_W { w: self }
    }
}
