#[doc = "Reader of register STORE6"]
pub type R = crate::R<u32, super::STORE6>;
#[doc = "Writer for register STORE6"]
pub type W = crate::W<u32, super::STORE6>;
#[doc = "Register STORE6 `reset()`'s with value 0"]
impl crate::ResetValue for super::STORE6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCRATCH6`"]
pub type SCRATCH6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCRATCH6`"]
pub struct SCRATCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch6(&self) -> SCRATCH6_R {
        SCRATCH6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch6(&mut self) -> SCRATCH6_W {
        SCRATCH6_W { w: self }
    }
}
