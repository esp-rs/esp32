#[doc = "Reader of register SRAM_DRD_CMD"]
pub type R = crate::R<u32, super::SRAM_DRD_CMD>;
#[doc = "Writer for register SRAM_DRD_CMD"]
pub type W = crate::W<u32, super::SRAM_DRD_CMD>;
#[doc = "Register SRAM_DRD_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAM_DRD_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_CACHE_SRAM_USR_RD_CMD_BITLEN`"]
pub type SPI_CACHE_SRAM_USR_RD_CMD_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CACHE_SRAM_USR_RD_CMD_BITLEN`"]
pub struct SPI_CACHE_SRAM_USR_RD_CMD_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CACHE_SRAM_USR_RD_CMD_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPI_CACHE_SRAM_USR_RD_CMD_VALUE`"]
pub type SPI_CACHE_SRAM_USR_RD_CMD_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_CACHE_SRAM_USR_RD_CMD_VALUE`"]
pub struct SPI_CACHE_SRAM_USR_RD_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CACHE_SRAM_USR_RD_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - For SPI0 When cache mode is enable it is the length in bits of command phase for SRAM. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_cache_sram_usr_rd_cmd_bitlen(&self) -> SPI_CACHE_SRAM_USR_RD_CMD_BITLEN_R {
        SPI_CACHE_SRAM_USR_RD_CMD_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - For SPI0 When cache mode is enable it is the read command value of command phase for SRAM."]
    #[inline(always)]
    pub fn spi_cache_sram_usr_rd_cmd_value(&self) -> SPI_CACHE_SRAM_USR_RD_CMD_VALUE_R {
        SPI_CACHE_SRAM_USR_RD_CMD_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31 - For SPI0 When cache mode is enable it is the length in bits of command phase for SRAM. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_cache_sram_usr_rd_cmd_bitlen(&mut self) -> SPI_CACHE_SRAM_USR_RD_CMD_BITLEN_W {
        SPI_CACHE_SRAM_USR_RD_CMD_BITLEN_W { w: self }
    }
    #[doc = "Bits 0:15 - For SPI0 When cache mode is enable it is the read command value of command phase for SRAM."]
    #[inline(always)]
    pub fn spi_cache_sram_usr_rd_cmd_value(&mut self) -> SPI_CACHE_SRAM_USR_RD_CMD_VALUE_W {
        SPI_CACHE_SRAM_USR_RD_CMD_VALUE_W { w: self }
    }
}
