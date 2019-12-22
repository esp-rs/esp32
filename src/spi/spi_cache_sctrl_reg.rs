#[doc = "Reader of register SPI_CACHE_SCTRL_REG"]
pub type R = crate::R<u32, super::SPI_CACHE_SCTRL_REG>;
#[doc = "Writer for register SPI_CACHE_SCTRL_REG"]
pub type W = crate::W<u32, super::SPI_CACHE_SCTRL_REG>;
#[doc = "Register SPI_CACHE_SCTRL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CACHE_SCTRL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_CACHE_SRAM_USR_WCMD`"]
pub type SPI_CACHE_SRAM_USR_WCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CACHE_SRAM_USR_WCMD`"]
pub struct SPI_CACHE_SRAM_USR_WCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CACHE_SRAM_USR_WCMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPI_SRAM_ADDR_BITLEN`"]
pub type SPI_SRAM_ADDR_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SRAM_ADDR_BITLEN`"]
pub struct SPI_SRAM_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SRAM_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 22)) | (((value as u32) & 0x3f) << 22);
        self.w
    }
}
#[doc = "Reader of field `SPI_SRAM_DUMMY_CYCLELEN`"]
pub type SPI_SRAM_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SRAM_DUMMY_CYCLELEN`"]
pub struct SPI_SRAM_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SRAM_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | (((value as u32) & 0xff) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPI_SRAM_BYTES_LEN`"]
pub type SPI_SRAM_BYTES_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SRAM_BYTES_LEN`"]
pub struct SPI_SRAM_BYTES_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SRAM_BYTES_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | (((value as u32) & 0xff) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPI_CACHE_SRAM_USR_RCMD`"]
pub type SPI_CACHE_SRAM_USR_RCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CACHE_SRAM_USR_RCMD`"]
pub struct SPI_CACHE_SRAM_USR_RCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CACHE_SRAM_USR_RCMD_W<'a> {
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
#[doc = "Reader of field `SPI_USR_RD_SRAM_DUMMY`"]
pub type SPI_USR_RD_SRAM_DUMMY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_RD_SRAM_DUMMY`"]
pub struct SPI_USR_RD_SRAM_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_RD_SRAM_DUMMY_W<'a> {
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
#[doc = "Reader of field `SPI_USR_WR_SRAM_DUMMY`"]
pub type SPI_USR_WR_SRAM_DUMMY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_WR_SRAM_DUMMY`"]
pub struct SPI_USR_WR_SRAM_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_WR_SRAM_DUMMY_W<'a> {
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
#[doc = "Reader of field `SPI_USR_SRAM_QIO`"]
pub type SPI_USR_SRAM_QIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_SRAM_QIO`"]
pub struct SPI_USR_SRAM_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_SRAM_QIO_W<'a> {
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
#[doc = "Reader of field `SPI_USR_SRAM_DIO`"]
pub type SPI_USR_SRAM_DIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_SRAM_DIO`"]
pub struct SPI_USR_SRAM_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_SRAM_DIO_W<'a> {
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
impl R {
    #[doc = "Bit 28 - For SPI0 In the spi sram mode cache write sram for user define command"]
    #[inline(always)]
    pub fn spi_cache_sram_usr_wcmd(&self) -> SPI_CACHE_SRAM_USR_WCMD_R {
        SPI_CACHE_SRAM_USR_WCMD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 22:27 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_sram_addr_bitlen(&self) -> SPI_SRAM_ADDR_BITLEN_R {
        SPI_SRAM_ADDR_BITLEN_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 14:21 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_sram_dummy_cyclelen(&self) -> SPI_SRAM_DUMMY_CYCLELEN_R {
        SPI_SRAM_DUMMY_CYCLELEN_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 6:13 - For SPI0 In the sram mode it is the byte length of spi read sram data."]
    #[inline(always)]
    pub fn spi_sram_bytes_len(&self) -> SPI_SRAM_BYTES_LEN_R {
        SPI_SRAM_BYTES_LEN_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bit 5 - For SPI0 In the spi sram mode cache read sram for user define command."]
    #[inline(always)]
    pub fn spi_cache_sram_usr_rcmd(&self) -> SPI_CACHE_SRAM_USR_RCMD_R {
        SPI_CACHE_SRAM_USR_RCMD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn spi_usr_rd_sram_dummy(&self) -> SPI_USR_RD_SRAM_DUMMY_R {
        SPI_USR_RD_SRAM_DUMMY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn spi_usr_wr_sram_dummy(&self) -> SPI_USR_WR_SRAM_DUMMY_R {
        SPI_USR_WR_SRAM_DUMMY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn spi_usr_sram_qio(&self) -> SPI_USR_SRAM_QIO_R {
        SPI_USR_SRAM_QIO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn spi_usr_sram_dio(&self) -> SPI_USR_SRAM_DIO_R {
        SPI_USR_SRAM_DIO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - For SPI0 In the spi sram mode cache write sram for user define command"]
    #[inline(always)]
    pub fn spi_cache_sram_usr_wcmd(&mut self) -> SPI_CACHE_SRAM_USR_WCMD_W {
        SPI_CACHE_SRAM_USR_WCMD_W { w: self }
    }
    #[doc = "Bits 22:27 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_sram_addr_bitlen(&mut self) -> SPI_SRAM_ADDR_BITLEN_W {
        SPI_SRAM_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bits 14:21 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_sram_dummy_cyclelen(&mut self) -> SPI_SRAM_DUMMY_CYCLELEN_W {
        SPI_SRAM_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 6:13 - For SPI0 In the sram mode it is the byte length of spi read sram data."]
    #[inline(always)]
    pub fn spi_sram_bytes_len(&mut self) -> SPI_SRAM_BYTES_LEN_W {
        SPI_SRAM_BYTES_LEN_W { w: self }
    }
    #[doc = "Bit 5 - For SPI0 In the spi sram mode cache read sram for user define command."]
    #[inline(always)]
    pub fn spi_cache_sram_usr_rcmd(&mut self) -> SPI_CACHE_SRAM_USR_RCMD_W {
        SPI_CACHE_SRAM_USR_RCMD_W { w: self }
    }
    #[doc = "Bit 4 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn spi_usr_rd_sram_dummy(&mut self) -> SPI_USR_RD_SRAM_DUMMY_W {
        SPI_USR_RD_SRAM_DUMMY_W { w: self }
    }
    #[doc = "Bit 3 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn spi_usr_wr_sram_dummy(&mut self) -> SPI_USR_WR_SRAM_DUMMY_W {
        SPI_USR_WR_SRAM_DUMMY_W { w: self }
    }
    #[doc = "Bit 2 - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn spi_usr_sram_qio(&mut self) -> SPI_USR_SRAM_QIO_W {
        SPI_USR_SRAM_QIO_W { w: self }
    }
    #[doc = "Bit 1 - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn spi_usr_sram_dio(&mut self) -> SPI_USR_SRAM_DIO_W {
        SPI_USR_SRAM_DIO_W { w: self }
    }
}
