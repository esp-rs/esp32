#[doc = "Reader of register ROM_MPU_TABLE2"]
pub type R = crate::R<u32, super::ROM_MPU_TABLE2>;
#[doc = "Writer for register ROM_MPU_TABLE2"]
pub type W = crate::W<u32, super::ROM_MPU_TABLE2>;
#[doc = "Register ROM_MPU_TABLE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROM_MPU_TABLE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ROM_MPU_TABLE2`"]
pub type ROM_MPU_TABLE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ROM_MPU_TABLE2`"]
pub struct ROM_MPU_TABLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_MPU_TABLE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_mpu_table2(&self) -> ROM_MPU_TABLE2_R {
        ROM_MPU_TABLE2_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rom_mpu_table2(&mut self) -> ROM_MPU_TABLE2_W {
        ROM_MPU_TABLE2_W { w: self }
    }
}
