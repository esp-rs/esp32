#[doc = "Reader of register BLK0_WDATA2"]
pub type R = crate::R<u32, super::BLK0_WDATA2>;
#[doc = "Writer for register BLK0_WDATA2"]
pub type W = crate::W<u32, super::BLK0_WDATA2>;
#[doc = "Register BLK0_WDATA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK0_WDATA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_WIFI_MAC_CRC_HIGH`"]
pub type EFUSE_WIFI_MAC_CRC_HIGH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFUSE_WIFI_MAC_CRC_HIGH`"]
pub struct EFUSE_WIFI_MAC_CRC_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_WIFI_MAC_CRC_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - program for high 24bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn efuse_wifi_mac_crc_high(&self) -> EFUSE_WIFI_MAC_CRC_HIGH_R {
        EFUSE_WIFI_MAC_CRC_HIGH_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - program for high 24bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn efuse_wifi_mac_crc_high(&mut self) -> EFUSE_WIFI_MAC_CRC_HIGH_W {
        EFUSE_WIFI_MAC_CRC_HIGH_W { w: self }
    }
}
