#[doc = "Reader of register UHCI_DMA_IN_LINK_REG"]
pub type R = crate::R<u32, super::UHCI_DMA_IN_LINK_REG>;
#[doc = "Writer for register UHCI_DMA_IN_LINK_REG"]
pub type W = crate::W<u32, super::UHCI_DMA_IN_LINK_REG>;
#[doc = "Register UHCI_DMA_IN_LINK_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_DMA_IN_LINK_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_INLINK_PARK`"]
pub type UHCI_INLINK_PARK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_INLINK_PARK`"]
pub struct UHCI_INLINK_PARK_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_INLINK_PARK_W<'a> {
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
#[doc = "Reader of field `UHCI_INLINK_RESTART`"]
pub type UHCI_INLINK_RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_INLINK_RESTART`"]
pub struct UHCI_INLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_INLINK_RESTART_W<'a> {
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
#[doc = "Reader of field `UHCI_INLINK_START`"]
pub type UHCI_INLINK_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_INLINK_START`"]
pub struct UHCI_INLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_INLINK_START_W<'a> {
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
#[doc = "Reader of field `UHCI_INLINK_STOP`"]
pub type UHCI_INLINK_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_INLINK_STOP`"]
pub struct UHCI_INLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_INLINK_STOP_W<'a> {
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
#[doc = "Reader of field `UHCI_INLINK_AUTO_RET`"]
pub type UHCI_INLINK_AUTO_RET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_INLINK_AUTO_RET`"]
pub struct UHCI_INLINK_AUTO_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_INLINK_AUTO_RET_W<'a> {
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
#[doc = "Reader of field `UHCI_INLINK_ADDR`"]
pub type UHCI_INLINK_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UHCI_INLINK_ADDR`"]
pub struct UHCI_INLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_INLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 1:the in link descriptor's fsm is in idle state. 0:the in link descriptor's fsm is working"]
    #[inline(always)]
    pub fn uhci_inlink_park(&self) -> UHCI_INLINK_PARK_R {
        UHCI_INLINK_PARK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Set this bit to mount on new in link descriptors"]
    #[inline(always)]
    pub fn uhci_inlink_restart(&self) -> UHCI_INLINK_RESTART_R {
        UHCI_INLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Set this bit to start dealing with the in link descriptors."]
    #[inline(always)]
    pub fn uhci_inlink_start(&self) -> UHCI_INLINK_START_R {
        UHCI_INLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set this bit to stop dealing with the in link descriptors."]
    #[inline(always)]
    pub fn uhci_inlink_stop(&self) -> UHCI_INLINK_STOP_R {
        UHCI_INLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 1:when a packet is wrong in link descriptor returns to the descriptor which is lately used."]
    #[inline(always)]
    pub fn uhci_inlink_auto_ret(&self) -> UHCI_INLINK_AUTO_RET_R {
        UHCI_INLINK_AUTO_RET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:19 - This register stores the least 20 bits of the first in link descriptor's address."]
    #[inline(always)]
    pub fn uhci_inlink_addr(&self) -> UHCI_INLINK_ADDR_R {
        UHCI_INLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - 1:the in link descriptor's fsm is in idle state. 0:the in link descriptor's fsm is working"]
    #[inline(always)]
    pub fn uhci_inlink_park(&mut self) -> UHCI_INLINK_PARK_W {
        UHCI_INLINK_PARK_W { w: self }
    }
    #[doc = "Bit 30 - Set this bit to mount on new in link descriptors"]
    #[inline(always)]
    pub fn uhci_inlink_restart(&mut self) -> UHCI_INLINK_RESTART_W {
        UHCI_INLINK_RESTART_W { w: self }
    }
    #[doc = "Bit 29 - Set this bit to start dealing with the in link descriptors."]
    #[inline(always)]
    pub fn uhci_inlink_start(&mut self) -> UHCI_INLINK_START_W {
        UHCI_INLINK_START_W { w: self }
    }
    #[doc = "Bit 28 - Set this bit to stop dealing with the in link descriptors."]
    #[inline(always)]
    pub fn uhci_inlink_stop(&mut self) -> UHCI_INLINK_STOP_W {
        UHCI_INLINK_STOP_W { w: self }
    }
    #[doc = "Bit 20 - 1:when a packet is wrong in link descriptor returns to the descriptor which is lately used."]
    #[inline(always)]
    pub fn uhci_inlink_auto_ret(&mut self) -> UHCI_INLINK_AUTO_RET_W {
        UHCI_INLINK_AUTO_RET_W { w: self }
    }
    #[doc = "Bits 0:19 - This register stores the least 20 bits of the first in link descriptor's address."]
    #[inline(always)]
    pub fn uhci_inlink_addr(&mut self) -> UHCI_INLINK_ADDR_W {
        UHCI_INLINK_ADDR_W { w: self }
    }
}
