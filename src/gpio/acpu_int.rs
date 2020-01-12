#[doc = "Reader of register ACPU_INT"]
pub type R = crate::R<u32, super::ACPU_INT>;
#[doc = "Writer for register ACPU_INT"]
pub type W = crate::W<u32, super::ACPU_INT>;
#[doc = "Register ACPU_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::ACPU_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APPCPU_INT`"]
pub type APPCPU_INT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APPCPU_INT`"]
pub struct APPCPU_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCPU_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 APP CPU interrupt status"]
    #[inline(always)]
    pub fn appcpu_int(&self) -> APPCPU_INT_R {
        APPCPU_INT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 APP CPU interrupt status"]
    #[inline(always)]
    pub fn appcpu_int(&mut self) -> APPCPU_INT_W {
        APPCPU_INT_W { w: self }
    }
}
