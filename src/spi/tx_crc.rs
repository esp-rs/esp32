#[doc = "Reader of register TX_CRC"]
pub type R = crate::R<u32, super::TX_CRC>;
#[doc = "Writer for register TX_CRC"]
pub type W = crate::W<u32, super::TX_CRC>;
#[doc = "Register TX_CRC `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_CRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_CRC_DATA`"]
pub type TX_CRC_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TX_CRC_DATA`"]
pub struct TX_CRC_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CRC_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - For SPI1 the value of crc32 for 256 bits data."]
    #[inline(always)]
    pub fn tx_crc_data(&self) -> TX_CRC_DATA_R {
        TX_CRC_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - For SPI1 the value of crc32 for 256 bits data."]
    #[inline(always)]
    pub fn tx_crc_data(&mut self) -> TX_CRC_DATA_W {
        TX_CRC_DATA_W { w: self }
    }
}
