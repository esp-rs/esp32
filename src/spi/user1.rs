#[doc = "Reader of register USER1"]
pub type R = crate::R<u32, super::USER1>;
#[doc = "Writer for register USER1"]
pub type W = crate::W<u32, super::USER1>;
#[doc = "Register USER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::USER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USR_ADDR_BITLEN`"]
pub type USR_ADDR_BITLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USR_ADDR_BITLEN`"]
pub struct USR_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `USR_DUMMY_CYCLELEN`"]
pub type USR_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USR_DUMMY_CYCLELEN`"]
pub struct USR_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> USR_ADDR_BITLEN_R {
        USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> USR_DUMMY_CYCLELEN_R {
        USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&mut self) -> USR_ADDR_BITLEN_W {
        USR_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&mut self) -> USR_DUMMY_CYCLELEN_W {
        USR_DUMMY_CYCLELEN_W { w: self }
    }
}
