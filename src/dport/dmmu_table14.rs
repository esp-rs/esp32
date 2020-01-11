#[doc = "Reader of register DMMU_TABLE14"]
pub type R = crate::R<u32, super::DMMU_TABLE14>;
#[doc = "Writer for register DMMU_TABLE14"]
pub type W = crate::W<u32, super::DMMU_TABLE14>;
#[doc = "Register DMMU_TABLE14 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMMU_TABLE14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_DMMU_TABLE14`"]
pub type DPORT_DMMU_TABLE14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_DMMU_TABLE14`"]
pub struct DPORT_DMMU_TABLE14_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_DMMU_TABLE14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dport_dmmu_table14(&self) -> DPORT_DMMU_TABLE14_R {
        DPORT_DMMU_TABLE14_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dport_dmmu_table14(&mut self) -> DPORT_DMMU_TABLE14_W {
        DPORT_DMMU_TABLE14_W { w: self }
    }
}
