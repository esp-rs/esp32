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
#[doc = "Reader of field `CHAR_NUM`"]
pub type CHAR_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHAR_NUM`"]
pub struct CHAR_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAR_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `AT_CMD_CHAR`"]
pub type AT_CMD_CHAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AT_CMD_CHAR`"]
pub struct AT_CMD_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_CMD_CHAR_W<'a> {
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
    pub fn char_num(&self) -> CHAR_NUM_R {
        CHAR_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - This register is used to configure the content of at_cmd char."]
    #[inline(always)]
    pub fn at_cmd_char(&self) -> AT_CMD_CHAR_R {
        AT_CMD_CHAR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - This register is used to configure the num of continous at_cmd chars received by receiver."]
    #[inline(always)]
    pub fn char_num(&mut self) -> CHAR_NUM_W {
        CHAR_NUM_W { w: self }
    }
    #[doc = "Bits 0:7 - This register is used to configure the content of at_cmd char."]
    #[inline(always)]
    pub fn at_cmd_char(&mut self) -> AT_CMD_CHAR_W {
        AT_CMD_CHAR_W { w: self }
    }
}
