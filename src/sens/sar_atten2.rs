#[doc = "Reader of register SAR_ATTEN2"]
pub type R = crate::R<u32, super::SAR_ATTEN2>;
#[doc = "Writer for register SAR_ATTEN2"]
pub type W = crate::W<u32, super::SAR_ATTEN2>;
#[doc = "Register SAR_ATTEN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAR_ATTEN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAR2_ATTEN`"]
pub type SAR2_ATTEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SAR2_ATTEN`"]
pub struct SAR2_ATTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR2_ATTEN_W<'a> {
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
    pub fn sar2_atten(&self) -> SAR2_ATTEN_R {
        SAR2_ATTEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad 11:1dB 10:6dB 01:3dB 00:0dB"]
    #[inline(always)]
    pub fn sar2_atten(&mut self) -> SAR2_ATTEN_W {
        SAR2_ATTEN_W { w: self }
    }
}
