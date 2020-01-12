#[doc = "Reader of register MEM_ACCESS_DBUG0"]
pub type R = crate::R<u32, super::MEM_ACCESS_DBUG0>;
#[doc = "Writer for register MEM_ACCESS_DBUG0"]
pub type W = crate::W<u32, super::MEM_ACCESS_DBUG0>;
#[doc = "Register MEM_ACCESS_DBUG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_ACCESS_DBUG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERNAL_SRAM_MMU_MULTI_HIT`"]
pub type INTERNAL_SRAM_MMU_MULTI_HIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTERNAL_SRAM_MMU_MULTI_HIT`"]
pub struct INTERNAL_SRAM_MMU_MULTI_HIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_MMU_MULTI_HIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | (((value as u32) & 0x0f) << 26);
        self.w
    }
}
#[doc = "Reader of field `INTERNAL_SRAM_IA`"]
pub type INTERNAL_SRAM_IA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTERNAL_SRAM_IA`"]
pub struct INTERNAL_SRAM_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 14)) | (((value as u32) & 0x0fff) << 14);
        self.w
    }
}
#[doc = "Reader of field `INTERNAL_SRAM_MMU_AD`"]
pub type INTERNAL_SRAM_MMU_AD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTERNAL_SRAM_MMU_AD`"]
pub struct INTERNAL_SRAM_MMU_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_MMU_AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SHARE_ROM_IA`"]
pub type SHARE_ROM_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHARE_ROM_IA`"]
pub struct SHARE_ROM_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> SHARE_ROM_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `SHARE_ROM_MPU_AD`"]
pub type SHARE_ROM_MPU_AD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHARE_ROM_MPU_AD`"]
pub struct SHARE_ROM_MPU_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> SHARE_ROM_MPU_AD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `APP_ROM_IA`"]
pub type APP_ROM_IA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_ROM_IA`"]
pub struct APP_ROM_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_ROM_IA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `APP_ROM_MPU_AD`"]
pub type APP_ROM_MPU_AD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_ROM_MPU_AD`"]
pub struct APP_ROM_MPU_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_ROM_MPU_AD_W<'a> {
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
#[doc = "Reader of field `PRO_ROM_IA`"]
pub type PRO_ROM_IA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_ROM_IA`"]
pub struct PRO_ROM_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ROM_IA_W<'a> {
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
#[doc = "Reader of field `PRO_ROM_MPU_AD`"]
pub type PRO_ROM_MPU_AD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_ROM_MPU_AD`"]
pub struct PRO_ROM_MPU_AD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_ROM_MPU_AD_W<'a> {
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
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn internal_sram_mmu_multi_hit(&self) -> INTERNAL_SRAM_MMU_MULTI_HIT_R {
        INTERNAL_SRAM_MMU_MULTI_HIT_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 14:25"]
    #[inline(always)]
    pub fn internal_sram_ia(&self) -> INTERNAL_SRAM_IA_R {
        INTERNAL_SRAM_IA_R::new(((self.bits >> 14) & 0x0fff) as u16)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn internal_sram_mmu_ad(&self) -> INTERNAL_SRAM_MMU_AD_R {
        INTERNAL_SRAM_MMU_AD_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn share_rom_ia(&self) -> SHARE_ROM_IA_R {
        SHARE_ROM_IA_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn share_rom_mpu_ad(&self) -> SHARE_ROM_MPU_AD_R {
        SHARE_ROM_MPU_AD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_rom_ia(&self) -> APP_ROM_IA_R {
        APP_ROM_IA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_rom_mpu_ad(&self) -> APP_ROM_MPU_AD_R {
        APP_ROM_MPU_AD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_rom_ia(&self) -> PRO_ROM_IA_R {
        PRO_ROM_IA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_mpu_ad(&self) -> PRO_ROM_MPU_AD_R {
        PRO_ROM_MPU_AD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn internal_sram_mmu_multi_hit(&mut self) -> INTERNAL_SRAM_MMU_MULTI_HIT_W {
        INTERNAL_SRAM_MMU_MULTI_HIT_W { w: self }
    }
    #[doc = "Bits 14:25"]
    #[inline(always)]
    pub fn internal_sram_ia(&mut self) -> INTERNAL_SRAM_IA_W {
        INTERNAL_SRAM_IA_W { w: self }
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn internal_sram_mmu_ad(&mut self) -> INTERNAL_SRAM_MMU_AD_W {
        INTERNAL_SRAM_MMU_AD_W { w: self }
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn share_rom_ia(&mut self) -> SHARE_ROM_IA_W {
        SHARE_ROM_IA_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn share_rom_mpu_ad(&mut self) -> SHARE_ROM_MPU_AD_W {
        SHARE_ROM_MPU_AD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_rom_ia(&mut self) -> APP_ROM_IA_W {
        APP_ROM_IA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_rom_mpu_ad(&mut self) -> APP_ROM_MPU_AD_W {
        APP_ROM_MPU_AD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_rom_ia(&mut self) -> PRO_ROM_IA_W {
        PRO_ROM_IA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_mpu_ad(&mut self) -> PRO_ROM_MPU_AD_W {
        PRO_ROM_MPU_AD_W { w: self }
    }
}
