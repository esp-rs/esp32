#[doc = "Reader of register W2"]
pub type R = crate::R<u32, super::W2>;
#[doc = "Writer for register W2"]
pub type W = crate::W<u32, super::W2>;
#[doc = "Register W2 `reset()`'s with value 0"]
impl crate::ResetValue for super::W2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF2`"]
pub type BUF2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUF2`"]
pub struct BUF2_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf2(&self) -> BUF2_R {
        BUF2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf2(&mut self) -> BUF2_W {
        BUF2_W { w: self }
    }
}
