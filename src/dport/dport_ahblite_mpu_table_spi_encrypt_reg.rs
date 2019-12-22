#[doc = "Reader of register DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG"]
pub type R = crate::R<u32, super::DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG>;
#[doc = "Writer for register DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG"]
pub type W = crate::W<u32, super::DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG>;
#[doc = "Register DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_AHBLITE_MPU_TABLE_SPI_ENCRYPT_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_SPI_ENCRYPY_ACCESS_GRANT_CONFIG`"]
pub type DPORT_SPI_ENCRYPY_ACCESS_GRANT_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_SPI_ENCRYPY_ACCESS_GRANT_CONFIG`"]
pub struct DPORT_SPI_ENCRYPY_ACCESS_GRANT_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SPI_ENCRYPY_ACCESS_GRANT_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dport_spi_encrypy_access_grant_config(&self) -> DPORT_SPI_ENCRYPY_ACCESS_GRANT_CONFIG_R {
        DPORT_SPI_ENCRYPY_ACCESS_GRANT_CONFIG_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dport_spi_encrypy_access_grant_config(
        &mut self,
    ) -> DPORT_SPI_ENCRYPY_ACCESS_GRANT_CONFIG_W {
        DPORT_SPI_ENCRYPY_ACCESS_GRANT_CONFIG_W { w: self }
    }
}
