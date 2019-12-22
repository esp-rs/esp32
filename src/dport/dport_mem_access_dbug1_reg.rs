#[doc = "Reader of register DPORT_MEM_ACCESS_DBUG1_REG"]
pub type R = crate::R<u32, super::DPORT_MEM_ACCESS_DBUG1_REG>;
#[doc = "Writer for register DPORT_MEM_ACCESS_DBUG1_REG"]
pub type W = crate::W<u32, super::DPORT_MEM_ACCESS_DBUG1_REG>;
#[doc = "Register DPORT_MEM_ACCESS_DBUG1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_MEM_ACCESS_DBUG1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_AHBLITE_IA`"]
pub type DPORT_AHBLITE_IA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AHBLITE_IA`"]
pub struct DPORT_AHBLITE_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHBLITE_IA_W<'a> {
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
#[doc = "Reader of field `DPORT_AHBLITE_ACCESS_DENY`"]
pub type DPORT_AHBLITE_ACCESS_DENY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AHBLITE_ACCESS_DENY`"]
pub struct DPORT_AHBLITE_ACCESS_DENY_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHBLITE_ACCESS_DENY_W<'a> {
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
#[doc = "Reader of field `DPORT_AHB_ACCESS_DENY`"]
pub type DPORT_AHB_ACCESS_DENY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AHB_ACCESS_DENY`"]
pub struct DPORT_AHB_ACCESS_DENY_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHB_ACCESS_DENY_W<'a> {
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
#[doc = "Reader of field `DPORT_PIDGEN_IA`"]
pub type DPORT_PIDGEN_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_PIDGEN_IA`"]
pub struct DPORT_PIDGEN_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PIDGEN_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DPORT_ARB_IA`"]
pub type DPORT_ARB_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_ARB_IA`"]
pub struct DPORT_ARB_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_ARB_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DPORT_INTERNAL_SRAM_MMU_MISS`"]
pub type DPORT_INTERNAL_SRAM_MMU_MISS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_INTERNAL_SRAM_MMU_MISS`"]
pub struct DPORT_INTERNAL_SRAM_MMU_MISS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_INTERNAL_SRAM_MMU_MISS_W<'a> {
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
    pub fn dport_ahblite_ia(&self) -> DPORT_AHBLITE_IA_R {
        DPORT_AHBLITE_IA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dport_ahblite_access_deny(&self) -> DPORT_AHBLITE_ACCESS_DENY_R {
        DPORT_AHBLITE_ACCESS_DENY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_ahb_access_deny(&self) -> DPORT_AHB_ACCESS_DENY_R {
        DPORT_AHB_ACCESS_DENY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn dport_pidgen_ia(&self) -> DPORT_PIDGEN_IA_R {
        DPORT_PIDGEN_IA_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dport_arb_ia(&self) -> DPORT_ARB_IA_R {
        DPORT_ARB_IA_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dport_internal_sram_mmu_miss(&self) -> DPORT_INTERNAL_SRAM_MMU_MISS_R {
        DPORT_INTERNAL_SRAM_MMU_MISS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dport_ahblite_ia(&mut self) -> DPORT_AHBLITE_IA_W {
        DPORT_AHBLITE_IA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dport_ahblite_access_deny(&mut self) -> DPORT_AHBLITE_ACCESS_DENY_W {
        DPORT_AHBLITE_ACCESS_DENY_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_ahb_access_deny(&mut self) -> DPORT_AHB_ACCESS_DENY_W {
        DPORT_AHB_ACCESS_DENY_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn dport_pidgen_ia(&mut self) -> DPORT_PIDGEN_IA_W {
        DPORT_PIDGEN_IA_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dport_arb_ia(&mut self) -> DPORT_ARB_IA_W {
        DPORT_ARB_IA_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dport_internal_sram_mmu_miss(&mut self) -> DPORT_INTERNAL_SRAM_MMU_MISS_W {
        DPORT_INTERNAL_SRAM_MMU_MISS_W { w: self }
    }
}
