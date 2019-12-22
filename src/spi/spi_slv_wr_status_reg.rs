#[doc = "Reader of register SPI_SLV_WR_STATUS_REG"]
pub type R = crate::R<u32, super::SPI_SLV_WR_STATUS_REG>;
#[doc = "Writer for register SPI_SLV_WR_STATUS_REG"]
pub type W = crate::W<u32, super::SPI_SLV_WR_STATUS_REG>;
#[doc = "Register SPI_SLV_WR_STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_SLV_WR_STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_SLV_WR_ST`"]
pub type SPI_SLV_WR_ST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_SLV_WR_ST`"]
pub struct SPI_SLV_WR_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WR_ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - In the slave mode this register are the status register for the master to write into. In the master mode this register are the higher 32bits in the 64 bits address condition."]
    #[inline(always)]
    pub fn spi_slv_wr_st(&self) -> SPI_SLV_WR_ST_R {
        SPI_SLV_WR_ST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - In the slave mode this register are the status register for the master to write into. In the master mode this register are the higher 32bits in the 64 bits address condition."]
    #[inline(always)]
    pub fn spi_slv_wr_st(&mut self) -> SPI_SLV_WR_ST_W {
        SPI_SLV_WR_ST_W { w: self }
    }
}
