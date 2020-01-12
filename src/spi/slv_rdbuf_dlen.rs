#[doc = "Reader of register SLV_RDBUF_DLEN"]
pub type R = crate::R<u32, super::SLV_RDBUF_DLEN>;
#[doc = "Writer for register SLV_RDBUF_DLEN"]
pub type W = crate::W<u32, super::SLV_RDBUF_DLEN>;
#[doc = "Register SLV_RDBUF_DLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SLV_RDBUF_DLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLV_RDBUF_DBITLEN`"]
pub type SLV_RDBUF_DBITLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLV_RDBUF_DBITLEN`"]
pub struct SLV_RDBUF_DBITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDBUF_DBITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rdbuf_dbitlen(&self) -> SLV_RDBUF_DBITLEN_R {
        SLV_RDBUF_DBITLEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - In the slave mode it is the length in bits for read-buffer operations. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rdbuf_dbitlen(&mut self) -> SLV_RDBUF_DBITLEN_W {
        SLV_RDBUF_DBITLEN_W { w: self }
    }
}
