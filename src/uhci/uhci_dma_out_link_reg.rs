#[doc = "Reader of register UHCI_DMA_OUT_LINK_REG"]
pub type R = crate::R<u32, super::UHCI_DMA_OUT_LINK_REG>;
#[doc = "Writer for register UHCI_DMA_OUT_LINK_REG"]
pub type W = crate::W<u32, super::UHCI_DMA_OUT_LINK_REG>;
#[doc = "Register UHCI_DMA_OUT_LINK_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_DMA_OUT_LINK_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_OUTLINK_PARK`"]
pub type UHCI_OUTLINK_PARK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUTLINK_PARK`"]
pub struct UHCI_OUTLINK_PARK_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUTLINK_PARK_W<'a> {
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
#[doc = "Reader of field `UHCI_OUTLINK_RESTART`"]
pub type UHCI_OUTLINK_RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUTLINK_RESTART`"]
pub struct UHCI_OUTLINK_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUTLINK_RESTART_W<'a> {
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
#[doc = "Reader of field `UHCI_OUTLINK_START`"]
pub type UHCI_OUTLINK_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUTLINK_START`"]
pub struct UHCI_OUTLINK_START_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUTLINK_START_W<'a> {
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
#[doc = "Reader of field `UHCI_OUTLINK_STOP`"]
pub type UHCI_OUTLINK_STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_OUTLINK_STOP`"]
pub struct UHCI_OUTLINK_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUTLINK_STOP_W<'a> {
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
#[doc = "Reader of field `UHCI_OUTLINK_ADDR`"]
pub type UHCI_OUTLINK_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UHCI_OUTLINK_ADDR`"]
pub struct UHCI_OUTLINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_OUTLINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 1\u{a3}\u{ba} the out link descriptor's fsm is in idle state. 0:the out link descriptor's fsm is working."]
    #[inline(always)]
    pub fn uhci_outlink_park(&self) -> UHCI_OUTLINK_PARK_R {
        UHCI_OUTLINK_PARK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Set this bit to mount on new out link descriptors"]
    #[inline(always)]
    pub fn uhci_outlink_restart(&self) -> UHCI_OUTLINK_RESTART_R {
        UHCI_OUTLINK_RESTART_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Set this bit to start dealing with the out link descriptors."]
    #[inline(always)]
    pub fn uhci_outlink_start(&self) -> UHCI_OUTLINK_START_R {
        UHCI_OUTLINK_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set this bit to stop dealing with the out link descriptors."]
    #[inline(always)]
    pub fn uhci_outlink_stop(&self) -> UHCI_OUTLINK_STOP_R {
        UHCI_OUTLINK_STOP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 0:19 - This register stores the least 20 bits of the first out link descriptor's address."]
    #[inline(always)]
    pub fn uhci_outlink_addr(&self) -> UHCI_OUTLINK_ADDR_R {
        UHCI_OUTLINK_ADDR_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - 1\u{a3}\u{ba} the out link descriptor's fsm is in idle state. 0:the out link descriptor's fsm is working."]
    #[inline(always)]
    pub fn uhci_outlink_park(&mut self) -> UHCI_OUTLINK_PARK_W {
        UHCI_OUTLINK_PARK_W { w: self }
    }
    #[doc = "Bit 30 - Set this bit to mount on new out link descriptors"]
    #[inline(always)]
    pub fn uhci_outlink_restart(&mut self) -> UHCI_OUTLINK_RESTART_W {
        UHCI_OUTLINK_RESTART_W { w: self }
    }
    #[doc = "Bit 29 - Set this bit to start dealing with the out link descriptors."]
    #[inline(always)]
    pub fn uhci_outlink_start(&mut self) -> UHCI_OUTLINK_START_W {
        UHCI_OUTLINK_START_W { w: self }
    }
    #[doc = "Bit 28 - Set this bit to stop dealing with the out link descriptors."]
    #[inline(always)]
    pub fn uhci_outlink_stop(&mut self) -> UHCI_OUTLINK_STOP_W {
        UHCI_OUTLINK_STOP_W { w: self }
    }
    #[doc = "Bits 0:19 - This register stores the least 20 bits of the first out link descriptor's address."]
    #[inline(always)]
    pub fn uhci_outlink_addr(&mut self) -> UHCI_OUTLINK_ADDR_W {
        UHCI_OUTLINK_ADDR_W { w: self }
    }
}
