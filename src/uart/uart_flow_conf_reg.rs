#[doc = "Reader of register UART_FLOW_CONF_REG"]
pub type R = crate::R<u32, super::UART_FLOW_CONF_REG>;
#[doc = "Writer for register UART_FLOW_CONF_REG"]
pub type W = crate::W<u32, super::UART_FLOW_CONF_REG>;
#[doc = "Register UART_FLOW_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_FLOW_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_SEND_XOFF`"]
pub type UART_SEND_XOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SEND_XOFF`"]
pub struct UART_SEND_XOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SEND_XOFF_W<'a> {
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
#[doc = "Reader of field `UART_SEND_XON`"]
pub type UART_SEND_XON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SEND_XON`"]
pub struct UART_SEND_XON_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SEND_XON_W<'a> {
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
#[doc = "Reader of field `UART_FORCE_XOFF`"]
pub type UART_FORCE_XOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_FORCE_XOFF`"]
pub struct UART_FORCE_XOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FORCE_XOFF_W<'a> {
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
#[doc = "Reader of field `UART_FORCE_XON`"]
pub type UART_FORCE_XON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_FORCE_XON`"]
pub struct UART_FORCE_XON_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FORCE_XON_W<'a> {
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
#[doc = "Reader of field `UART_XONOFF_DEL`"]
pub type UART_XONOFF_DEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_XONOFF_DEL`"]
pub struct UART_XONOFF_DEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_XONOFF_DEL_W<'a> {
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
#[doc = "Reader of field `UART_SW_FLOW_CON_EN`"]
pub type UART_SW_FLOW_CON_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART_SW_FLOW_CON_EN`"]
pub struct UART_SW_FLOW_CON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SW_FLOW_CON_EN_W<'a> {
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
    pub fn uart_send_xoff(&self) -> UART_SEND_XOFF_R {
        UART_SEND_XOFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to send xon char. it is cleared by hardware automatically."]
    #[inline(always)]
    pub fn uart_send_xon(&self) -> UART_SEND_XON_R {
        UART_SEND_XON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to set ctsn to enable the transmitter to go on sending data."]
    #[inline(always)]
    pub fn uart_force_xoff(&self) -> UART_FORCE_XOFF_R {
        UART_FORCE_XOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to clear ctsn to stop the transmitter from sending data."]
    #[inline(always)]
    pub fn uart_force_xon(&self) -> UART_FORCE_XON_R {
        UART_FORCE_XON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    pub fn uart_xonoff_del(&self) -> UART_XONOFF_DEL_R {
        UART_XONOFF_DEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to enable software flow control. it is used with register sw_xon or sw_xoff ."]
    #[inline(always)]
    pub fn uart_sw_flow_con_en(&self) -> UART_SW_FLOW_CON_EN_R {
        UART_SW_FLOW_CON_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Set this bit to send xoff char. it is cleared by hardware automatically."]
    #[inline(always)]
    pub fn uart_send_xoff(&mut self) -> UART_SEND_XOFF_W {
        UART_SEND_XOFF_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to send xon char. it is cleared by hardware automatically."]
    #[inline(always)]
    pub fn uart_send_xon(&mut self) -> UART_SEND_XON_W {
        UART_SEND_XON_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to set ctsn to enable the transmitter to go on sending data."]
    #[inline(always)]
    pub fn uart_force_xoff(&mut self) -> UART_FORCE_XOFF_W {
        UART_FORCE_XOFF_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear ctsn to stop the transmitter from sending data."]
    #[inline(always)]
    pub fn uart_force_xon(&mut self) -> UART_FORCE_XON_W {
        UART_FORCE_XON_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    pub fn uart_xonoff_del(&mut self) -> UART_XONOFF_DEL_W {
        UART_XONOFF_DEL_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to enable software flow control. it is used with register sw_xon or sw_xoff ."]
    #[inline(always)]
    pub fn uart_sw_flow_con_en(&mut self) -> UART_SW_FLOW_CON_EN_W {
        UART_SW_FLOW_CON_EN_W { w: self }
    }
}
