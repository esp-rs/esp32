#[doc = "Reader of register APP_CACHE_CTRL1"]
pub type R = crate::R<u32, super::APP_CACHE_CTRL1>;
#[doc = "Writer for register APP_CACHE_CTRL1"]
pub type W = crate::W<u32, super::APP_CACHE_CTRL1>;
#[doc = "Register APP_CACHE_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_CACHE_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_CACHE_MMU_IA_CLR`"]
pub type APP_CACHE_MMU_IA_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_MMU_IA_CLR`"]
pub struct APP_CACHE_MMU_IA_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MMU_IA_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `APP_CMMU_PD`"]
pub type APP_CMMU_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CMMU_PD`"]
pub struct APP_CMMU_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CMMU_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `APP_CMMU_FORCE_ON`"]
pub type APP_CMMU_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CMMU_FORCE_ON`"]
pub struct APP_CMMU_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CMMU_FORCE_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `APP_CMMU_FLASH_PAGE_MODE`"]
pub type APP_CMMU_FLASH_PAGE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APP_CMMU_FLASH_PAGE_MODE`"]
pub struct APP_CMMU_FLASH_PAGE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CMMU_FLASH_PAGE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `APP_CMMU_SRAM_PAGE_MODE`"]
pub type APP_CMMU_SRAM_PAGE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APP_CMMU_SRAM_PAGE_MODE`"]
pub struct APP_CMMU_SRAM_PAGE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CMMU_SRAM_PAGE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `APP_CACHE_MASK_OPSDRAM`"]
pub type APP_CACHE_MASK_OPSDRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_MASK_OPSDRAM`"]
pub struct APP_CACHE_MASK_OPSDRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_OPSDRAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `APP_CACHE_MASK_DROM0`"]
pub type APP_CACHE_MASK_DROM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_MASK_DROM0`"]
pub struct APP_CACHE_MASK_DROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_DROM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `APP_CACHE_MASK_DRAM1`"]
pub type APP_CACHE_MASK_DRAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_MASK_DRAM1`"]
pub struct APP_CACHE_MASK_DRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_DRAM1_W<'a> {
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
#[doc = "Reader of field `APP_CACHE_MASK_IROM0`"]
pub type APP_CACHE_MASK_IROM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_MASK_IROM0`"]
pub struct APP_CACHE_MASK_IROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_IROM0_W<'a> {
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
#[doc = "Reader of field `APP_CACHE_MASK_IRAM1`"]
pub type APP_CACHE_MASK_IRAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_MASK_IRAM1`"]
pub struct APP_CACHE_MASK_IRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_IRAM1_W<'a> {
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
#[doc = "Reader of field `APP_CACHE_MASK_IRAM0`"]
pub type APP_CACHE_MASK_IRAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_MASK_IRAM0`"]
pub struct APP_CACHE_MASK_IRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MASK_IRAM0_W<'a> {
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn app_cache_mmu_ia_clr(&self) -> APP_CACHE_MMU_IA_CLR_R {
        APP_CACHE_MMU_IA_CLR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn app_cmmu_pd(&self) -> APP_CMMU_PD_R {
        APP_CMMU_PD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_cmmu_force_on(&self) -> APP_CMMU_FORCE_ON_R {
        APP_CMMU_FORCE_ON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn app_cmmu_flash_page_mode(&self) -> APP_CMMU_FLASH_PAGE_MODE_R {
        APP_CMMU_FLASH_PAGE_MODE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn app_cmmu_sram_page_mode(&self) -> APP_CMMU_SRAM_PAGE_MODE_R {
        APP_CMMU_SRAM_PAGE_MODE_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn app_cache_mask_opsdram(&self) -> APP_CACHE_MASK_OPSDRAM_R {
        APP_CACHE_MASK_OPSDRAM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_mask_drom0(&self) -> APP_CACHE_MASK_DROM0_R {
        APP_CACHE_MASK_DROM0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_mask_dram1(&self) -> APP_CACHE_MASK_DRAM1_R {
        APP_CACHE_MASK_DRAM1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mask_irom0(&self) -> APP_CACHE_MASK_IROM0_R {
        APP_CACHE_MASK_IROM0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_cache_mask_iram1(&self) -> APP_CACHE_MASK_IRAM1_R {
        APP_CACHE_MASK_IRAM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cache_mask_iram0(&self) -> APP_CACHE_MASK_IRAM0_R {
        APP_CACHE_MASK_IRAM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn app_cache_mmu_ia_clr(&mut self) -> APP_CACHE_MMU_IA_CLR_W {
        APP_CACHE_MMU_IA_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn app_cmmu_pd(&mut self) -> APP_CMMU_PD_W {
        APP_CMMU_PD_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_cmmu_force_on(&mut self) -> APP_CMMU_FORCE_ON_W {
        APP_CMMU_FORCE_ON_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn app_cmmu_flash_page_mode(&mut self) -> APP_CMMU_FLASH_PAGE_MODE_W {
        APP_CMMU_FLASH_PAGE_MODE_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn app_cmmu_sram_page_mode(&mut self) -> APP_CMMU_SRAM_PAGE_MODE_W {
        APP_CMMU_SRAM_PAGE_MODE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn app_cache_mask_opsdram(&mut self) -> APP_CACHE_MASK_OPSDRAM_W {
        APP_CACHE_MASK_OPSDRAM_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_mask_drom0(&mut self) -> APP_CACHE_MASK_DROM0_W {
        APP_CACHE_MASK_DROM0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_mask_dram1(&mut self) -> APP_CACHE_MASK_DRAM1_W {
        APP_CACHE_MASK_DRAM1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mask_irom0(&mut self) -> APP_CACHE_MASK_IROM0_W {
        APP_CACHE_MASK_IROM0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_cache_mask_iram1(&mut self) -> APP_CACHE_MASK_IRAM1_W {
        APP_CACHE_MASK_IRAM1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cache_mask_iram0(&mut self) -> APP_CACHE_MASK_IRAM0_W {
        APP_CACHE_MASK_IRAM0_W { w: self }
    }
}
