#[doc = "Reader of register BLK2_RDATA2"]
pub type R = crate::R<u32, super::BLK2_RDATA2>;
#[doc = "Writer for register BLK2_RDATA2"]
pub type W = crate::W<u32, super::BLK2_RDATA2>;
#[doc = "Register BLK2_RDATA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK2_RDATA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLK2_DOUT2`"]
pub type BLK2_DOUT2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BLK2_DOUT2`"]
pub struct BLK2_DOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK2_DOUT2_W<'a> {
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
    pub fn blk2_dout2(&self) -> BLK2_DOUT2_R {
        BLK2_DOUT2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - read for BLOCK2"]
    #[inline(always)]
    pub fn blk2_dout2(&mut self) -> BLK2_DOUT2_W {
        BLK2_DOUT2_W { w: self }
    }
}
