#[doc = "Reader of register W3"]
pub type R = crate::R<u32, super::W3>;
#[doc = "Writer for register W3"]
pub type W = crate::W<u32, super::W3>;
#[doc = "Register W3 `reset()`'s with value 0"]
impl crate::ResetValue for super::W3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF3`"]
pub type BUF3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUF3`"]
pub struct BUF3_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF3_W<'a> {
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
    pub fn buf3(&self) -> BUF3_R {
        BUF3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf3(&mut self) -> BUF3_W {
        BUF3_W { w: self }
    }
}
