#[doc = "Reader of register MPU_IA_INT_EN"]
pub type R = crate::R<u32, super::MPU_IA_INT_EN>;
#[doc = "Writer for register MPU_IA_INT_EN"]
pub type W = crate::W<u32, super::MPU_IA_INT_EN>;
#[doc = "Register MPU_IA_INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_IA_INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPU_IA_INT_EN`"]
pub type MPU_IA_INT_EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MPU_IA_INT_EN`"]
pub struct MPU_IA_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPU_IA_INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn mpu_ia_int_en(&self) -> MPU_IA_INT_EN_R {
        MPU_IA_INT_EN_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn mpu_ia_int_en(&mut self) -> MPU_IA_INT_EN_W {
        MPU_IA_INT_EN_W { w: self }
    }
}
