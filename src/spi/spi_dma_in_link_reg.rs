#[doc = "Reader of register SPI_DMA_IN_LINK_REG"]
pub type R = crate::R<u32, super::SPI_DMA_IN_LINK_REG>;
#[doc = "Writer for register SPI_DMA_IN_LINK_REG"]
pub type W = crate::W<u32, super::SPI_DMA_IN_LINK_REG>;
#[doc = "Register SPI_DMA_IN_LINK_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_DMA_IN_LINK_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_INLINK_RESTART`"]
pub type SPI_INLINK_RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_INLINK_RESTART`"]
pub struct SPI_INLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INLINK_RESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SPI_INLINK_START`"]
pub type SPI_INLINK_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_INLINK_START`"]
pub struct SPI_INLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INLINK_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SPI_INLINK_STOP`"]
pub type SPI_INLINK_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_INLINK_STOP`"]
pub struct SPI_INLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INLINK_STOP_W<'a> {
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
#[doc = "Reader of field `SPI_INLINK_AUTO_RET`"]
pub type SPI_INLINK_AUTO_RET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_INLINK_AUTO_RET`"]
pub struct SPI_INLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INLINK_AUTO_RET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SPI_INLINK_ADDR`"]
pub type SPI_INLINK_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_INLINK_ADDR`"]
pub struct SPI_INLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_INLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Set the bit to mount on new inlink descriptors."]
    #[inline(always)]
    pub fn spi_inlink_restart(&self) -> SPI_INLINK_RESTART_R {
        SPI_INLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Set the bit to start to use inlink descriptor."]
    #[inline(always)]
    pub fn spi_inlink_start(&self) -> SPI_INLINK_START_R {
        SPI_INLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set the bit to stop to use inlink descriptor."]
    #[inline(always)]
    pub fn spi_inlink_stop(&self) -> SPI_INLINK_STOP_R {
        SPI_INLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 20 - when the bit is set inlink descriptor returns to the next descriptor while a packet is wrong"]
    #[inline(always)]
    pub fn spi_inlink_auto_ret(&self) -> SPI_INLINK_AUTO_RET_R {
        SPI_INLINK_AUTO_RET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:19 - The address of the first inlink descriptor."]
    #[inline(always)]
    pub fn spi_inlink_addr(&self) -> SPI_INLINK_ADDR_R {
        SPI_INLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 30 - Set the bit to mount on new inlink descriptors."]
    #[inline(always)]
    pub fn spi_inlink_restart(&mut self) -> SPI_INLINK_RESTART_W {
        SPI_INLINK_RESTART_W { w: self }
    }
    #[doc = "Bit 29 - Set the bit to start to use inlink descriptor."]
    #[inline(always)]
    pub fn spi_inlink_start(&mut self) -> SPI_INLINK_START_W {
        SPI_INLINK_START_W { w: self }
    }
    #[doc = "Bit 28 - Set the bit to stop to use inlink descriptor."]
    #[inline(always)]
    pub fn spi_inlink_stop(&mut self) -> SPI_INLINK_STOP_W {
        SPI_INLINK_STOP_W { w: self }
    }
    #[doc = "Bit 20 - when the bit is set inlink descriptor returns to the next descriptor while a packet is wrong"]
    #[inline(always)]
    pub fn spi_inlink_auto_ret(&mut self) -> SPI_INLINK_AUTO_RET_W {
        SPI_INLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bits 0:19 - The address of the first inlink descriptor."]
    #[inline(always)]
    pub fn spi_inlink_addr(&mut self) -> SPI_INLINK_ADDR_W {
        SPI_INLINK_ADDR_W { w: self }
    }
}
