#[doc = "Reader of register W4"]
pub type R = crate::R<u32, super::W4>;
#[doc = "Writer for register W4"]
pub type W = crate::W<u32, super::W4>;
#[doc = "Register W4 `reset()`'s with value 0"]
impl crate::ResetValue for super::W4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF4`"]
pub type BUF4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUF4`"]
pub struct BUF4_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF4_W<'a> {
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
    pub fn buf4(&self) -> BUF4_R {
        BUF4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf4(&mut self) -> BUF4_W {
        BUF4_W { w: self }
    }
}
