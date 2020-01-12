#[doc = "Reader of register CACHE_SCTRL"]
pub type R = crate::R<u32, super::CACHE_SCTRL>;
#[doc = "Writer for register CACHE_SCTRL"]
pub type W = crate::W<u32, super::CACHE_SCTRL>;
#[doc = "Register CACHE_SCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CACHE_SCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CACHE_SRAM_USR_WCMD`"]
pub type CACHE_SRAM_USR_WCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_SRAM_USR_WCMD`"]
pub struct CACHE_SRAM_USR_WCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_WCMD_W<'a> {
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
#[doc = "Reader of field `SRAM_ADDR_BITLEN`"]
pub type SRAM_ADDR_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRAM_ADDR_BITLEN`"]
pub struct SRAM_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 22)) | (((value as u32) & 0x3f) << 22);
        self.w
    }
}
#[doc = "Reader of field `SRAM_DUMMY_CYCLELEN`"]
pub type SRAM_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRAM_DUMMY_CYCLELEN`"]
pub struct SRAM_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | (((value as u32) & 0xff) << 14);
        self.w
    }
}
#[doc = "Reader of field `SRAM_BYTES_LEN`"]
pub type SRAM_BYTES_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRAM_BYTES_LEN`"]
pub struct SRAM_BYTES_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_BYTES_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | (((value as u32) & 0xff) << 6);
        self.w
    }
}
#[doc = "Reader of field `CACHE_SRAM_USR_RCMD`"]
pub type CACHE_SRAM_USR_RCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_SRAM_USR_RCMD`"]
pub struct CACHE_SRAM_USR_RCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_RCMD_W<'a> {
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
#[doc = "Reader of field `USR_RD_SRAM_DUMMY`"]
pub type USR_RD_SRAM_DUMMY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_RD_SRAM_DUMMY`"]
pub struct USR_RD_SRAM_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_RD_SRAM_DUMMY_W<'a> {
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
#[doc = "Reader of field `USR_WR_SRAM_DUMMY`"]
pub type USR_WR_SRAM_DUMMY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_WR_SRAM_DUMMY`"]
pub struct USR_WR_SRAM_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_WR_SRAM_DUMMY_W<'a> {
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
#[doc = "Reader of field `USR_SRAM_QIO`"]
pub type USR_SRAM_QIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_SRAM_QIO`"]
pub struct USR_SRAM_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_SRAM_QIO_W<'a> {
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
#[doc = "Reader of field `USR_SRAM_DIO`"]
pub type USR_SRAM_DIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USR_SRAM_DIO`"]
pub struct USR_SRAM_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_SRAM_DIO_W<'a> {
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
    pub fn cache_sram_usr_wcmd(&self) -> CACHE_SRAM_USR_WCMD_R {
        CACHE_SRAM_USR_WCMD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 22:27 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&self) -> SRAM_ADDR_BITLEN_R {
        SRAM_ADDR_BITLEN_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 14:21 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_dummy_cyclelen(&self) -> SRAM_DUMMY_CYCLELEN_R {
        SRAM_DUMMY_CYCLELEN_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 6:13 - For SPI0 In the sram mode it is the byte length of spi read sram data."]
    #[inline(always)]
    pub fn sram_bytes_len(&self) -> SRAM_BYTES_LEN_R {
        SRAM_BYTES_LEN_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bit 5 - For SPI0 In the spi sram mode cache read sram for user define command."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&self) -> CACHE_SRAM_USR_RCMD_R {
        CACHE_SRAM_USR_RCMD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&self) -> USR_RD_SRAM_DUMMY_R {
        USR_RD_SRAM_DUMMY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&self) -> USR_WR_SRAM_DUMMY_R {
        USR_WR_SRAM_DUMMY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_qio(&self) -> USR_SRAM_QIO_R {
        USR_SRAM_QIO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_dio(&self) -> USR_SRAM_DIO_R {
        USR_SRAM_DIO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - For SPI0 In the spi sram mode cache write sram for user define command"]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&mut self) -> CACHE_SRAM_USR_WCMD_W {
        CACHE_SRAM_USR_WCMD_W { w: self }
    }
    #[doc = "Bits 22:27 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&mut self) -> SRAM_ADDR_BITLEN_W {
        SRAM_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bits 14:21 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_dummy_cyclelen(&mut self) -> SRAM_DUMMY_CYCLELEN_W {
        SRAM_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 6:13 - For SPI0 In the sram mode it is the byte length of spi read sram data."]
    #[inline(always)]
    pub fn sram_bytes_len(&mut self) -> SRAM_BYTES_LEN_W {
        SRAM_BYTES_LEN_W { w: self }
    }
    #[doc = "Bit 5 - For SPI0 In the spi sram mode cache read sram for user define command."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&mut self) -> CACHE_SRAM_USR_RCMD_W {
        CACHE_SRAM_USR_RCMD_W { w: self }
    }
    #[doc = "Bit 4 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&mut self) -> USR_RD_SRAM_DUMMY_W {
        USR_RD_SRAM_DUMMY_W { w: self }
    }
    #[doc = "Bit 3 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&mut self) -> USR_WR_SRAM_DUMMY_W {
        USR_WR_SRAM_DUMMY_W { w: self }
    }
    #[doc = "Bit 2 - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_qio(&mut self) -> USR_SRAM_QIO_W {
        USR_SRAM_QIO_W { w: self }
    }
    #[doc = "Bit 1 - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_dio(&mut self) -> USR_SRAM_DIO_W {
        USR_SRAM_DIO_W { w: self }
    }
}
