#[doc = "Reader of register SPI_EXT0_REG"]
pub type R = crate::R<u32, super::SPI_EXT0_REG>;
#[doc = "Writer for register SPI_EXT0_REG"]
pub type W = crate::W<u32, super::SPI_EXT0_REG>;
#[doc = "Register SPI_EXT0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_EXT0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_T_PP_ENA`"]
pub type SPI_T_PP_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_T_PP_ENA`"]
pub struct SPI_T_PP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_T_PP_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SPI_T_PP_SHIFT`"]
pub type SPI_T_PP_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_T_PP_SHIFT`"]
pub struct SPI_T_PP_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_T_PP_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_T_PP_TIME`"]
pub type SPI_T_PP_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_T_PP_TIME`"]
pub struct SPI_T_PP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_T_PP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - page program delay enable."]
    #[inline(always)]
    pub fn spi_t_pp_ena(&self) -> SPI_T_PP_ENA_R {
        SPI_T_PP_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - page program delay time shift ."]
    #[inline(always)]
    pub fn spi_t_pp_shift(&self) -> SPI_T_PP_SHIFT_R {
        SPI_T_PP_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - page program delay time by system clock."]
    #[inline(always)]
    pub fn spi_t_pp_time(&self) -> SPI_T_PP_TIME_R {
        SPI_T_PP_TIME_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - page program delay enable."]
    #[inline(always)]
    pub fn spi_t_pp_ena(&mut self) -> SPI_T_PP_ENA_W {
        SPI_T_PP_ENA_W { w: self }
    }
    #[doc = "Bits 16:19 - page program delay time shift ."]
    #[inline(always)]
    pub fn spi_t_pp_shift(&mut self) -> SPI_T_PP_SHIFT_W {
        SPI_T_PP_SHIFT_W { w: self }
    }
    #[doc = "Bits 0:11 - page program delay time by system clock."]
    #[inline(always)]
    pub fn spi_t_pp_time(&mut self) -> SPI_T_PP_TIME_W {
        SPI_T_PP_TIME_W { w: self }
    }
}
