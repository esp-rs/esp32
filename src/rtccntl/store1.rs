#[doc = "Reader of register STORE1"]
pub type R = crate::R<u32, super::STORE1>;
#[doc = "Writer for register STORE1"]
pub type W = crate::W<u32, super::STORE1>;
#[doc = "Register STORE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STORE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCRATCH1`"]
pub type SCRATCH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SCRATCH1`"]
pub struct SCRATCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH1_W<'a> {
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
    pub fn scratch1(&self) -> SCRATCH1_R {
        SCRATCH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit general purpose retention register"]
    #[inline(always)]
    pub fn scratch1(&mut self) -> SCRATCH1_W {
        SCRATCH1_W { w: self }
    }
}
