#[doc = "Reader of register SPI_DMA_CONF_REG"]
pub type R = crate::R<u32, super::SPI_DMA_CONF_REG>;
#[doc = "Writer for register SPI_DMA_CONF_REG"]
pub type W = crate::W<u32, super::SPI_DMA_CONF_REG>;
#[doc = "Register SPI_DMA_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DMA_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_DMA_CONTINUE`"]
pub type SPI_DMA_CONTINUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DMA_CONTINUE`"]
pub struct SPI_DMA_CONTINUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_CONTINUE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_DMA_TX_STOP`"]
pub type SPI_DMA_TX_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DMA_TX_STOP`"]
pub struct SPI_DMA_TX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_TX_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SPI_DMA_RX_STOP`"]
pub type SPI_DMA_RX_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DMA_RX_STOP`"]
pub struct SPI_DMA_RX_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_RX_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPI_OUT_DATA_BURST_EN`"]
pub type SPI_OUT_DATA_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUT_DATA_BURST_EN`"]
pub struct SPI_OUT_DATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUT_DATA_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SPI_INDSCR_BURST_EN`"]
pub type SPI_INDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_INDSCR_BURST_EN`"]
pub struct SPI_INDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SPI_OUTDSCR_BURST_EN`"]
pub type SPI_OUTDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUTDSCR_BURST_EN`"]
pub struct SPI_OUTDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUTDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPI_OUT_EOF_MODE`"]
pub type SPI_OUT_EOF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUT_EOF_MODE`"]
pub struct SPI_OUT_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUT_EOF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPI_OUT_AUTO_WRBACK`"]
pub type SPI_OUT_AUTO_WRBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUT_AUTO_WRBACK`"]
pub struct SPI_OUT_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUT_AUTO_WRBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI_OUT_LOOP_TEST`"]
pub type SPI_OUT_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUT_LOOP_TEST`"]
pub struct SPI_OUT_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUT_LOOP_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SPI_IN_LOOP_TEST`"]
pub type SPI_IN_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_IN_LOOP_TEST`"]
pub struct SPI_IN_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_IN_LOOP_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPI_AHBM_RST`"]
pub type SPI_AHBM_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_AHBM_RST`"]
pub struct SPI_AHBM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_AHBM_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SPI_AHBM_FIFO_RST`"]
pub type SPI_AHBM_FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_AHBM_FIFO_RST`"]
pub struct SPI_AHBM_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_AHBM_FIFO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI_OUT_RST`"]
pub type SPI_OUT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_OUT_RST`"]
pub struct SPI_OUT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_OUT_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SPI_IN_RST`"]
pub type SPI_IN_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_IN_RST`"]
pub struct SPI_IN_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_IN_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - spi dma continue tx/rx data."]
    #[inline(always)]
    pub fn spi_dma_continue(&self) -> SPI_DMA_CONTINUE_R {
        SPI_DMA_CONTINUE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - spi dma write data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn spi_dma_tx_stop(&self) -> SPI_DMA_TX_STOP_R {
        SPI_DMA_TX_STOP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - spi dma read data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn spi_dma_rx_stop(&self) -> SPI_DMA_RX_STOP_R {
        SPI_DMA_RX_STOP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - spi dma read data from memory in burst mode."]
    #[inline(always)]
    pub fn spi_out_data_burst_en(&self) -> SPI_OUT_DATA_BURST_EN_R {
        SPI_OUT_DATA_BURST_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - read descriptor use burst mode when write data to memory."]
    #[inline(always)]
    pub fn spi_indscr_burst_en(&self) -> SPI_INDSCR_BURST_EN_R {
        SPI_INDSCR_BURST_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - read descriptor use burst mode when read data for memory."]
    #[inline(always)]
    pub fn spi_outdscr_burst_en(&self) -> SPI_OUTDSCR_BURST_EN_R {
        SPI_OUTDSCR_BURST_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
    #[inline(always)]
    pub fn spi_out_eof_mode(&self) -> SPI_OUT_EOF_MODE_R {
        SPI_OUT_EOF_MODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - when the link is empty jump to next automatically."]
    #[inline(always)]
    pub fn spi_out_auto_wrback(&self) -> SPI_OUT_AUTO_WRBACK_R {
        SPI_OUT_AUTO_WRBACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Set bit to test out link."]
    #[inline(always)]
    pub fn spi_out_loop_test(&self) -> SPI_OUT_LOOP_TEST_R {
        SPI_OUT_LOOP_TEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set bit to test in link."]
    #[inline(always)]
    pub fn spi_in_loop_test(&self) -> SPI_IN_LOOP_TEST_R {
        SPI_IN_LOOP_TEST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - reset spi dma ahb master."]
    #[inline(always)]
    pub fn spi_ahbm_rst(&self) -> SPI_AHBM_RST_R {
        SPI_AHBM_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - reset spi dma ahb master fifo pointer."]
    #[inline(always)]
    pub fn spi_ahbm_fifo_rst(&self) -> SPI_AHBM_FIFO_RST_R {
        SPI_AHBM_FIFO_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The bit is used to reset out dma fsm and out data fifo pointer."]
    #[inline(always)]
    pub fn spi_out_rst(&self) -> SPI_OUT_RST_R {
        SPI_OUT_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The bit is used to reset in dma fsm and in data fifo pointer."]
    #[inline(always)]
    pub fn spi_in_rst(&self) -> SPI_IN_RST_R {
        SPI_IN_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - spi dma continue tx/rx data."]
    #[inline(always)]
    pub fn spi_dma_continue(&mut self) -> SPI_DMA_CONTINUE_W {
        SPI_DMA_CONTINUE_W { w: self }
    }
    #[doc = "Bit 15 - spi dma write data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn spi_dma_tx_stop(&mut self) -> SPI_DMA_TX_STOP_W {
        SPI_DMA_TX_STOP_W { w: self }
    }
    #[doc = "Bit 14 - spi dma read data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn spi_dma_rx_stop(&mut self) -> SPI_DMA_RX_STOP_W {
        SPI_DMA_RX_STOP_W { w: self }
    }
    #[doc = "Bit 12 - spi dma read data from memory in burst mode."]
    #[inline(always)]
    pub fn spi_out_data_burst_en(&mut self) -> SPI_OUT_DATA_BURST_EN_W {
        SPI_OUT_DATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 11 - read descriptor use burst mode when write data to memory."]
    #[inline(always)]
    pub fn spi_indscr_burst_en(&mut self) -> SPI_INDSCR_BURST_EN_W {
        SPI_INDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 10 - read descriptor use burst mode when read data for memory."]
    #[inline(always)]
    pub fn spi_outdscr_burst_en(&mut self) -> SPI_OUTDSCR_BURST_EN_W {
        SPI_OUTDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 9 - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
    #[inline(always)]
    pub fn spi_out_eof_mode(&mut self) -> SPI_OUT_EOF_MODE_W {
        SPI_OUT_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 8 - when the link is empty jump to next automatically."]
    #[inline(always)]
    pub fn spi_out_auto_wrback(&mut self) -> SPI_OUT_AUTO_WRBACK_W {
        SPI_OUT_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 7 - Set bit to test out link."]
    #[inline(always)]
    pub fn spi_out_loop_test(&mut self) -> SPI_OUT_LOOP_TEST_W {
        SPI_OUT_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 6 - Set bit to test in link."]
    #[inline(always)]
    pub fn spi_in_loop_test(&mut self) -> SPI_IN_LOOP_TEST_W {
        SPI_IN_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 5 - reset spi dma ahb master."]
    #[inline(always)]
    pub fn spi_ahbm_rst(&mut self) -> SPI_AHBM_RST_W {
        SPI_AHBM_RST_W { w: self }
    }
    #[doc = "Bit 4 - reset spi dma ahb master fifo pointer."]
    #[inline(always)]
    pub fn spi_ahbm_fifo_rst(&mut self) -> SPI_AHBM_FIFO_RST_W {
        SPI_AHBM_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 3 - The bit is used to reset out dma fsm and out data fifo pointer."]
    #[inline(always)]
    pub fn spi_out_rst(&mut self) -> SPI_OUT_RST_W {
        SPI_OUT_RST_W { w: self }
    }
    #[doc = "Bit 2 - The bit is used to reset in dma fsm and in data fifo pointer."]
    #[inline(always)]
    pub fn spi_in_rst(&mut self) -> SPI_IN_RST_W {
        SPI_IN_RST_W { w: self }
    }
}