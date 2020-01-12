#[doc = "Reader of register BLK2_RDATA7"]
pub type R = crate::R<u32, super::BLK2_RDATA7>;
#[doc = "Writer for register BLK2_RDATA7"]
pub type W = crate::W<u32, super::BLK2_RDATA7>;
#[doc = "Register BLK2_RDATA7 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK2_RDATA7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLK2_DOUT7`"]
pub type BLK2_DOUT7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BLK2_DOUT7`"]
pub struct BLK2_DOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK2_DOUT7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK2"]
    #[inline(always)]
    pub fn blk2_dout7(&self) -> BLK2_DOUT7_R {
        BLK2_DOUT7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - read for BLOCK2"]
    #[inline(always)]
    pub fn blk2_dout7(&mut self) -> BLK2_DOUT7_W {
        BLK2_DOUT7_W { w: self }
    }
}