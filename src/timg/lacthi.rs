#[doc = "Reader of register LACTHI"]
pub type R = crate::R<u32, super::LACTHI>;
#[doc = "Writer for register LACTHI"]
pub type W = crate::W<u32, super::LACTHI>;
#[doc = "Register LACTHI `reset()`'s with value 0"]
impl crate::ResetValue for super::LACTHI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LACT_HI`"]
pub type LACT_HI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LACT_HI`"]
pub struct LACT_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_HI_W<'a> {
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
    pub fn lact_hi(&self) -> LACT_HI_R {
        LACT_HI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_hi(&mut self) -> LACT_HI_W {
        LACT_HI_W { w: self }
    }
}
