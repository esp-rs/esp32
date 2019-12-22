#[doc = "Reader of register SPI_DMA_TSTATUS_REG"]
pub type R = crate::R<u32, super::SPI_DMA_TSTATUS_REG>;
#[doc = "Writer for register SPI_DMA_TSTATUS_REG"]
pub type W = crate::W<u32, super::SPI_DMA_TSTATUS_REG>;
#[doc = "Register SPI_DMA_TSTATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DMA_TSTATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_DMA_IN_STATUS`"]
pub type SPI_DMA_IN_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_DMA_IN_STATUS`"]
pub struct SPI_DMA_IN_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_IN_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - spi dma write data to memory status."]
    #[inline(always)]
    pub fn spi_dma_in_status(&self) -> SPI_DMA_IN_STATUS_R {
        SPI_DMA_IN_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - spi dma write data to memory status."]
    #[inline(always)]
    pub fn spi_dma_in_status(&mut self) -> SPI_DMA_IN_STATUS_W {
        SPI_DMA_IN_STATUS_W { w: self }
    }
}
