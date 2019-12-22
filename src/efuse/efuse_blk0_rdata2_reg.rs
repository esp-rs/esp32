#[doc = "Reader of register EFUSE_BLK0_RDATA2_REG"]
pub type R = crate::R<u32, super::EFUSE_BLK0_RDATA2_REG>;
#[doc = "Writer for register EFUSE_BLK0_RDATA2_REG"]
pub type W = crate::W<u32, super::EFUSE_BLK0_RDATA2_REG>;
#[doc = "Register EFUSE_BLK0_RDATA2_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_BLK0_RDATA2_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_RD_WIFI_MAC_CRC_HIGH`"]
pub type EFUSE_RD_WIFI_MAC_CRC_HIGH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_RD_WIFI_MAC_CRC_HIGH`"]
pub struct EFUSE_RD_WIFI_MAC_CRC_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_WIFI_MAC_CRC_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - read for high 24bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn efuse_rd_wifi_mac_crc_high(&self) -> EFUSE_RD_WIFI_MAC_CRC_HIGH_R {
        EFUSE_RD_WIFI_MAC_CRC_HIGH_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - read for high 24bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn efuse_rd_wifi_mac_crc_high(&mut self) -> EFUSE_RD_WIFI_MAC_CRC_HIGH_W {
        EFUSE_RD_WIFI_MAC_CRC_HIGH_W { w: self }
    }
}
