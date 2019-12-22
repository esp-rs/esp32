#[doc = "Reader of register SLC_1RX_LINK_REG"]
pub type R = crate::R<u32, super::SLC_1RX_LINK_REG>;
#[doc = "Writer for register SLC_1RX_LINK_REG"]
pub type W = crate::W<u32, super::SLC_1RX_LINK_REG>;
#[doc = "Register SLC_1RX_LINK_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_1RX_LINK_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC1_RXLINK_PARK`"]
pub type SLC_SLC1_RXLINK_PARK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RXLINK_PARK`"]
pub struct SLC_SLC1_RXLINK_PARK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RXLINK_PARK_W<'a> {
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
#[doc = "Reader of field `SLC_SLC1_RXLINK_RESTART`"]
pub type SLC_SLC1_RXLINK_RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RXLINK_RESTART`"]
pub struct SLC_SLC1_RXLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RXLINK_RESTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RXLINK_START`"]
pub type SLC_SLC1_RXLINK_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RXLINK_START`"]
pub struct SLC_SLC1_RXLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RXLINK_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RXLINK_STOP`"]
pub type SLC_SLC1_RXLINK_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_RXLINK_STOP`"]
pub struct SLC_SLC1_RXLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RXLINK_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_BT_PACKET`"]
pub type SLC_SLC1_BT_PACKET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_BT_PACKET`"]
pub struct SLC_SLC1_BT_PACKET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_BT_PACKET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_RXLINK_ADDR`"]
pub type SLC_SLC1_RXLINK_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC_SLC1_RXLINK_ADDR`"]
pub struct SLC_SLC1_RXLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_RXLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_park(&self) -> SLC_SLC1_RXLINK_PARK_R {
        SLC_SLC1_RXLINK_PARK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_restart(&self) -> SLC_SLC1_RXLINK_RESTART_R {
        SLC_SLC1_RXLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_start(&self) -> SLC_SLC1_RXLINK_START_R {
        SLC_SLC1_RXLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_stop(&self) -> SLC_SLC1_RXLINK_STOP_R {
        SLC_SLC1_RXLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc1_bt_packet(&self) -> SLC_SLC1_BT_PACKET_R {
        SLC_SLC1_BT_PACKET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_addr(&self) -> SLC_SLC1_RXLINK_ADDR_R {
        SLC_SLC1_RXLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_park(&mut self) -> SLC_SLC1_RXLINK_PARK_W {
        SLC_SLC1_RXLINK_PARK_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_restart(&mut self) -> SLC_SLC1_RXLINK_RESTART_W {
        SLC_SLC1_RXLINK_RESTART_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_start(&mut self) -> SLC_SLC1_RXLINK_START_W {
        SLC_SLC1_RXLINK_START_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_stop(&mut self) -> SLC_SLC1_RXLINK_STOP_W {
        SLC_SLC1_RXLINK_STOP_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc_slc1_bt_packet(&mut self) -> SLC_SLC1_BT_PACKET_W {
        SLC_SLC1_BT_PACKET_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc_slc1_rxlink_addr(&mut self) -> SLC_SLC1_RXLINK_ADDR_W {
        SLC_SLC1_RXLINK_ADDR_W { w: self }
    }
}
