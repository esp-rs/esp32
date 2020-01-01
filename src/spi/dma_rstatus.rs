#[doc = "Reader of register DMA_RSTATUS"]
pub type R = crate::R<u32, super::DMA_RSTATUS>;
#[doc = "Writer for register DMA_RSTATUS"]
pub type W = crate::W<u32, super::DMA_RSTATUS>;
#[doc = "Register DMA_RSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_RSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_DMA_OUT_STATUS`"]
pub type SPI_DMA_OUT_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_DMA_OUT_STATUS`"]
pub struct SPI_DMA_OUT_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_OUT_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - spi dma read data from memory status."]
    #[inline(always)]
    pub fn spi_dma_out_status(&self) -> SPI_DMA_OUT_STATUS_R {
        SPI_DMA_OUT_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - spi dma read data from memory status."]
    #[inline(always)]
    pub fn spi_dma_out_status(&mut self) -> SPI_DMA_OUT_STATUS_W {
        SPI_DMA_OUT_STATUS_W { w: self }
    }
}
