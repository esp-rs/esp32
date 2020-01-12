#[doc = "Reader of register MEM_ACCESS_DBUG1"]
pub type R = crate::R<u32, super::MEM_ACCESS_DBUG1>;
#[doc = "Writer for register MEM_ACCESS_DBUG1"]
pub type W = crate::W<u32, super::MEM_ACCESS_DBUG1>;
#[doc = "Register MEM_ACCESS_DBUG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_ACCESS_DBUG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AHBLITE_IA`"]
pub type AHBLITE_IA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBLITE_IA`"]
pub struct AHBLITE_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBLITE_IA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `AHBLITE_ACCESS_DENY`"]
pub type AHBLITE_ACCESS_DENY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBLITE_ACCESS_DENY`"]
pub struct AHBLITE_ACCESS_DENY_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBLITE_ACCESS_DENY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `AHB_ACCESS_DENY`"]
pub type AHB_ACCESS_DENY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_ACCESS_DENY`"]
pub struct AHB_ACCESS_DENY_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_ACCESS_DENY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PIDGEN_IA`"]
pub type PIDGEN_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIDGEN_IA`"]
pub struct PIDGEN_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> PIDGEN_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `ARB_IA`"]
pub type ARB_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ARB_IA`"]
pub struct ARB_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `INTERNAL_SRAM_MMU_MISS`"]
pub type INTERNAL_SRAM_MMU_MISS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTERNAL_SRAM_MMU_MISS`"]
pub struct INTERNAL_SRAM_MMU_MISS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_SRAM_MMU_MISS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ahblite_ia(&self) -> AHBLITE_IA_R {
        AHBLITE_IA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ahblite_access_deny(&self) -> AHBLITE_ACCESS_DENY_R {
        AHBLITE_ACCESS_DENY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ahb_access_deny(&self) -> AHB_ACCESS_DENY_R {
        AHB_ACCESS_DENY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pidgen_ia(&self) -> PIDGEN_IA_R {
        PIDGEN_IA_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn arb_ia(&self) -> ARB_IA_R {
        ARB_IA_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn internal_sram_mmu_miss(&self) -> INTERNAL_SRAM_MMU_MISS_R {
        INTERNAL_SRAM_MMU_MISS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ahblite_ia(&mut self) -> AHBLITE_IA_W {
        AHBLITE_IA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ahblite_access_deny(&mut self) -> AHBLITE_ACCESS_DENY_W {
        AHBLITE_ACCESS_DENY_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ahb_access_deny(&mut self) -> AHB_ACCESS_DENY_W {
        AHB_ACCESS_DENY_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pidgen_ia(&mut self) -> PIDGEN_IA_W {
        PIDGEN_IA_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn arb_ia(&mut self) -> ARB_IA_W {
        ARB_IA_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn internal_sram_mmu_miss(&mut self) -> INTERNAL_SRAM_MMU_MISS_W {
        INTERNAL_SRAM_MMU_MISS_W { w: self }
    }
}
