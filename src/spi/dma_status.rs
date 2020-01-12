#[doc = "Reader of register DMA_STATUS"]
pub type R = crate::R<u32, super::DMA_STATUS>;
#[doc = "Writer for register DMA_STATUS"]
pub type W = crate::W<u32, super::DMA_STATUS>;
#[doc = "Register DMA_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_TX_EN`"]
pub type DMA_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_TX_EN`"]
pub struct DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DMA_RX_EN`"]
pub type DMA_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_RX_EN`"]
pub struct DMA_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - spi dma write data status bit."]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - spi dma read data status bit."]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - spi dma write data status bit."]
    #[inline(always)]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W {
        DMA_TX_EN_W { w: self }
    }
    #[doc = "Bit 0 - spi dma read data status bit."]
    #[inline(always)]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W {
        DMA_RX_EN_W { w: self }
    }
}
