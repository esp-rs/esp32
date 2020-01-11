#[doc = "Reader of register ESC_CONF0"]
pub type R = crate::R<u32, super::ESC_CONF0>;
#[doc = "Writer for register ESC_CONF0"]
pub type W = crate::W<u32, super::ESC_CONF0>;
#[doc = "Register ESC_CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ESC_CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_SEPER_ESC_CHAR1`"]
pub type UHCI_SEPER_ESC_CHAR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_SEPER_ESC_CHAR1`"]
pub struct UHCI_SEPER_ESC_CHAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEPER_ESC_CHAR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `UHCI_SEPER_ESC_CHAR0`"]
pub type UHCI_SEPER_ESC_CHAR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_SEPER_ESC_CHAR0`"]
pub struct UHCI_SEPER_ESC_CHAR0_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEPER_ESC_CHAR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `UHCI_SEPER_CHAR`"]
pub type UHCI_SEPER_CHAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UHCI_SEPER_CHAR`"]
pub struct UHCI_SEPER_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEPER_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
    #[inline(always)]
    pub fn uhci_seper_esc_char1(&self) -> UHCI_SEPER_ESC_CHAR1_R {
        UHCI_SEPER_ESC_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register stores thee first char used to replace seperator char in data."]
    #[inline(always)]
    pub fn uhci_seper_esc_char0(&self) -> UHCI_SEPER_ESC_CHAR0_R {
        UHCI_SEPER_ESC_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - This register stores the seperator char seperator char is used to seperate the data frame."]
    #[inline(always)]
    pub fn uhci_seper_char(&self) -> UHCI_SEPER_CHAR_R {
        UHCI_SEPER_CHAR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - This register stores the second char used to replace seperator char in data . 0xdc 0xdb replace 0xc0 by default."]
    #[inline(always)]
    pub fn uhci_seper_esc_char1(&mut self) -> UHCI_SEPER_ESC_CHAR1_W {
        UHCI_SEPER_ESC_CHAR1_W { w: self }
    }
    #[doc = "Bits 8:15 - This register stores thee first char used to replace seperator char in data."]
    #[inline(always)]
    pub fn uhci_seper_esc_char0(&mut self) -> UHCI_SEPER_ESC_CHAR0_W {
        UHCI_SEPER_ESC_CHAR0_W { w: self }
    }
    #[doc = "Bits 0:7 - This register stores the seperator char seperator char is used to seperate the data frame."]
    #[inline(always)]
    pub fn uhci_seper_char(&mut self) -> UHCI_SEPER_CHAR_W {
        UHCI_SEPER_CHAR_W { w: self }
    }
}
