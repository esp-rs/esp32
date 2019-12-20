#[doc = "Reader of register EFUSE_BLK0_WDATA1_REG"]
pub type R = crate::R<u32, super::EFUSE_BLK0_WDATA1_REG>;
#[doc = "Writer for register EFUSE_BLK0_WDATA1_REG"]
pub type W = crate::W<u32, super::EFUSE_BLK0_WDATA1_REG>;
#[doc = "Register EFUSE_BLK0_WDATA1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_BLK0_WDATA1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_WIFI_MAC_CRC_LOW`"]
pub type EFUSE_WIFI_MAC_CRC_LOW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_WIFI_MAC_CRC_LOW`"]
pub struct EFUSE_WIFI_MAC_CRC_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_WIFI_MAC_CRC_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - program for low 32bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn efuse_wifi_mac_crc_low(&self) -> EFUSE_WIFI_MAC_CRC_LOW_R {
        EFUSE_WIFI_MAC_CRC_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for low 32bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn efuse_wifi_mac_crc_low(&mut self) -> EFUSE_WIFI_MAC_CRC_LOW_W {
        EFUSE_WIFI_MAC_CRC_LOW_W { w: self }
    }
}
