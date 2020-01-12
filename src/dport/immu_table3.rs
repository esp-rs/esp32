#[doc = "Reader of register IMMU_TABLE3"]
pub type R = crate::R<u32, super::IMMU_TABLE3>;
#[doc = "Writer for register IMMU_TABLE3"]
pub type W = crate::W<u32, super::IMMU_TABLE3>;
#[doc = "Register IMMU_TABLE3 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMMU_TABLE3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMMU_TABLE3`"]
pub type IMMU_TABLE3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IMMU_TABLE3`"]
pub struct IMMU_TABLE3_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMU_TABLE3_W<'a> {
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
    pub fn immu_table3(&self) -> IMMU_TABLE3_R {
        IMMU_TABLE3_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn immu_table3(&mut self) -> IMMU_TABLE3_W {
        IMMU_TABLE3_W { w: self }
    }
}
