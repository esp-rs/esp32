#[doc = "Reader of register Q4_WORD0"]
pub type R = crate::R<u32, super::Q4_WORD0>;
#[doc = "Writer for register Q4_WORD0"]
pub type W = crate::W<u32, super::Q4_WORD0>;
#[doc = "Register Q4_WORD0 `reset()`'s with value 0"]
impl crate::ResetValue for super::Q4_WORD0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEND_Q4_WORD0`"]
pub type SEND_Q4_WORD0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEND_Q4_WORD0`"]
pub struct SEND_Q4_WORD0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_Q4_WORD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This register stores the content of short packet's first dword"]
    #[inline(always)]
    pub fn send_q4_word0(&self) -> SEND_Q4_WORD0_R {
        SEND_Q4_WORD0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the content of short packet's first dword"]
    #[inline(always)]
    pub fn send_q4_word0(&mut self) -> SEND_Q4_WORD0_W {
        SEND_Q4_WORD0_W { w: self }
    }
}
