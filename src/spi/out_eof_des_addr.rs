#[doc = "Reader of register OUT_EOF_DES_ADDR"]
pub type R = crate::R<u32, super::OUT_EOF_DES_ADDR>;
#[doc = "Writer for register OUT_EOF_DES_ADDR"]
pub type W = crate::W<u32, super::OUT_EOF_DES_ADDR>;
#[doc = "Register OUT_EOF_DES_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_EOF_DES_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_OUT_EOF_DES_ADDR`"]
pub type DMA_OUT_EOF_DES_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_OUT_EOF_DES_ADDR`"]
pub struct DMA_OUT_EOF_DES_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_EOF_DES_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The last outlink descriptor address when spi dma produce to_eof."]
    #[inline(always)]
    pub fn dma_out_eof_des_addr(&self) -> DMA_OUT_EOF_DES_ADDR_R {
        DMA_OUT_EOF_DES_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The last outlink descriptor address when spi dma produce to_eof."]
    #[inline(always)]
    pub fn dma_out_eof_des_addr(&mut self) -> DMA_OUT_EOF_DES_ADDR_W {
        DMA_OUT_EOF_DES_ADDR_W { w: self }
    }
}
