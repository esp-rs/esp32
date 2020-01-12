#[doc = "Reader of register DMMU_TABLE0"]
pub type R = crate::R<u32, super::DMMU_TABLE0>;
#[doc = "Writer for register DMMU_TABLE0"]
pub type W = crate::W<u32, super::DMMU_TABLE0>;
#[doc = "Register DMMU_TABLE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMMU_TABLE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMMU_TABLE0`"]
pub type DMMU_TABLE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMMU_TABLE0`"]
pub struct DMMU_TABLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMMU_TABLE0_W<'a> {
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
    pub fn dmmu_table0(&self) -> DMMU_TABLE0_R {
        DMMU_TABLE0_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn dmmu_table0(&mut self) -> DMMU_TABLE0_W {
        DMMU_TABLE0_W { w: self }
    }
}