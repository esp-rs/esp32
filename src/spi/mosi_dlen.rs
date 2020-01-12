#[doc = "Reader of register MOSI_DLEN"]
pub type R = crate::R<u32, super::MOSI_DLEN>;
#[doc = "Writer for register MOSI_DLEN"]
pub type W = crate::W<u32, super::MOSI_DLEN>;
#[doc = "Register MOSI_DLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::MOSI_DLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USR_MOSI_DBITLEN`"]
pub type USR_MOSI_DBITLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `USR_MOSI_DBITLEN`"]
pub struct USR_MOSI_DBITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MOSI_DBITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - The length in bits of write-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn usr_mosi_dbitlen(&self) -> USR_MOSI_DBITLEN_R {
        USR_MOSI_DBITLEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - The length in bits of write-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn usr_mosi_dbitlen(&mut self) -> USR_MOSI_DBITLEN_W {
        USR_MOSI_DBITLEN_W { w: self }
    }
}
