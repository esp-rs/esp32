#[doc = "Reader of register APP_CPU_RECORD_PDEBUGSTATUS"]
pub type R = crate::R<u32, super::APP_CPU_RECORD_PDEBUGSTATUS>;
#[doc = "Writer for register APP_CPU_RECORD_PDEBUGSTATUS"]
pub type W = crate::W<u32, super::APP_CPU_RECORD_PDEBUGSTATUS>;
#[doc = "Register APP_CPU_RECORD_PDEBUGSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_CPU_RECORD_PDEBUGSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RECORD_APP_PDEBUGSTATUS`"]
pub type RECORD_APP_PDEBUGSTATUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RECORD_APP_PDEBUGSTATUS`"]
pub struct RECORD_APP_PDEBUGSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_APP_PDEBUGSTATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_app_pdebugstatus(&self) -> RECORD_APP_PDEBUGSTATUS_R {
        RECORD_APP_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_app_pdebugstatus(&mut self) -> RECORD_APP_PDEBUGSTATUS_W {
        RECORD_APP_PDEBUGSTATUS_W { w: self }
    }
}
