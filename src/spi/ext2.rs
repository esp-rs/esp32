#[doc = "Reader of register EXT2"]
pub type R = crate::R<u32, super::EXT2>;
#[doc = "Writer for register EXT2"]
pub type W = crate::W<u32, super::EXT2>;
#[doc = "Register EXT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ST`"]
pub type ST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ST`"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - The status of spi state machine ."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The status of spi state machine ."]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
}
