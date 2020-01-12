#[doc = "Reader of register BLK3_WDATA0"]
pub type R = crate::R<u32, super::BLK3_WDATA0>;
#[doc = "Writer for register BLK3_WDATA0"]
pub type W = crate::W<u32, super::BLK3_WDATA0>;
#[doc = "Register BLK3_WDATA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK3_WDATA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLK3_DIN0`"]
pub type BLK3_DIN0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BLK3_DIN0`"]
pub struct BLK3_DIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK3_DIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - program for BLOCK3"]
    #[inline(always)]
    pub fn blk3_din0(&self) -> BLK3_DIN0_R {
        BLK3_DIN0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for BLOCK3"]
    #[inline(always)]
    pub fn blk3_din0(&mut self) -> BLK3_DIN0_W {
        BLK3_DIN0_W { w: self }
    }
}
