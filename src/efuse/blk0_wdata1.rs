#[doc = "Reader of register BLK0_WDATA1"]
pub type R = crate::R<u32, super::BLK0_WDATA1>;
#[doc = "Writer for register BLK0_WDATA1"]
pub type W = crate::W<u32, super::BLK0_WDATA1>;
#[doc = "Register BLK0_WDATA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK0_WDATA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WIFI_MAC_CRC_LOW`"]
pub type WIFI_MAC_CRC_LOW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WIFI_MAC_CRC_LOW`"]
pub struct WIFI_MAC_CRC_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MAC_CRC_LOW_W<'a> {
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
    pub fn wifi_mac_crc_low(&self) -> WIFI_MAC_CRC_LOW_R {
        WIFI_MAC_CRC_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - program for low 32bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn wifi_mac_crc_low(&mut self) -> WIFI_MAC_CRC_LOW_W {
        WIFI_MAC_CRC_LOW_W { w: self }
    }
}