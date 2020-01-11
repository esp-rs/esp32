#[doc = "Reader of register W13"]
pub type R = crate::R<u32, super::W13>;
#[doc = "Writer for register W13"]
pub type W = crate::W<u32, super::W13>;
#[doc = "Register W13 `reset()`'s with value 0"]
impl crate::ResetValue for super::W13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_BUF13`"]
pub type SPI_BUF13_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_BUF13`"]
pub struct SPI_BUF13_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BUF13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf13(&self) -> SPI_BUF13_R {
        SPI_BUF13_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf13(&mut self) -> SPI_BUF13_W {
        SPI_BUF13_W { w: self }
    }
}
