#[doc = "Reader of register SPI_RD_STATUS_REG"]
pub type R = crate::R<u32, super::SPI_RD_STATUS_REG>;
#[doc = "Writer for register SPI_RD_STATUS_REG"]
pub type W = crate::W<u32, super::SPI_RD_STATUS_REG>;
#[doc = "Register SPI_RD_STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_RD_STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_STATUS_EXT`"]
pub type SPI_STATUS_EXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_STATUS_EXT`"]
pub struct SPI_STATUS_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_STATUS_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI_WB_MODE`"]
pub type SPI_WB_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_WB_MODE`"]
pub struct SPI_WB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_WB_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_STATUS`"]
pub type SPI_STATUS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_STATUS`"]
pub struct SPI_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - In the slave mode,it is the status for master to read out."]
    #[inline(always)]
    pub fn spi_status_ext(&self) -> SPI_STATUS_EXT_R {
        SPI_STATUS_EXT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode, it is combined with spi_fastrd_mode bit."]
    #[inline(always)]
    pub fn spi_wb_mode(&self) -> SPI_WB_MODE_R {
        SPI_WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - In the slave mode, it is the status for master to read out."]
    #[inline(always)]
    pub fn spi_status(&self) -> SPI_STATUS_R {
        SPI_STATUS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - In the slave mode,it is the status for master to read out."]
    #[inline(always)]
    pub fn spi_status_ext(&mut self) -> SPI_STATUS_EXT_W {
        SPI_STATUS_EXT_W { w: self }
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode, it is combined with spi_fastrd_mode bit."]
    #[inline(always)]
    pub fn spi_wb_mode(&mut self) -> SPI_WB_MODE_W {
        SPI_WB_MODE_W { w: self }
    }
    #[doc = "Bits 0:15 - In the slave mode, it is the status for master to read out."]
    #[inline(always)]
    pub fn spi_status(&mut self) -> SPI_STATUS_W {
        SPI_STATUS_W { w: self }
    }
}
