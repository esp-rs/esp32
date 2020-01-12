#[doc = "Reader of register STORE3"]
pub type R = crate::R<u32, super::STORE3>;
#[doc = "Writer for register STORE3"]
pub type W = crate::W<u32, super::STORE3>;
#[doc = "Register STORE3 `reset()`'s with value 0"]
impl crate::ResetValue for super::STORE3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCRATCH3`"]
pub type SCRATCH3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCRATCH3`"]
pub struct SCRATCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH3_W<'a> {
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
    pub fn scratch3(&self) -> SCRATCH3_R {
        SCRATCH3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch3(&mut self) -> SCRATCH3_W {
        SCRATCH3_W { w: self }
    }
}
