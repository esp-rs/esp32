#[doc = "Reader of register MISO_DLEN"]
pub type R = crate::R<u32, super::MISO_DLEN>;
#[doc = "Writer for register MISO_DLEN"]
pub type W = crate::W<u32, super::MISO_DLEN>;
#[doc = "Register MISO_DLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::MISO_DLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_USR_MISO_DBITLEN`"]
pub type SPI_USR_MISO_DBITLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_USR_MISO_DBITLEN`"]
pub struct SPI_USR_MISO_DBITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_MISO_DBITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - The length in bits of read-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_usr_miso_dbitlen(&self) -> SPI_USR_MISO_DBITLEN_R {
        SPI_USR_MISO_DBITLEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - The length in bits of read-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_usr_miso_dbitlen(&mut self) -> SPI_USR_MISO_DBITLEN_W {
        SPI_USR_MISO_DBITLEN_W { w: self }
    }
}
