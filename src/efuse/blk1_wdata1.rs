#[doc = "Reader of register BLK1_WDATA1"]
pub type R = crate::R<u32, super::BLK1_WDATA1>;
#[doc = "Writer for register BLK1_WDATA1"]
pub type W = crate::W<u32, super::BLK1_WDATA1>;
#[doc = "Register BLK1_WDATA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK1_WDATA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLK1_DIN1`"]
pub type BLK1_DIN1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BLK1_DIN1`"]
pub struct BLK1_DIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK1_DIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - program for BLOCK1"]
    #[inline(always)]
    pub fn blk1_din1(&self) -> BLK1_DIN1_R {
        BLK1_DIN1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for BLOCK1"]
    #[inline(always)]
    pub fn blk1_din1(&mut self) -> BLK1_DIN1_W {
        BLK1_DIN1_W { w: self }
    }
}