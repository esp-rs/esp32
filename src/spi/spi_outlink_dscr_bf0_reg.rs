#[doc = "Reader of register SPI_OUTLINK_DSCR_BF0_REG"]
pub type R = crate::R<u32, super::SPI_OUTLINK_DSCR_BF0_REG>;
#[doc = "Writer for register SPI_OUTLINK_DSCR_BF0_REG"]
pub type W = crate::W<u32, super::SPI_OUTLINK_DSCR_BF0_REG>;
#[doc = "Register SPI_OUTLINK_DSCR_BF0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_OUTLINK_DSCR_BF0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_DMA_OUTLINK_DSCR_BF0`"]
pub type SPI_DMA_OUTLINK_DSCR_BF0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_DMA_OUTLINK_DSCR_BF0`"]
pub struct SPI_DMA_OUTLINK_DSCR_BF0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_OUTLINK_DSCR_BF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The content of next out descriptor pointer."]
    #[inline(always)]
    pub fn spi_dma_outlink_dscr_bf0(&self) -> SPI_DMA_OUTLINK_DSCR_BF0_R {
        SPI_DMA_OUTLINK_DSCR_BF0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The content of next out descriptor pointer."]
    #[inline(always)]
    pub fn spi_dma_outlink_dscr_bf0(&mut self) -> SPI_DMA_OUTLINK_DSCR_BF0_W {
        SPI_DMA_OUTLINK_DSCR_BF0_W { w: self }
    }
}
