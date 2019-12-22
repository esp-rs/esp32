#[doc = "Reader of register UART_SWFC_CONF_REG"]
pub type R = crate::R<u32, super::UART_SWFC_CONF_REG>;
#[doc = "Writer for register UART_SWFC_CONF_REG"]
pub type W = crate::W<u32, super::UART_SWFC_CONF_REG>;
#[doc = "Register UART_SWFC_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_SWFC_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_XOFF_CHAR`"]
pub type UART_XOFF_CHAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_XOFF_CHAR`"]
pub struct UART_XOFF_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_XOFF_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `UART_XON_CHAR`"]
pub type UART_XON_CHAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_XON_CHAR`"]
pub struct UART_XON_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_XON_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `UART_XOFF_THRESHOLD`"]
pub type UART_XOFF_THRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_XOFF_THRESHOLD`"]
pub struct UART_XOFF_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_XOFF_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `UART_XON_THRESHOLD`"]
pub type UART_XON_THRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_XON_THRESHOLD`"]
pub struct UART_XON_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_XON_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - This register stores the xoff flow control char."]
    #[inline(always)]
    pub fn uart_xoff_char(&self) -> UART_XOFF_CHAR_R {
        UART_XOFF_CHAR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    pub fn uart_xon_char(&self) -> UART_XON_CHAR_R {
        UART_XON_CHAR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn uart_xoff_threshold(&self) -> UART_XOFF_THRESHOLD_R {
        UART_XOFF_THRESHOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn uart_xon_threshold(&self) -> UART_XON_THRESHOLD_R {
        UART_XON_THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - This register stores the xoff flow control char."]
    #[inline(always)]
    pub fn uart_xoff_char(&mut self) -> UART_XOFF_CHAR_W {
        UART_XOFF_CHAR_W { w: self }
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    pub fn uart_xon_char(&mut self) -> UART_XON_CHAR_W {
        UART_XON_CHAR_W { w: self }
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn uart_xoff_threshold(&mut self) -> UART_XOFF_THRESHOLD_W {
        UART_XOFF_THRESHOLD_W { w: self }
    }
    #[doc = "Bits 0:7 - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn uart_xon_threshold(&mut self) -> UART_XON_THRESHOLD_W {
        UART_XON_THRESHOLD_W { w: self }
    }
}
