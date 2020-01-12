#[doc = "Reader of register SRAM_DWR_CMD"]
pub type R = crate::R<u32, super::SRAM_DWR_CMD>;
#[doc = "Writer for register SRAM_DWR_CMD"]
pub type W = crate::W<u32, super::SRAM_DWR_CMD>;
#[doc = "Register SRAM_DWR_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAM_DWR_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CACHE_SRAM_USR_WR_CMD_BITLEN`"]
pub type CACHE_SRAM_USR_WR_CMD_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CACHE_SRAM_USR_WR_CMD_BITLEN`"]
pub struct CACHE_SRAM_USR_WR_CMD_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_WR_CMD_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `CACHE_SRAM_USR_WR_CMD_VALUE`"]
pub type CACHE_SRAM_USR_WR_CMD_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CACHE_SRAM_USR_WR_CMD_VALUE`"]
pub struct CACHE_SRAM_USR_WR_CMD_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_SRAM_USR_WR_CMD_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - For SPI0 When cache mode is enable it is the in bits of command phase for SRAM. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_bitlen(&self) -> CACHE_SRAM_USR_WR_CMD_BITLEN_R {
        CACHE_SRAM_USR_WR_CMD_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - For SPI0 When cache mode is enable it is the write command value of command phase for SRAM."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_value(&self) -> CACHE_SRAM_USR_WR_CMD_VALUE_R {
        CACHE_SRAM_USR_WR_CMD_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31 - For SPI0 When cache mode is enable it is the in bits of command phase for SRAM. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_bitlen(&mut self) -> CACHE_SRAM_USR_WR_CMD_BITLEN_W {
        CACHE_SRAM_USR_WR_CMD_BITLEN_W { w: self }
    }
    #[doc = "Bits 0:15 - For SPI0 When cache mode is enable it is the write command value of command phase for SRAM."]
    #[inline(always)]
    pub fn cache_sram_usr_wr_cmd_value(&mut self) -> CACHE_SRAM_USR_WR_CMD_VALUE_W {
        CACHE_SRAM_USR_WR_CMD_VALUE_W { w: self }
    }
}
