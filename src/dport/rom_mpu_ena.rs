#[doc = "Reader of register ROM_MPU_ENA"]
pub type R = crate::R<u32, super::ROM_MPU_ENA>;
#[doc = "Writer for register ROM_MPU_ENA"]
pub type W = crate::W<u32, super::ROM_MPU_ENA>;
#[doc = "Register ROM_MPU_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::ROM_MPU_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_ROM_MPU_ENA`"]
pub type APP_ROM_MPU_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_ROM_MPU_ENA`"]
pub struct APP_ROM_MPU_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_ROM_MPU_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PRO_ROM_MPU_ENA`"]
pub type PRO_ROM_MPU_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_ROM_MPU_ENA`"]
pub struct PRO_ROM_MPU_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ROM_MPU_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SHARE_ROM_MPU_ENA`"]
pub type SHARE_ROM_MPU_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHARE_ROM_MPU_ENA`"]
pub struct SHARE_ROM_MPU_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SHARE_ROM_MPU_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_rom_mpu_ena(&self) -> APP_ROM_MPU_ENA_R {
        APP_ROM_MPU_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_rom_mpu_ena(&self) -> PRO_ROM_MPU_ENA_R {
        PRO_ROM_MPU_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn share_rom_mpu_ena(&self) -> SHARE_ROM_MPU_ENA_R {
        SHARE_ROM_MPU_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_rom_mpu_ena(&mut self) -> APP_ROM_MPU_ENA_W {
        APP_ROM_MPU_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_rom_mpu_ena(&mut self) -> PRO_ROM_MPU_ENA_W {
        PRO_ROM_MPU_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn share_rom_mpu_ena(&mut self) -> SHARE_ROM_MPU_ENA_W {
        SHARE_ROM_MPU_ENA_W { w: self }
    }
}
