#[doc = "Reader of register CACHE_FCTRL"]
pub type R = crate::R<u32, super::CACHE_FCTRL>;
#[doc = "Writer for register CACHE_FCTRL"]
pub type W = crate::W<u32, super::CACHE_FCTRL>;
#[doc = "Register CACHE_FCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CACHE_FCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_CACHE_FLASH_PES_EN`"]
pub type SPI_CACHE_FLASH_PES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CACHE_FLASH_PES_EN`"]
pub struct SPI_CACHE_FLASH_PES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CACHE_FLASH_PES_EN_W<'a> {
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
#[doc = "Reader of field `SPI_CACHE_FLASH_USR_CMD`"]
pub type SPI_CACHE_FLASH_USR_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CACHE_FLASH_USR_CMD`"]
pub struct SPI_CACHE_FLASH_USR_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CACHE_FLASH_USR_CMD_W<'a> {
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
#[doc = "Reader of field `SPI_CACHE_USR_CMD_4BYTE`"]
pub type SPI_CACHE_USR_CMD_4BYTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CACHE_USR_CMD_4BYTE`"]
pub struct SPI_CACHE_USR_CMD_4BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CACHE_USR_CMD_4BYTE_W<'a> {
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
#[doc = "Reader of field `SPI_CACHE_REQ_EN`"]
pub type SPI_CACHE_REQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CACHE_REQ_EN`"]
pub struct SPI_CACHE_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CACHE_REQ_EN_W<'a> {
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
    #[doc = "Bit 3 - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable."]
    #[inline(always)]
    pub fn spi_cache_flash_pes_en(&self) -> SPI_CACHE_FLASH_PES_EN_R {
        SPI_CACHE_FLASH_PES_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - For SPI0 cache read flash for user define command 1: enable 0:disable."]
    #[inline(always)]
    pub fn spi_cache_flash_usr_cmd(&self) -> SPI_CACHE_FLASH_USR_CMD_R {
        SPI_CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable."]
    #[inline(always)]
    pub fn spi_cache_usr_cmd_4byte(&self) -> SPI_CACHE_USR_CMD_4BYTE_R {
        SPI_CACHE_USR_CMD_4BYTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - For SPI0 Cache access enable 1: enable 0:disable."]
    #[inline(always)]
    pub fn spi_cache_req_en(&self) -> SPI_CACHE_REQ_EN_R {
        SPI_CACHE_REQ_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable."]
    #[inline(always)]
    pub fn spi_cache_flash_pes_en(&mut self) -> SPI_CACHE_FLASH_PES_EN_W {
        SPI_CACHE_FLASH_PES_EN_W { w: self }
    }
    #[doc = "Bit 2 - For SPI0 cache read flash for user define command 1: enable 0:disable."]
    #[inline(always)]
    pub fn spi_cache_flash_usr_cmd(&mut self) -> SPI_CACHE_FLASH_USR_CMD_W {
        SPI_CACHE_FLASH_USR_CMD_W { w: self }
    }
    #[doc = "Bit 1 - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable."]
    #[inline(always)]
    pub fn spi_cache_usr_cmd_4byte(&mut self) -> SPI_CACHE_USR_CMD_4BYTE_W {
        SPI_CACHE_USR_CMD_4BYTE_W { w: self }
    }
    #[doc = "Bit 0 - For SPI0 Cache access enable 1: enable 0:disable."]
    #[inline(always)]
    pub fn spi_cache_req_en(&mut self) -> SPI_CACHE_REQ_EN_W {
        SPI_CACHE_REQ_EN_W { w: self }
    }
}
