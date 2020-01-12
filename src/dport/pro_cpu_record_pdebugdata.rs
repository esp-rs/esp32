#[doc = "Reader of register PRO_CPU_RECORD_PDEBUGDATA"]
pub type R = crate::R<u32, super::PRO_CPU_RECORD_PDEBUGDATA>;
#[doc = "Writer for register PRO_CPU_RECORD_PDEBUGDATA"]
pub type W = crate::W<u32, super::PRO_CPU_RECORD_PDEBUGDATA>;
#[doc = "Register PRO_CPU_RECORD_PDEBUGDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_CPU_RECORD_PDEBUGDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RECORD_PRO_PDEBUGDATA`"]
pub type RECORD_PRO_PDEBUGDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RECORD_PRO_PDEBUGDATA`"]
pub struct RECORD_PRO_PDEBUGDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PRO_PDEBUGDATA_W<'a> {
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
    pub fn record_pro_pdebugdata(&self) -> RECORD_PRO_PDEBUGDATA_R {
        RECORD_PRO_PDEBUGDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebugdata(&mut self) -> RECORD_PRO_PDEBUGDATA_W {
        RECORD_PRO_PDEBUGDATA_W { w: self }
    }
}
