#[doc = "Reader of register W11"]
pub type R = crate::R<u32, super::W11>;
#[doc = "Writer for register W11"]
pub type W = crate::W<u32, super::W11>;
#[doc = "Register W11 `reset()`'s with value 0"]
impl crate::ResetValue for super::W11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF11`"]
pub type BUF11_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUF11`"]
pub struct BUF11_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF11_W<'a> {
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
    pub fn buf11(&self) -> BUF11_R {
        BUF11_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf11(&mut self) -> BUF11_W {
        BUF11_W { w: self }
    }
}
