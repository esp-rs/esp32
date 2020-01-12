#[doc = "Reader of register APP_CPU_RECORD_PDEBUGLS0STAT"]
pub type R = crate::R<u32, super::APP_CPU_RECORD_PDEBUGLS0STAT>;
#[doc = "Writer for register APP_CPU_RECORD_PDEBUGLS0STAT"]
pub type W = crate::W<u32, super::APP_CPU_RECORD_PDEBUGLS0STAT>;
#[doc = "Register APP_CPU_RECORD_PDEBUGLS0STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_CPU_RECORD_PDEBUGLS0STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RECORD_APP_PDEBUGLS0STAT`"]
pub type RECORD_APP_PDEBUGLS0STAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RECORD_APP_PDEBUGLS0STAT`"]
pub struct RECORD_APP_PDEBUGLS0STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_APP_PDEBUGLS0STAT_W<'a> {
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
    pub fn record_app_pdebugls0stat(&self) -> RECORD_APP_PDEBUGLS0STAT_R {
        RECORD_APP_PDEBUGLS0STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_app_pdebugls0stat(&mut self) -> RECORD_APP_PDEBUGLS0STAT_W {
        RECORD_APP_PDEBUGLS0STAT_W { w: self }
    }
}
