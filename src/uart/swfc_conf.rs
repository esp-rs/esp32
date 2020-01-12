#[doc = "Reader of register SWFC_CONF"]
pub type R = crate::R<u32, super::SWFC_CONF>;
#[doc = "Writer for register SWFC_CONF"]
pub type W = crate::W<u32, super::SWFC_CONF>;
#[doc = "Register SWFC_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SWFC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOFF_CHAR`"]
pub type XOFF_CHAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOFF_CHAR`"]
pub struct XOFF_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `XON_CHAR`"]
pub type XON_CHAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XON_CHAR`"]
pub struct XON_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> XON_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `XOFF_THRESHOLD`"]
pub type XOFF_THRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOFF_THRESHOLD`"]
pub struct XOFF_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `XON_THRESHOLD`"]
pub type XON_THRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XON_THRESHOLD`"]
pub struct XON_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> XON_THRESHOLD_W<'a> {
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
    pub fn xoff_char(&self) -> XOFF_CHAR_R {
        XOFF_CHAR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&self) -> XON_CHAR_R {
        XON_CHAR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xoff_threshold(&self) -> XOFF_THRESHOLD_R {
        XOFF_THRESHOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xon_threshold(&self) -> XON_THRESHOLD_R {
        XON_THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - This register stores the xoff flow control char."]
    #[inline(always)]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W {
        XOFF_CHAR_W { w: self }
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&mut self) -> XON_CHAR_W {
        XON_CHAR_W { w: self }
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W {
        XOFF_THRESHOLD_W { w: self }
    }
    #[doc = "Bits 0:7 - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xon_threshold(&mut self) -> XON_THRESHOLD_W {
        XON_THRESHOLD_W { w: self }
    }
}
