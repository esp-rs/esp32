#[doc = "Reader of register STORE5"]
pub type R = crate::R<u32, super::STORE5>;
#[doc = "Writer for register STORE5"]
pub type W = crate::W<u32, super::STORE5>;
#[doc = "Register STORE5 `reset()`'s with value 0"]
impl crate::ResetValue for super::STORE5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCRATCH5`"]
pub type SCRATCH5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCRATCH5`"]
pub struct SCRATCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH5_W<'a> {
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
    pub fn scratch5(&self) -> SCRATCH5_R {
        SCRATCH5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch5(&mut self) -> SCRATCH5_W {
        SCRATCH5_W { w: self }
    }
}
