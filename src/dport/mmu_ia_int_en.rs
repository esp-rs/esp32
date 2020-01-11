#[doc = "Reader of register MMU_IA_INT_EN"]
pub type R = crate::R<u32, super::MMU_IA_INT_EN>;
#[doc = "Writer for register MMU_IA_INT_EN"]
pub type W = crate::W<u32, super::MMU_IA_INT_EN>;
#[doc = "Register MMU_IA_INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::MMU_IA_INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_MMU_IA_INT_EN`"]
pub type DPORT_MMU_IA_INT_EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_MMU_IA_INT_EN`"]
pub struct DPORT_MMU_IA_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_MMU_IA_INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn dport_mmu_ia_int_en(&self) -> DPORT_MMU_IA_INT_EN_R {
        DPORT_MMU_IA_INT_EN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn dport_mmu_ia_int_en(&mut self) -> DPORT_MMU_IA_INT_EN_W {
        DPORT_MMU_IA_INT_EN_W { w: self }
    }
}
