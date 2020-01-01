#[doc = "Reader of register W10"]
pub type R = crate::R<u32, super::W10>;
#[doc = "Writer for register W10"]
pub type W = crate::W<u32, super::W10>;
#[doc = "Register W10 `reset()`'s with value 0"]
impl crate::ResetValue for super::W10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_BUF10`"]
pub type SPI_BUF10_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_BUF10`"]
pub struct SPI_BUF10_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BUF10_W<'a> {
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
    pub fn spi_buf10(&self) -> SPI_BUF10_R {
        SPI_BUF10_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf10(&mut self) -> SPI_BUF10_W {
        SPI_BUF10_W { w: self }
    }
}
