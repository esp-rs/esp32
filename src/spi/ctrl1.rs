#[doc = "Reader of register CTRL1"]
pub type R = crate::R<u32, super::CTRL1>;
#[doc = "Writer for register CTRL1"]
pub type W = crate::W<u32, super::CTRL1>;
#[doc = "Register CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_CS_HOLD_DELAY`"]
pub type SPI_CS_HOLD_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CS_HOLD_DELAY`"]
pub struct SPI_CS_HOLD_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_HOLD_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPI_CS_HOLD_DELAY_RES`"]
pub type SPI_CS_HOLD_DELAY_RES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_CS_HOLD_DELAY_RES`"]
pub struct SPI_CS_HOLD_DELAY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_HOLD_DELAY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - SPI cs signal is delayed by spi clock cycles"]
    #[inline(always)]
    pub fn spi_cs_hold_delay(&self) -> SPI_CS_HOLD_DELAY_R {
        SPI_CS_HOLD_DELAY_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
    #[inline(always)]
    pub fn spi_cs_hold_delay_res(&self) -> SPI_CS_HOLD_DELAY_RES_R {
        SPI_CS_HOLD_DELAY_RES_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31 - SPI cs signal is delayed by spi clock cycles"]
    #[inline(always)]
    pub fn spi_cs_hold_delay(&mut self) -> SPI_CS_HOLD_DELAY_W {
        SPI_CS_HOLD_DELAY_W { w: self }
    }
    #[doc = "Bits 16:27 - Delay cycles of resume Flash when resume Flash is enable by spi clock."]
    #[inline(always)]
    pub fn spi_cs_hold_delay_res(&mut self) -> SPI_CS_HOLD_DELAY_RES_W {
        SPI_CS_HOLD_DELAY_RES_W { w: self }
    }
}
