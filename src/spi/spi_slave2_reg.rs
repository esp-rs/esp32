#[doc = "Reader of register SPI_SLAVE2_REG"]
pub type R = crate::R<u32, super::SPI_SLAVE2_REG>;
#[doc = "Writer for register SPI_SLAVE2_REG"]
pub type W = crate::W<u32, super::SPI_SLAVE2_REG>;
#[doc = "Register SPI_SLAVE2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_SLAVE2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_SLV_WRBUF_DUMMY_CYCLELEN`"]
pub type SPI_SLV_WRBUF_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_WRBUF_DUMMY_CYCLELEN`"]
pub struct SPI_SLV_WRBUF_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WRBUF_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_RDBUF_DUMMY_CYCLELEN`"]
pub type SPI_SLV_RDBUF_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_RDBUF_DUMMY_CYCLELEN`"]
pub struct SPI_SLV_RDBUF_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RDBUF_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_WRSTA_DUMMY_CYCLELEN`"]
pub type SPI_SLV_WRSTA_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_WRSTA_DUMMY_CYCLELEN`"]
pub struct SPI_SLV_WRSTA_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_WRSTA_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLV_RDSTA_DUMMY_CYCLELEN`"]
pub type SPI_SLV_RDSTA_DUMMY_CYCLELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SLV_RDSTA_DUMMY_CYCLELEN`"]
pub struct SPI_SLV_RDSTA_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLV_RDSTA_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_slv_wrbuf_dummy_cyclelen(&self) -> SPI_SLV_WRBUF_DUMMY_CYCLELEN_R {
        SPI_SLV_WRBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_slv_rdbuf_dummy_cyclelen(&self) -> SPI_SLV_RDBUF_DUMMY_CYCLELEN_R {
        SPI_SLV_RDBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_slv_wrsta_dummy_cyclelen(&self) -> SPI_SLV_WRSTA_DUMMY_CYCLELEN_R {
        SPI_SLV_WRSTA_DUMMY_CYCLELEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_slv_rdsta_dummy_cyclelen(&self) -> SPI_SLV_RDSTA_DUMMY_CYCLELEN_R {
        SPI_SLV_RDSTA_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_slv_wrbuf_dummy_cyclelen(&mut self) -> SPI_SLV_WRBUF_DUMMY_CYCLELEN_W {
        SPI_SLV_WRBUF_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 16:23 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_slv_rdbuf_dummy_cyclelen(&mut self) -> SPI_SLV_RDBUF_DUMMY_CYCLELEN_W {
        SPI_SLV_RDBUF_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 8:15 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_slv_wrsta_dummy_cyclelen(&mut self) -> SPI_SLV_WRSTA_DUMMY_CYCLELEN_W {
        SPI_SLV_WRSTA_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Bits 0:7 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_slv_rdsta_dummy_cyclelen(&mut self) -> SPI_SLV_RDSTA_DUMMY_CYCLELEN_W {
        SPI_SLV_RDSTA_DUMMY_CYCLELEN_W { w: self }
    }
}
