#[doc = "Reader of register BLK1_RDATA0"]
pub type R = crate::R<u32, super::BLK1_RDATA0>;
#[doc = "Writer for register BLK1_RDATA0"]
pub type W = crate::W<u32, super::BLK1_RDATA0>;
#[doc = "Register BLK1_RDATA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK1_RDATA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLK1_DOUT0`"]
pub type BLK1_DOUT0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BLK1_DOUT0`"]
pub struct BLK1_DOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK1_DOUT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK1"]
    #[inline(always)]
    pub fn blk1_dout0(&self) -> BLK1_DOUT0_R {
        BLK1_DOUT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - read for BLOCK1"]
    #[inline(always)]
    pub fn blk1_dout0(&mut self) -> BLK1_DOUT0_W {
        BLK1_DOUT0_W { w: self }
    }
}
