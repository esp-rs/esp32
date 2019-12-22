#[doc = "Reader of register SENS_SAR_ATTEN2_REG"]
pub type R = crate::R<u32, super::SENS_SAR_ATTEN2_REG>;
#[doc = "Writer for register SENS_SAR_ATTEN2_REG"]
pub type W = crate::W<u32, super::SENS_SAR_ATTEN2_REG>;
#[doc = "Register SENS_SAR_ATTEN2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SENS_SAR_ATTEN2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENS_SAR2_ATTEN`"]
pub type SENS_SAR2_ATTEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SENS_SAR2_ATTEN`"]
pub struct SENS_SAR2_ATTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SENS_SAR2_ATTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
    #[inline(always)]
    pub fn sens_sar2_atten(&self) -> SENS_SAR2_ATTEN_R {
        SENS_SAR2_ATTEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
    #[inline(always)]
    pub fn sens_sar2_atten(&mut self) -> SENS_SAR2_ATTEN_W {
        SENS_SAR2_ATTEN_W { w: self }
    }
}
