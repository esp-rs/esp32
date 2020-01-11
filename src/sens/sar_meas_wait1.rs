#[doc = "Reader of register SAR_MEAS_WAIT1"]
pub type R = crate::R<u32, super::SAR_MEAS_WAIT1>;
#[doc = "Writer for register SAR_MEAS_WAIT1"]
pub type W = crate::W<u32, super::SAR_MEAS_WAIT1>;
#[doc = "Register SAR_MEAS_WAIT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_MEAS_WAIT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_SAR_AMP_WAIT2`"]
pub type SENS_SAR_AMP_WAIT2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_SAR_AMP_WAIT2`"]
pub struct SENS_SAR_AMP_WAIT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR_AMP_WAIT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENS_SAR_AMP_WAIT1`"]
pub type SENS_SAR_AMP_WAIT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENS_SAR_AMP_WAIT1`"]
pub struct SENS_SAR_AMP_WAIT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR_AMP_WAIT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sens_sar_amp_wait2(&self) -> SENS_SAR_AMP_WAIT2_R {
        SENS_SAR_AMP_WAIT2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sens_sar_amp_wait1(&self) -> SENS_SAR_AMP_WAIT1_R {
        SENS_SAR_AMP_WAIT1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sens_sar_amp_wait2(&mut self) -> SENS_SAR_AMP_WAIT2_W {
        SENS_SAR_AMP_WAIT2_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sens_sar_amp_wait1(&mut self) -> SENS_SAR_AMP_WAIT1_W {
        SENS_SAR_AMP_WAIT1_W { w: self }
    }
}
