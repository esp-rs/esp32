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
#[doc = "Reader of field `CACHE_FLASH_PES_EN`"]
pub type CACHE_FLASH_PES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_FLASH_PES_EN`"]
pub struct CACHE_FLASH_PES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_FLASH_PES_EN_W<'a> {
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
#[doc = "Reader of field `CACHE_FLASH_USR_CMD`"]
pub type CACHE_FLASH_USR_CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_FLASH_USR_CMD`"]
pub struct CACHE_FLASH_USR_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_FLASH_USR_CMD_W<'a> {
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
#[doc = "Reader of field `CACHE_USR_CMD_4BYTE`"]
pub type CACHE_USR_CMD_4BYTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_USR_CMD_4BYTE`"]
pub struct CACHE_USR_CMD_4BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_USR_CMD_4BYTE_W<'a> {
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
#[doc = "Reader of field `CACHE_REQ_EN`"]
pub type CACHE_REQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_REQ_EN`"]
pub struct CACHE_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_REQ_EN_W<'a> {
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
    pub fn cache_flash_pes_en(&self) -> CACHE_FLASH_PES_EN_R {
        CACHE_FLASH_PES_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - For SPI0 cache read flash for user define command 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&self) -> CACHE_FLASH_USR_CMD_R {
        CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&self) -> CACHE_USR_CMD_4BYTE_R {
        CACHE_USR_CMD_4BYTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - For SPI0 Cache access enable 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_req_en(&self) -> CACHE_REQ_EN_R {
        CACHE_REQ_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - For SPI0 spi1 send suspend command before cache read flash 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_flash_pes_en(&mut self) -> CACHE_FLASH_PES_EN_W {
        CACHE_FLASH_PES_EN_W { w: self }
    }
    #[doc = "Bit 2 - For SPI0 cache read flash for user define command 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&mut self) -> CACHE_FLASH_USR_CMD_W {
        CACHE_FLASH_USR_CMD_W { w: self }
    }
    #[doc = "Bit 1 - For SPI0 cache read flash with 4 bytes command 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&mut self) -> CACHE_USR_CMD_4BYTE_W {
        CACHE_USR_CMD_4BYTE_W { w: self }
    }
    #[doc = "Bit 0 - For SPI0 Cache access enable 1: enable 0:disable."]
    #[inline(always)]
    pub fn cache_req_en(&mut self) -> CACHE_REQ_EN_W {
        CACHE_REQ_EN_W { w: self }
    }
}
