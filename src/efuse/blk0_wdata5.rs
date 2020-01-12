#[doc = "Reader of register BLK0_WDATA5"]
pub type R = crate::R<u32, super::BLK0_WDATA5>;
#[doc = "Writer for register BLK0_WDATA5"]
pub type W = crate::W<u32, super::BLK0_WDATA5>;
#[doc = "Register BLK0_WDATA5 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK0_WDATA5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_CRYPT_CONFIG`"]
pub type FLASH_CRYPT_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLASH_CRYPT_CONFIG`"]
pub struct FLASH_CRYPT_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_CRYPT_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `INST_CONFIG`"]
pub type INST_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INST_CONFIG`"]
pub struct INST_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> INST_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `SPI_PAD_CONFIG_D`"]
pub type SPI_PAD_CONFIG_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_PAD_CONFIG_D`"]
pub struct SPI_PAD_CONFIG_D_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_PAD_CONFIG_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPI_PAD_CONFIG_Q`"]
pub type SPI_PAD_CONFIG_Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_PAD_CONFIG_Q`"]
pub struct SPI_PAD_CONFIG_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_PAD_CONFIG_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `SPI_PAD_CONFIG_CLK`"]
pub type SPI_PAD_CONFIG_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_PAD_CONFIG_CLK`"]
pub struct SPI_PAD_CONFIG_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_PAD_CONFIG_CLK_W<'a> {
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
    pub fn flash_crypt_config(&self) -> FLASH_CRYPT_CONFIG_R {
        FLASH_CRYPT_CONFIG_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - program for SPI_pad_config_cs0"]
    #[inline(always)]
    pub fn inst_config(&self) -> INST_CONFIG_R {
        INST_CONFIG_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 10:14 - program for SPI_pad_config_d"]
    #[inline(always)]
    pub fn spi_pad_config_d(&self) -> SPI_PAD_CONFIG_D_R {
        SPI_PAD_CONFIG_D_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - program for SPI_pad_config_q"]
    #[inline(always)]
    pub fn spi_pad_config_q(&self) -> SPI_PAD_CONFIG_Q_R {
        SPI_PAD_CONFIG_Q_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - program for SPI_pad_config_clk"]
    #[inline(always)]
    pub fn spi_pad_config_clk(&self) -> SPI_PAD_CONFIG_CLK_R {
        SPI_PAD_CONFIG_CLK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - program for flash_crypt_config"]
    #[inline(always)]
    pub fn flash_crypt_config(&mut self) -> FLASH_CRYPT_CONFIG_W {
        FLASH_CRYPT_CONFIG_W { w: self }
    }
    #[doc = "Bits 20:27 - program for SPI_pad_config_cs0"]
    #[inline(always)]
    pub fn inst_config(&mut self) -> INST_CONFIG_W {
        INST_CONFIG_W { w: self }
    }
    #[doc = "Bits 10:14 - program for SPI_pad_config_d"]
    #[inline(always)]
    pub fn spi_pad_config_d(&mut self) -> SPI_PAD_CONFIG_D_W {
        SPI_PAD_CONFIG_D_W { w: self }
    }
    #[doc = "Bits 5:9 - program for SPI_pad_config_q"]
    #[inline(always)]
    pub fn spi_pad_config_q(&mut self) -> SPI_PAD_CONFIG_Q_W {
        SPI_PAD_CONFIG_Q_W { w: self }
    }
    #[doc = "Bits 0:4 - program for SPI_pad_config_clk"]
    #[inline(always)]
    pub fn spi_pad_config_clk(&mut self) -> SPI_PAD_CONFIG_CLK_W {
        SPI_PAD_CONFIG_CLK_W { w: self }
    }
}
