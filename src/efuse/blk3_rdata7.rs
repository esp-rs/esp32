#[doc = "Reader of register BLK3_RDATA7"]
pub type R = crate::R<u32, super::BLK3_RDATA7>;
#[doc = "Writer for register BLK3_RDATA7"]
pub type W = crate::W<u32, super::BLK3_RDATA7>;
#[doc = "Register BLK3_RDATA7 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK3_RDATA7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_BLK3_DOUT7`"]
pub type EFUSE_BLK3_DOUT7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_BLK3_DOUT7`"]
pub struct EFUSE_BLK3_DOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_BLK3_DOUT7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - read for BLOCK3"]
    #[inline(always)]
    pub fn efuse_blk3_dout7(&self) -> EFUSE_BLK3_DOUT7_R {
        EFUSE_BLK3_DOUT7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - read for BLOCK3"]
    #[inline(always)]
    pub fn efuse_blk3_dout7(&mut self) -> EFUSE_BLK3_DOUT7_W {
        EFUSE_BLK3_DOUT7_W { w: self }
    }
}
