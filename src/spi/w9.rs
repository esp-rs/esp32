#[doc = "Reader of register W9"]
pub type R = crate::R<u32, super::W9>;
#[doc = "Writer for register W9"]
pub type W = crate::W<u32, super::W9>;
#[doc = "Register W9 `reset()`'s with value 0"]
impl crate::ResetValue for super::W9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF9`"]
pub type BUF9_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUF9`"]
pub struct BUF9_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF9_W<'a> {
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
    pub fn buf9(&self) -> BUF9_R {
        BUF9_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf9(&mut self) -> BUF9_W {
        BUF9_W { w: self }
    }
}
