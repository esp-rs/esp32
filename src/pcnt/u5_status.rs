#[doc = "Reader of register U5_STATUS"]
pub type R = crate::R<u32, super::U5_STATUS>;
#[doc = "Writer for register U5_STATUS"]
pub type W = crate::W<u32, super::U5_STATUS>;
#[doc = "Register U5_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::U5_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT_CORE_STATUS_U5`"]
pub type PCNT_CORE_STATUS_U5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PCNT_CORE_STATUS_U5`"]
pub struct PCNT_CORE_STATUS_U5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CORE_STATUS_U5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pcnt_core_status_u5(&self) -> PCNT_CORE_STATUS_U5_R {
        PCNT_CORE_STATUS_U5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pcnt_core_status_u5(&mut self) -> PCNT_CORE_STATUS_U5_W {
        PCNT_CORE_STATUS_U5_W { w: self }
    }
}
