#[doc = "Reader of register ULP_CP_SLEEP_CYC3"]
pub type R = crate::R<u32, super::ULP_CP_SLEEP_CYC3>;
#[doc = "Writer for register ULP_CP_SLEEP_CYC3"]
pub type W = crate::W<u32, super::ULP_CP_SLEEP_CYC3>;
#[doc = "Register ULP_CP_SLEEP_CYC3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ULP_CP_SLEEP_CYC3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLEEP_CYCLES_S3`"]
pub type SLEEP_CYCLES_S3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLEEP_CYCLES_S3`"]
pub struct SLEEP_CYCLES_S3_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_CYCLES_S3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sleep_cycles_s3(&self) -> SLEEP_CYCLES_S3_R {
        SLEEP_CYCLES_S3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sleep_cycles_s3(&mut self) -> SLEEP_CYCLES_S3_W {
        SLEEP_CYCLES_S3_W { w: self }
    }
}