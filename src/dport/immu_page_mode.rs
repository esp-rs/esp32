#[doc = "Reader of register IMMU_PAGE_MODE"]
pub type R = crate::R<u32, super::IMMU_PAGE_MODE>;
#[doc = "Writer for register IMMU_PAGE_MODE"]
pub type W = crate::W<u32, super::IMMU_PAGE_MODE>;
#[doc = "Register IMMU_PAGE_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::IMMU_PAGE_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMMU_PAGE_MODE`"]
pub type IMMU_PAGE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IMMU_PAGE_MODE`"]
pub struct IMMU_PAGE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IMMU_PAGE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `INTERNAL_SRAM_IMMU_ENA`"]
pub type INTERNAL_SRAM_IMMU_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERNAL_SRAM_IMMU_ENA`"]
pub struct INTERNAL_SRAM_IMMU_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_IMMU_ENA_W<'a> {
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
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn immu_page_mode(&self) -> IMMU_PAGE_MODE_R {
        IMMU_PAGE_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn internal_sram_immu_ena(&self) -> INTERNAL_SRAM_IMMU_ENA_R {
        INTERNAL_SRAM_IMMU_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn immu_page_mode(&mut self) -> IMMU_PAGE_MODE_W {
        IMMU_PAGE_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn internal_sram_immu_ena(&mut self) -> INTERNAL_SRAM_IMMU_ENA_W {
        INTERNAL_SRAM_IMMU_ENA_W { w: self }
    }
}
