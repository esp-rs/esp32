#[doc = "Reader of register SLV_RD_BIT"]
pub type R = crate::R<u32, super::SLV_RD_BIT>;
#[doc = "Writer for register SLV_RD_BIT"]
pub type W = crate::W<u32, super::SLV_RD_BIT>;
#[doc = "Register SLV_RD_BIT `reset()`'s with value 0"]
impl crate::ResetValue for super::SLV_RD_BIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_SLV_RDATA_BIT`"]
pub type SPI_SLV_RDATA_BIT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_SLV_RDATA_BIT`"]
pub struct SPI_SLV_RDATA_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RDATA_BIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - In the slave mode it is the bit length of read data. The value is the length - 1."]
    #[inline(always)]
    pub fn spi_slv_rdata_bit(&self) -> SPI_SLV_RDATA_BIT_R {
        SPI_SLV_RDATA_BIT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - In the slave mode it is the bit length of read data. The value is the length - 1."]
    #[inline(always)]
    pub fn spi_slv_rdata_bit(&mut self) -> SPI_SLV_RDATA_BIT_W {
        SPI_SLV_RDATA_BIT_W { w: self }
    }
}
