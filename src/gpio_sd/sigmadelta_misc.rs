#[doc = "Reader of register SIGMADELTA_MISC"]
pub type R = crate::R<u32, super::SIGMADELTA_MISC>;
#[doc = "Writer for register SIGMADELTA_MISC"]
pub type W = crate::W<u32, super::SIGMADELTA_MISC>;
#[doc = "Register SIGMADELTA_MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::SIGMADELTA_MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_SWAP`"]
pub type SPI_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SWAP`"]
pub struct SPI_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SWAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_swap(&self) -> SPI_SWAP_R {
        SPI_SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_swap(&mut self) -> SPI_SWAP_W {
        SPI_SWAP_W { w: self }
    }
}
