#[doc = "Reader of register ESC_CONF3"]
pub type R = crate::R<u32, super::ESC_CONF3>;
#[doc = "Writer for register ESC_CONF3"]
pub type W = crate::W<u32, super::ESC_CONF3>;
#[doc = "Register ESC_CONF3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ESC_CONF3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ESC_SEQ2_CHAR1`"]
pub type ESC_SEQ2_CHAR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ESC_SEQ2_CHAR1`"]
pub struct ESC_SEQ2_CHAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_SEQ2_CHAR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ESC_SEQ2_CHAR0`"]
pub type ESC_SEQ2_CHAR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ESC_SEQ2_CHAR0`"]
pub struct ESC_SEQ2_CHAR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_SEQ2_CHAR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ESC_SEQ2`"]
pub type ESC_SEQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ESC_SEQ2`"]
pub struct ESC_SEQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_SEQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - This register stores the second char used to replace the reg_esc_seq2 in data."]
    #[inline(always)]
    pub fn esc_seq2_char1(&self) -> ESC_SEQ2_CHAR1_R {
        ESC_SEQ2_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register stores the first char used to replace the reg_esc_seq2 in data."]
    #[inline(always)]
    pub fn esc_seq2_char0(&self) -> ESC_SEQ2_CHAR0_R {
        ESC_SEQ2_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - This register stores the flow_control char to turn off the flow_control"]
    #[inline(always)]
    pub fn esc_seq2(&self) -> ESC_SEQ2_R {
        ESC_SEQ2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - This register stores the second char used to replace the reg_esc_seq2 in data."]
    #[inline(always)]
    pub fn esc_seq2_char1(&mut self) -> ESC_SEQ2_CHAR1_W {
        ESC_SEQ2_CHAR1_W { w: self }
    }
    #[doc = "Bits 8:15 - This register stores the first char used to replace the reg_esc_seq2 in data."]
    #[inline(always)]
    pub fn esc_seq2_char0(&mut self) -> ESC_SEQ2_CHAR0_W {
        ESC_SEQ2_CHAR0_W { w: self }
    }
    #[doc = "Bits 0:7 - This register stores the flow_control char to turn off the flow_control"]
    #[inline(always)]
    pub fn esc_seq2(&mut self) -> ESC_SEQ2_W {
        ESC_SEQ2_W { w: self }
    }
}
