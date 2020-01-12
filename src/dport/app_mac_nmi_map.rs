#[doc = "Reader of register APP_MAC_NMI_MAP"]
pub type R = crate::R<u32, super::APP_MAC_NMI_MAP>;
#[doc = "Writer for register APP_MAC_NMI_MAP"]
pub type W = crate::W<u32, super::APP_MAC_NMI_MAP>;
#[doc = "Register APP_MAC_NMI_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_MAC_NMI_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_MAC_NMI_MAP`"]
pub type APP_MAC_NMI_MAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APP_MAC_NMI_MAP`"]
pub struct APP_MAC_NMI_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_MAC_NMI_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_mac_nmi_map(&self) -> APP_MAC_NMI_MAP_R {
        APP_MAC_NMI_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_mac_nmi_map(&mut self) -> APP_MAC_NMI_MAP_W {
        APP_MAC_NMI_MAP_W { w: self }
    }
}
