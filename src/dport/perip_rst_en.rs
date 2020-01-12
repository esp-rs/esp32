#[doc = "Reader of register PERIP_RST_EN"]
pub type R = crate::R<u32, super::PERIP_RST_EN>;
#[doc = "Writer for register PERIP_RST_EN"]
pub type W = crate::W<u32, super::PERIP_RST_EN>;
#[doc = "Register PERIP_RST_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::PERIP_RST_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERIP_RST`"]
pub type PERIP_RST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PERIP_RST`"]
pub struct PERIP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIP_RST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
#[doc = "Reader of field `SPI_DECRYPT_ENABLE`"]
pub type SPI_DECRYPT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DECRYPT_ENABLE`"]
pub struct SPI_DECRYPT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DECRYPT_ENABLE_W<'a> {
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
#[doc = "Reader of field `SPI_ENCRYPT_ENABLE`"]
pub type SPI_ENCRYPT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_ENCRYPT_ENABLE`"]
pub struct SPI_ENCRYPT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_ENCRYPT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_SPI_MASK_APP`"]
pub type SLAVE_SPI_MASK_APP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_SPI_MASK_APP`"]
pub struct SLAVE_SPI_MASK_APP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SPI_MASK_APP_W<'a> {
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
#[doc = "Reader of field `SLAVE_SPI_MASK_PRO`"]
pub type SLAVE_SPI_MASK_PRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_SPI_MASK_PRO`"]
pub struct SLAVE_SPI_MASK_PRO_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SPI_MASK_PRO_W<'a> {
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
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn perip_rst(&self) -> PERIP_RST_R {
        PERIP_RST_R::new((self.bits & 0xffff_ffff) as u32)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_decrypt_enable(&self) -> SPI_DECRYPT_ENABLE_R {
        SPI_DECRYPT_ENABLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_encrypt_enable(&self) -> SPI_ENCRYPT_ENABLE_R {
        SPI_ENCRYPT_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slave_spi_mask_app(&self) -> SLAVE_SPI_MASK_APP_R {
        SLAVE_SPI_MASK_APP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slave_spi_mask_pro(&self) -> SLAVE_SPI_MASK_PRO_R {
        SLAVE_SPI_MASK_PRO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn perip_rst(&mut self) -> PERIP_RST_W {
        PERIP_RST_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_decrypt_enable(&mut self) -> SPI_DECRYPT_ENABLE_W {
        SPI_DECRYPT_ENABLE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_encrypt_enable(&mut self) -> SPI_ENCRYPT_ENABLE_W {
        SPI_ENCRYPT_ENABLE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slave_spi_mask_app(&mut self) -> SLAVE_SPI_MASK_APP_W {
        SLAVE_SPI_MASK_APP_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slave_spi_mask_pro(&mut self) -> SLAVE_SPI_MASK_PRO_W {
        SLAVE_SPI_MASK_PRO_W { w: self }
    }
}
