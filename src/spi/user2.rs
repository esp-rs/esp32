#[doc = "Reader of register USER2"]
pub type R = crate::R<u32, super::USER2>;
#[doc = "Writer for register USER2"]
pub type W = crate::W<u32, super::USER2>;
#[doc = "Register USER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::USER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_USR_COMMAND_BITLEN`"]
pub type SPI_USR_COMMAND_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_USR_COMMAND_BITLEN`"]
pub struct SPI_USR_COMMAND_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_COMMAND_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_COMMAND_VALUE`"]
pub type SPI_USR_COMMAND_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_USR_COMMAND_VALUE`"]
pub struct SPI_USR_COMMAND_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_COMMAND_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn spi_usr_command_bitlen(&self) -> SPI_USR_COMMAND_BITLEN_R {
        SPI_USR_COMMAND_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    pub fn spi_usr_command_value(&self) -> SPI_USR_COMMAND_VALUE_R {
        SPI_USR_COMMAND_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn spi_usr_command_bitlen(&mut self) -> SPI_USR_COMMAND_BITLEN_W {
        SPI_USR_COMMAND_BITLEN_W { w: self }
    }
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    pub fn spi_usr_command_value(&mut self) -> SPI_USR_COMMAND_VALUE_W {
        SPI_USR_COMMAND_VALUE_W { w: self }
    }
}
