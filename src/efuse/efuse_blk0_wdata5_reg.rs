#[doc = "Reader of register EFUSE_BLK0_WDATA5_REG"]
pub type R = crate::R<u32, super::EFUSE_BLK0_WDATA5_REG>;
#[doc = "Writer for register EFUSE_BLK0_WDATA5_REG"]
pub type W = crate::W<u32, super::EFUSE_BLK0_WDATA5_REG>;
#[doc = "Register EFUSE_BLK0_WDATA5_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_BLK0_WDATA5_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_FLASH_CRYPT_CONFIG`"]
pub type EFUSE_FLASH_CRYPT_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_FLASH_CRYPT_CONFIG`"]
pub struct EFUSE_FLASH_CRYPT_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_FLASH_CRYPT_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_INST_CONFIG`"]
pub type EFUSE_INST_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_INST_CONFIG`"]
pub struct EFUSE_INST_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_INST_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_SPI_PAD_CONFIG_D`"]
pub type EFUSE_SPI_PAD_CONFIG_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_SPI_PAD_CONFIG_D`"]
pub struct EFUSE_SPI_PAD_CONFIG_D_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SPI_PAD_CONFIG_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_SPI_PAD_CONFIG_Q`"]
pub type EFUSE_SPI_PAD_CONFIG_Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_SPI_PAD_CONFIG_Q`"]
pub struct EFUSE_SPI_PAD_CONFIG_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SPI_PAD_CONFIG_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_SPI_PAD_CONFIG_CLK`"]
pub type EFUSE_SPI_PAD_CONFIG_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_SPI_PAD_CONFIG_CLK`"]
pub struct EFUSE_SPI_PAD_CONFIG_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_SPI_PAD_CONFIG_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - program for flash_crypt_config"]
    #[inline(always)]
    pub fn efuse_flash_crypt_config(&self) -> EFUSE_FLASH_CRYPT_CONFIG_R {
        EFUSE_FLASH_CRYPT_CONFIG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - program for SPI_pad_config_cs0"]
    #[inline(always)]
    pub fn efuse_inst_config(&self) -> EFUSE_INST_CONFIG_R {
        EFUSE_INST_CONFIG_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 10:14 - program for SPI_pad_config_d"]
    #[inline(always)]
    pub fn efuse_spi_pad_config_d(&self) -> EFUSE_SPI_PAD_CONFIG_D_R {
        EFUSE_SPI_PAD_CONFIG_D_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - program for SPI_pad_config_q"]
    #[inline(always)]
    pub fn efuse_spi_pad_config_q(&self) -> EFUSE_SPI_PAD_CONFIG_Q_R {
        EFUSE_SPI_PAD_CONFIG_Q_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - program for SPI_pad_config_clk"]
    #[inline(always)]
    pub fn efuse_spi_pad_config_clk(&self) -> EFUSE_SPI_PAD_CONFIG_CLK_R {
        EFUSE_SPI_PAD_CONFIG_CLK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - program for flash_crypt_config"]
    #[inline(always)]
    pub fn efuse_flash_crypt_config(&mut self) -> EFUSE_FLASH_CRYPT_CONFIG_W {
        EFUSE_FLASH_CRYPT_CONFIG_W { w: self }
    }
    #[doc = "Bits 20:27 - program for SPI_pad_config_cs0"]
    #[inline(always)]
    pub fn efuse_inst_config(&mut self) -> EFUSE_INST_CONFIG_W {
        EFUSE_INST_CONFIG_W { w: self }
    }
    #[doc = "Bits 10:14 - program for SPI_pad_config_d"]
    #[inline(always)]
    pub fn efuse_spi_pad_config_d(&mut self) -> EFUSE_SPI_PAD_CONFIG_D_W {
        EFUSE_SPI_PAD_CONFIG_D_W { w: self }
    }
    #[doc = "Bits 5:9 - program for SPI_pad_config_q"]
    #[inline(always)]
    pub fn efuse_spi_pad_config_q(&mut self) -> EFUSE_SPI_PAD_CONFIG_Q_W {
        EFUSE_SPI_PAD_CONFIG_Q_W { w: self }
    }
    #[doc = "Bits 0:4 - program for SPI_pad_config_clk"]
    #[inline(always)]
    pub fn efuse_spi_pad_config_clk(&mut self) -> EFUSE_SPI_PAD_CONFIG_CLK_W {
        EFUSE_SPI_PAD_CONFIG_CLK_W { w: self }
    }
}
