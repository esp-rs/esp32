#[doc = "Reader of register PRO_CPU_RECORD_PDEBUGINST"]
pub type R = crate::R<u32, super::PRO_CPU_RECORD_PDEBUGINST>;
#[doc = "Writer for register PRO_CPU_RECORD_PDEBUGINST"]
pub type W = crate::W<u32, super::PRO_CPU_RECORD_PDEBUGINST>;
#[doc = "Register PRO_CPU_RECORD_PDEBUGINST `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_CPU_RECORD_PDEBUGINST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RECORD_PRO_PDEBUGINST`"]
pub type RECORD_PRO_PDEBUGINST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RECORD_PRO_PDEBUGINST`"]
pub struct RECORD_PRO_PDEBUGINST_W<'a> {
    w: &'a mut W,
}
impl<'a> RECORD_PRO_PDEBUGINST_W<'a> {
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
    pub fn record_pro_pdebuginst(&self) -> RECORD_PRO_PDEBUGINST_R {
        RECORD_PRO_PDEBUGINST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebuginst(&mut self) -> RECORD_PRO_PDEBUGINST_W {
        RECORD_PRO_PDEBUGINST_W { w: self }
    }
}
