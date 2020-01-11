#[doc = "Reader of register SLAVE3"]
pub type R = crate::R<u32, super::SLAVE3>;
#[doc = "Writer for register SLAVE3"]
pub type W = crate::W<u32, super::SLAVE3>;
#[doc = "Register SLAVE3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SLAVE3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_SLV_WRSTA_CMD_VALUE`"]
pub type SPI_SLV_WRSTA_CMD_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_WRSTA_CMD_VALUE`"]
pub struct SPI_SLV_WRSTA_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WRSTA_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_RDSTA_CMD_VALUE`"]
pub type SPI_SLV_RDSTA_CMD_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_RDSTA_CMD_VALUE`"]
pub struct SPI_SLV_RDSTA_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RDSTA_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_WRBUF_CMD_VALUE`"]
pub type SPI_SLV_WRBUF_CMD_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_WRBUF_CMD_VALUE`"]
pub struct SPI_SLV_WRBUF_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WRBUF_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_RDBUF_CMD_VALUE`"]
pub type SPI_SLV_RDBUF_CMD_VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_RDBUF_CMD_VALUE`"]
pub struct SPI_SLV_RDBUF_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RDBUF_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - In the slave mode it is the value of write-status command."]
    #[inline(always)]
    pub fn spi_slv_wrsta_cmd_value(&self) -> SPI_SLV_WRSTA_CMD_VALUE_R {
        SPI_SLV_WRSTA_CMD_VALUE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of read-status command."]
    #[inline(always)]
    pub fn spi_slv_rdsta_cmd_value(&self) -> SPI_SLV_RDSTA_CMD_VALUE_R {
        SPI_SLV_RDSTA_CMD_VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the slave mode it is the value of write-buffer command."]
    #[inline(always)]
    pub fn spi_slv_wrbuf_cmd_value(&self) -> SPI_SLV_WRBUF_CMD_VALUE_R {
        SPI_SLV_WRBUF_CMD_VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - In the slave mode it is the value of read-buffer command."]
    #[inline(always)]
    pub fn spi_slv_rdbuf_cmd_value(&self) -> SPI_SLV_RDBUF_CMD_VALUE_R {
        SPI_SLV_RDBUF_CMD_VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - In the slave mode it is the value of write-status command."]
    #[inline(always)]
    pub fn spi_slv_wrsta_cmd_value(&mut self) -> SPI_SLV_WRSTA_CMD_VALUE_W {
        SPI_SLV_WRSTA_CMD_VALUE_W { w: self }
    }
    #[doc = "Bits 16:23 - In the slave mode it is the value of read-status command."]
    #[inline(always)]
    pub fn spi_slv_rdsta_cmd_value(&mut self) -> SPI_SLV_RDSTA_CMD_VALUE_W {
        SPI_SLV_RDSTA_CMD_VALUE_W { w: self }
    }
    #[doc = "Bits 8:15 - In the slave mode it is the value of write-buffer command."]
    #[inline(always)]
    pub fn spi_slv_wrbuf_cmd_value(&mut self) -> SPI_SLV_WRBUF_CMD_VALUE_W {
        SPI_SLV_WRBUF_CMD_VALUE_W { w: self }
    }
    #[doc = "Bits 0:7 - In the slave mode it is the value of read-buffer command."]
    #[inline(always)]
    pub fn spi_slv_rdbuf_cmd_value(&mut self) -> SPI_SLV_RDBUF_CMD_VALUE_W {
        SPI_SLV_RDBUF_CMD_VALUE_W { w: self }
    }
}
