#[doc = "Reader of register LACTALARMLO"]
pub type R = crate::R<u32, super::LACTALARMLO>;
#[doc = "Writer for register LACTALARMLO"]
pub type W = crate::W<u32, super::LACTALARMLO>;
#[doc = "Register LACTALARMLO `reset()`'s with value 0"]
impl crate::ResetValue for super::LACTALARMLO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LACT_ALARM_LO`"]
pub type LACT_ALARM_LO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LACT_ALARM_LO`"]
pub struct LACT_ALARM_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> LACT_ALARM_LO_W<'a> {
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
    pub fn lact_alarm_lo(&self) -> LACT_ALARM_LO_R {
        LACT_ALARM_LO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_alarm_lo(&mut self) -> LACT_ALARM_LO_W {
        LACT_ALARM_LO_W { w: self }
    }
}
