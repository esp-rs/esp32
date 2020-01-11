#[doc = "Reader of register VERSION"]
pub type R = crate::R<u32, super::VERSION>;
#[doc = "Writer for register VERSION"]
pub type W = crate::W<u32, super::VERSION>;
#[doc = "Register VERSION `reset()`'s with value 0"]
impl crate::ResetValue for super::VERSION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_DATE`"]
pub type MCPWM_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCPWM_DATE`"]
pub struct MCPWM_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - Version of this reg file"]
    #[inline(always)]
    pub fn mcpwm_date(&self) -> MCPWM_DATE_R {
        MCPWM_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version of this reg file"]
    #[inline(always)]
    pub fn mcpwm_date(&mut self) -> MCPWM_DATE_W {
        MCPWM_DATE_W { w: self }
    }
}
