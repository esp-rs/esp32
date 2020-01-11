#[doc = "Reader of register AT_CMD_CHAR"]
pub type R = crate::R<u32, super::AT_CMD_CHAR>;
#[doc = "Writer for register AT_CMD_CHAR"]
pub type W = crate::W<u32, super::AT_CMD_CHAR>;
#[doc = "Register AT_CMD_CHAR `reset()`'s with value 0"]
impl crate::ResetValue for super::AT_CMD_CHAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_CHAR_NUM`"]
pub type UART_CHAR_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_CHAR_NUM`"]
pub struct UART_CHAR_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CHAR_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `UART_AT_CMD_CHAR`"]
pub type UART_AT_CMD_CHAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART_AT_CMD_CHAR`"]
pub struct UART_AT_CMD_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_AT_CMD_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - This register is used to configure the num of continous at_cmd chars received by receiver."]
    #[inline(always)]
    pub fn uart_char_num(&self) -> UART_CHAR_NUM_R {
        UART_CHAR_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - This register is used to configure the content of at_cmd char."]
    #[inline(always)]
    pub fn uart_at_cmd_char(&self) -> UART_AT_CMD_CHAR_R {
        UART_AT_CMD_CHAR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - This register is used to configure the num of continous at_cmd chars received by receiver."]
    #[inline(always)]
    pub fn uart_char_num(&mut self) -> UART_CHAR_NUM_W {
        UART_CHAR_NUM_W { w: self }
    }
    #[doc = "Bits 0:7 - This register is used to configure the content of at_cmd char."]
    #[inline(always)]
    pub fn uart_at_cmd_char(&mut self) -> UART_AT_CMD_CHAR_W {
        UART_AT_CMD_CHAR_W { w: self }
    }
}
