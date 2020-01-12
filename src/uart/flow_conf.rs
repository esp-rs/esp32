#[doc = "Reader of register FLOW_CONF"]
pub type R = crate::R<u32, super::FLOW_CONF>;
#[doc = "Writer for register FLOW_CONF"]
pub type W = crate::W<u32, super::FLOW_CONF>;
#[doc = "Register FLOW_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::FLOW_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEND_XOFF`"]
pub type SEND_XOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_XOFF`"]
pub struct SEND_XOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_XOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SEND_XON`"]
pub type SEND_XON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEND_XON`"]
pub struct SEND_XON_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_XON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FORCE_XOFF`"]
pub type FORCE_XOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_XOFF`"]
pub struct FORCE_XOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FORCE_XON`"]
pub type FORCE_XON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_XON`"]
pub struct FORCE_XON_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `XONOFF_DEL`"]
pub type XONOFF_DEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XONOFF_DEL`"]
pub struct XONOFF_DEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XONOFF_DEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SW_FLOW_CON_EN`"]
pub type SW_FLOW_CON_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_FLOW_CON_EN`"]
pub struct SW_FLOW_CON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_FLOW_CON_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Set this bit to send xoff char. it is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xoff(&self) -> SEND_XOFF_R {
        SEND_XOFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to send xon char. it is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xon(&self) -> SEND_XON_R {
        SEND_XON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to set ctsn to enable the transmitter to go on sending data."]
    #[inline(always)]
    pub fn force_xoff(&self) -> FORCE_XOFF_R {
        FORCE_XOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to clear ctsn to stop the transmitter from sending data."]
    #[inline(always)]
    pub fn force_xon(&self) -> FORCE_XON_R {
        FORCE_XON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    pub fn xonoff_del(&self) -> XONOFF_DEL_R {
        XONOFF_DEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to enable software flow control. it is used with register sw_xon or sw_xoff ."]
    #[inline(always)]
    pub fn sw_flow_con_en(&self) -> SW_FLOW_CON_EN_R {
        SW_FLOW_CON_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Set this bit to send xoff char. it is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xoff(&mut self) -> SEND_XOFF_W {
        SEND_XOFF_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to send xon char. it is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xon(&mut self) -> SEND_XON_W {
        SEND_XON_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to set ctsn to enable the transmitter to go on sending data."]
    #[inline(always)]
    pub fn force_xoff(&mut self) -> FORCE_XOFF_W {
        FORCE_XOFF_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear ctsn to stop the transmitter from sending data."]
    #[inline(always)]
    pub fn force_xon(&mut self) -> FORCE_XON_W {
        FORCE_XON_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    pub fn xonoff_del(&mut self) -> XONOFF_DEL_W {
        XONOFF_DEL_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to enable software flow control. it is used with register sw_xon or sw_xoff ."]
    #[inline(always)]
    pub fn sw_flow_con_en(&mut self) -> SW_FLOW_CON_EN_W {
        SW_FLOW_CON_EN_W { w: self }
    }
}
