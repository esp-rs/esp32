#[doc = "Reader of register EXT_WAKEUP1_STATUS"]
pub type R = crate::R<u32, super::EXT_WAKEUP1_STATUS>;
#[doc = "Writer for register EXT_WAKEUP1_STATUS"]
pub type W = crate::W<u32, super::EXT_WAKEUP1_STATUS>;
#[doc = "Register EXT_WAKEUP1_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_WAKEUP1_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXT_WAKEUP1_STATUS`"]
pub type EXT_WAKEUP1_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EXT_WAKEUP1_STATUS`"]
pub struct EXT_WAKEUP1_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_WAKEUP1_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - ext wakeup1 status"]
    #[inline(always)]
    pub fn ext_wakeup1_status(&self) -> EXT_WAKEUP1_STATUS_R {
        EXT_WAKEUP1_STATUS_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - ext wakeup1 status"]
    #[inline(always)]
    pub fn ext_wakeup1_status(&mut self) -> EXT_WAKEUP1_STATUS_W {
        EXT_WAKEUP1_STATUS_W { w: self }
    }
}
