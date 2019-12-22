#[doc = "Reader of register SPI_SLAVE1_REG"]
pub type R = crate::R<u32, super::SPI_SLAVE1_REG>;
#[doc = "Writer for register SPI_SLAVE1_REG"]
pub type W = crate::W<u32, super::SPI_SLAVE1_REG>;
#[doc = "Register SPI_SLAVE1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_SLAVE1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_SLV_STATUS_BITLEN`"]
pub type SPI_SLV_STATUS_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_STATUS_BITLEN`"]
pub struct SPI_SLV_STATUS_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_STATUS_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_STATUS_FAST_EN`"]
pub type SPI_SLV_STATUS_FAST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_STATUS_FAST_EN`"]
pub struct SPI_SLV_STATUS_FAST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_STATUS_FAST_EN_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_STATUS_READBACK`"]
pub type SPI_SLV_STATUS_READBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_STATUS_READBACK`"]
pub struct SPI_SLV_STATUS_READBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_STATUS_READBACK_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_RD_ADDR_BITLEN`"]
pub type SPI_SLV_RD_ADDR_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_RD_ADDR_BITLEN`"]
pub struct SPI_SLV_RD_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RD_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_WR_ADDR_BITLEN`"]
pub type SPI_SLV_WR_ADDR_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_WR_ADDR_BITLEN`"]
pub struct SPI_SLV_WR_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WR_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_WRSTA_DUMMY_EN`"]
pub type SPI_SLV_WRSTA_DUMMY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_WRSTA_DUMMY_EN`"]
pub struct SPI_SLV_WRSTA_DUMMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WRSTA_DUMMY_EN_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_RDSTA_DUMMY_EN`"]
pub type SPI_SLV_RDSTA_DUMMY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_RDSTA_DUMMY_EN`"]
pub struct SPI_SLV_RDSTA_DUMMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RDSTA_DUMMY_EN_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_WRBUF_DUMMY_EN`"]
pub type SPI_SLV_WRBUF_DUMMY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_WRBUF_DUMMY_EN`"]
pub struct SPI_SLV_WRBUF_DUMMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WRBUF_DUMMY_EN_W<'a> {
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
#[doc = "Reader of field `SPI_SLV_RDBUF_DUMMY_EN`"]
pub type SPI_SLV_RDBUF_DUMMY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLV_RDBUF_DUMMY_EN`"]
pub struct SPI_SLV_RDBUF_DUMMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RDBUF_DUMMY_EN_W<'a> {
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
    #[doc = "Bits 27:31 - In the slave mode it is the length of status bit."]
    #[inline(always)]
    pub fn spi_slv_status_bitlen(&self) -> SPI_SLV_STATUS_BITLEN_R {
        SPI_SLV_STATUS_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - In the slave mode enable fast read status."]
    #[inline(always)]
    pub fn spi_slv_status_fast_en(&self) -> SPI_SLV_STATUS_FAST_EN_R {
        SPI_SLV_STATUS_FAST_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
    #[inline(always)]
    pub fn spi_slv_status_readback(&self) -> SPI_SLV_STATUS_READBACK_R {
        SPI_SLV_STATUS_READBACK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_slv_rd_addr_bitlen(&self) -> SPI_SLV_RD_ADDR_BITLEN_R {
        SPI_SLV_RD_ADDR_BITLEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 4:9 - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_slv_wr_addr_bitlen(&self) -> SPI_SLV_WR_ADDR_BITLEN_R {
        SPI_SLV_WR_ADDR_BITLEN_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 3 - In the slave mode it is the enable bit of dummy phase for write-status operations."]
    #[inline(always)]
    pub fn spi_slv_wrsta_dummy_en(&self) -> SPI_SLV_WRSTA_DUMMY_EN_R {
        SPI_SLV_WRSTA_DUMMY_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - In the slave mode it is the enable bit of dummy phase for read-status operations."]
    #[inline(always)]
    pub fn spi_slv_rdsta_dummy_en(&self) -> SPI_SLV_RDSTA_DUMMY_EN_R {
        SPI_SLV_RDSTA_DUMMY_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
    #[inline(always)]
    pub fn spi_slv_wrbuf_dummy_en(&self) -> SPI_SLV_WRBUF_DUMMY_EN_R {
        SPI_SLV_WRBUF_DUMMY_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
    #[inline(always)]
    pub fn spi_slv_rdbuf_dummy_en(&self) -> SPI_SLV_RDBUF_DUMMY_EN_R {
        SPI_SLV_RDBUF_DUMMY_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:31 - In the slave mode it is the length of status bit."]
    #[inline(always)]
    pub fn spi_slv_status_bitlen(&mut self) -> SPI_SLV_STATUS_BITLEN_W {
        SPI_SLV_STATUS_BITLEN_W { w: self }
    }
    #[doc = "Bit 26 - In the slave mode enable fast read status."]
    #[inline(always)]
    pub fn spi_slv_status_fast_en(&mut self) -> SPI_SLV_STATUS_FAST_EN_W {
        SPI_SLV_STATUS_FAST_EN_W { w: self }
    }
    #[doc = "Bit 25 - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
    #[inline(always)]
    pub fn spi_slv_status_readback(&mut self) -> SPI_SLV_STATUS_READBACK_W {
        SPI_SLV_STATUS_READBACK_W { w: self }
    }
    #[doc = "Bits 10:15 - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_slv_rd_addr_bitlen(&mut self) -> SPI_SLV_RD_ADDR_BITLEN_W {
        SPI_SLV_RD_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bits 4:9 - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_slv_wr_addr_bitlen(&mut self) -> SPI_SLV_WR_ADDR_BITLEN_W {
        SPI_SLV_WR_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bit 3 - In the slave mode it is the enable bit of dummy phase for write-status operations."]
    #[inline(always)]
    pub fn spi_slv_wrsta_dummy_en(&mut self) -> SPI_SLV_WRSTA_DUMMY_EN_W {
        SPI_SLV_WRSTA_DUMMY_EN_W { w: self }
    }
    #[doc = "Bit 2 - In the slave mode it is the enable bit of dummy phase for read-status operations."]
    #[inline(always)]
    pub fn spi_slv_rdsta_dummy_en(&mut self) -> SPI_SLV_RDSTA_DUMMY_EN_W {
        SPI_SLV_RDSTA_DUMMY_EN_W { w: self }
    }
    #[doc = "Bit 1 - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
    #[inline(always)]
    pub fn spi_slv_wrbuf_dummy_en(&mut self) -> SPI_SLV_WRBUF_DUMMY_EN_W {
        SPI_SLV_WRBUF_DUMMY_EN_W { w: self }
    }
    #[doc = "Bit 0 - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
    #[inline(always)]
    pub fn spi_slv_rdbuf_dummy_en(&mut self) -> SPI_SLV_RDBUF_DUMMY_EN_W {
        SPI_SLV_RDBUF_DUMMY_EN_W { w: self }
    }
}
