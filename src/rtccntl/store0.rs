#[doc = "Reader of register STORE0"]
pub type R = crate::R<u32, super::STORE0>;
#[doc = "Writer for register STORE0"]
pub type W = crate::W<u32, super::STORE0>;
#[doc = "Register STORE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STORE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCRATCH0`"]
pub type SCRATCH0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCRATCH0`"]
pub struct SCRATCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH0_W<'a> {
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
    pub fn scratch0(&self) -> SCRATCH0_R {
        SCRATCH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch0(&mut self) -> SCRATCH0_W {
        SCRATCH0_W { w: self }
    }
}
