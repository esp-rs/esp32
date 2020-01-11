#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_WR_BIT_ORDER`"]
pub type SPI_WR_BIT_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_WR_BIT_ORDER`"]
pub struct SPI_WR_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_WR_BIT_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `SPI_RD_BIT_ORDER`"]
pub type SPI_RD_BIT_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_RD_BIT_ORDER`"]
pub struct SPI_RD_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RD_BIT_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SPI_FREAD_QIO`"]
pub type SPI_FREAD_QIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FREAD_QIO`"]
pub struct SPI_FREAD_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FREAD_QIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI_FREAD_DIO`"]
pub type SPI_FREAD_DIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FREAD_DIO`"]
pub struct SPI_FREAD_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FREAD_DIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SPI_WRSR_2B`"]
pub type SPI_WRSR_2B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_WRSR_2B`"]
pub struct SPI_WRSR_2B_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_WRSR_2B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SPI_WP_REG`"]
pub type SPI_WP_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_WP_REG`"]
pub struct SPI_WP_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_WP_REG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SPI_FREAD_QUAD`"]
pub type SPI_FREAD_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FREAD_QUAD`"]
pub struct SPI_FREAD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FREAD_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SPI_RESANDRES`"]
pub type SPI_RESANDRES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_RESANDRES`"]
pub struct SPI_RESANDRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RESANDRES_W<'a> {
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
#[doc = "Reader of field `SPI_FREAD_DUAL`"]
pub type SPI_FREAD_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FREAD_DUAL`"]
pub struct SPI_FREAD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FREAD_DUAL_W<'a> {
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
#[doc = "Reader of field `SPI_FASTRD_MODE`"]
pub type SPI_FASTRD_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FASTRD_MODE`"]
pub struct SPI_FASTRD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FASTRD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SPI_WAIT_FLASH_IDLE_EN`"]
pub type SPI_WAIT_FLASH_IDLE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_WAIT_FLASH_IDLE_EN`"]
pub struct SPI_WAIT_FLASH_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_WAIT_FLASH_IDLE_EN_W<'a> {
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
#[doc = "Reader of field `SPI_TX_CRC_EN`"]
pub type SPI_TX_CRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_TX_CRC_EN`"]
pub struct SPI_TX_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TX_CRC_EN_W<'a> {
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
#[doc = "Reader of field `SPI_FCS_CRC_EN`"]
pub type SPI_FCS_CRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FCS_CRC_EN`"]
pub struct SPI_FCS_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FCS_CRC_EN_W<'a> {
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
impl R {
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first"]
    #[inline(always)]
    pub fn spi_wr_bit_order(&self) -> SPI_WR_BIT_ORDER_R {
        SPI_WR_BIT_ORDER_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first"]
    #[inline(always)]
    pub fn spi_rd_bit_order(&self) -> SPI_RD_BIT_ORDER_R {
        SPI_RD_BIT_ORDER_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fread_qio(&self) -> SPI_FREAD_QIO_R {
        SPI_FREAD_QIO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fread_dio(&self) -> SPI_FREAD_DIO_R {
        SPI_FREAD_DIO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_wrsr_2b(&self) -> SPI_WRSR_2B_R {
        SPI_WRSR_2B_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high 0: output low."]
    #[inline(always)]
    pub fn spi_wp_reg(&self) -> SPI_WP_REG_R {
        SPI_WP_REG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fread_quad(&self) -> SPI_FREAD_QUAD_R {
        SPI_FREAD_QUAD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_RD_STATUS register, this bit combine with spi_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_resandres(&self) -> SPI_RESANDRES_R {
        SPI_RESANDRES_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - In the read operations read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fread_dual(&self) -> SPI_FREAD_DUAL_R {
        SPI_FREAD_DUAL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_fread_qio spi_fread_dio spi_fread_qout and spi_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fastrd_mode(&self) -> SPI_FASTRD_MODE_R {
        SPI_FASTRD_MODE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - wait flash idle when program flash or erase flash. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_wait_flash_idle_en(&self) -> SPI_WAIT_FLASH_IDLE_EN_R {
        SPI_WAIT_FLASH_IDLE_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - For SPI1 enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    pub fn spi_tx_crc_en(&self) -> SPI_TX_CRC_EN_R {
        SPI_TX_CRC_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - For SPI1 initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    pub fn spi_fcs_crc_en(&self) -> SPI_FCS_CRC_EN_R {
        SPI_FCS_CRC_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first"]
    #[inline(always)]
    pub fn spi_wr_bit_order(&mut self) -> SPI_WR_BIT_ORDER_W {
        SPI_WR_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first"]
    #[inline(always)]
    pub fn spi_rd_bit_order(&mut self) -> SPI_RD_BIT_ORDER_W {
        SPI_RD_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fread_qio(&mut self) -> SPI_FREAD_QIO_W {
        SPI_FREAD_QIO_W { w: self }
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fread_dio(&mut self) -> SPI_FREAD_DIO_W {
        SPI_FREAD_DIO_W { w: self }
    }
    #[doc = "Bit 22 - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_wrsr_2b(&mut self) -> SPI_WRSR_2B_W {
        SPI_WRSR_2B_W { w: self }
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high 0: output low."]
    #[inline(always)]
    pub fn spi_wp_reg(&mut self) -> SPI_WP_REG_W {
        SPI_WP_REG_W { w: self }
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fread_quad(&mut self) -> SPI_FREAD_QUAD_W {
        SPI_FREAD_QUAD_W { w: self }
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_RD_STATUS register, this bit combine with spi_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_resandres(&mut self) -> SPI_RESANDRES_W {
        SPI_RESANDRES_W { w: self }
    }
    #[doc = "Bit 14 - In the read operations read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fread_dual(&mut self) -> SPI_FREAD_DUAL_W {
        SPI_FREAD_DUAL_W { w: self }
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_fread_qio spi_fread_dio spi_fread_qout and spi_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_fastrd_mode(&mut self) -> SPI_FASTRD_MODE_W {
        SPI_FASTRD_MODE_W { w: self }
    }
    #[doc = "Bit 12 - wait flash idle when program flash or erase flash. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_wait_flash_idle_en(&mut self) -> SPI_WAIT_FLASH_IDLE_EN_W {
        SPI_WAIT_FLASH_IDLE_EN_W { w: self }
    }
    #[doc = "Bit 11 - For SPI1 enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    pub fn spi_tx_crc_en(&mut self) -> SPI_TX_CRC_EN_W {
        SPI_TX_CRC_EN_W { w: self }
    }
    #[doc = "Bit 10 - For SPI1 initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    pub fn spi_fcs_crc_en(&mut self) -> SPI_FCS_CRC_EN_W {
        SPI_FCS_CRC_EN_W { w: self }
    }
}
