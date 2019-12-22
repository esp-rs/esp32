#[doc = "Reader of register SENS_SAR_MEAS_WAIT2_REG"]
pub type R = crate::R<u32, super::SENS_SAR_MEAS_WAIT2_REG>;
#[doc = "Writer for register SENS_SAR_MEAS_WAIT2_REG"]
pub type W = crate::W<u32, super::SENS_SAR_MEAS_WAIT2_REG>;
#[doc = "Register SENS_SAR_MEAS_WAIT2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_SAR_MEAS_WAIT2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_SAR2_RSTB_WAIT`"]
pub type SENS_SAR2_RSTB_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_SAR2_RSTB_WAIT`"]
pub struct SENS_SAR2_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR2_RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `SENS_FORCE_XPD_SAR`"]
pub type SENS_FORCE_XPD_SAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_FORCE_XPD_SAR`"]
pub struct SENS_FORCE_XPD_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_FORCE_XPD_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `SENS_FORCE_XPD_AMP`"]
pub type SENS_FORCE_XPD_AMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENS_FORCE_XPD_AMP`"]
pub struct SENS_FORCE_XPD_AMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_FORCE_XPD_AMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENS_SAR_AMP_WAIT3`"]
pub type SENS_SAR_AMP_WAIT3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_SAR_AMP_WAIT3`"]
pub struct SENS_SAR_AMP_WAIT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR_AMP_WAIT3_W<'a> {
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
    pub fn sens_sar2_rstb_wait(&self) -> SENS_SAR2_RSTB_WAIT_R {
        SENS_SAR2_RSTB_WAIT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sens_force_xpd_sar(&self) -> SENS_FORCE_XPD_SAR_R {
        SENS_FORCE_XPD_SAR_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sens_force_xpd_amp(&self) -> SENS_FORCE_XPD_AMP_R {
        SENS_FORCE_XPD_AMP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sens_sar_amp_wait3(&self) -> SENS_SAR_AMP_WAIT3_R {
        SENS_SAR_AMP_WAIT3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn sens_sar2_rstb_wait(&mut self) -> SENS_SAR2_RSTB_WAIT_W {
        SENS_SAR2_RSTB_WAIT_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sens_force_xpd_sar(&mut self) -> SENS_FORCE_XPD_SAR_W {
        SENS_FORCE_XPD_SAR_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sens_force_xpd_amp(&mut self) -> SENS_FORCE_XPD_AMP_W {
        SENS_FORCE_XPD_AMP_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sens_sar_amp_wait3(&mut self) -> SENS_SAR_AMP_WAIT3_W {
        SENS_SAR_AMP_WAIT3_W { w: self }
    }
}
