#[doc = "Reader of register DPORT_IRAM_DRAM_AHB_SEL_REG"]
pub type R = crate::R<u32, super::DPORT_IRAM_DRAM_AHB_SEL_REG>;
#[doc = "Writer for register DPORT_IRAM_DRAM_AHB_SEL_REG"]
pub type W = crate::W<u32, super::DPORT_IRAM_DRAM_AHB_SEL_REG>;
#[doc = "Register DPORT_IRAM_DRAM_AHB_SEL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_IRAM_DRAM_AHB_SEL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_MAC_DUMP_MODE`"]
pub type DPORT_MAC_DUMP_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_MAC_DUMP_MODE`"]
pub struct DPORT_MAC_DUMP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_MAC_DUMP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `DPORT_MASK_AHB`"]
pub type DPORT_MASK_AHB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_MASK_AHB`"]
pub struct DPORT_MASK_AHB_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_MASK_AHB_W<'a> {
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
#[doc = "Reader of field `DPORT_MASK_APP_DRAM`"]
pub type DPORT_MASK_APP_DRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_MASK_APP_DRAM`"]
pub struct DPORT_MASK_APP_DRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_MASK_APP_DRAM_W<'a> {
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
#[doc = "Reader of field `DPORT_MASK_PRO_DRAM`"]
pub type DPORT_MASK_PRO_DRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_MASK_PRO_DRAM`"]
pub struct DPORT_MASK_PRO_DRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_MASK_PRO_DRAM_W<'a> {
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
#[doc = "Reader of field `DPORT_MASK_APP_IRAM`"]
pub type DPORT_MASK_APP_IRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_MASK_APP_IRAM`"]
pub struct DPORT_MASK_APP_IRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_MASK_APP_IRAM_W<'a> {
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
#[doc = "Reader of field `DPORT_MASK_PRO_IRAM`"]
pub type DPORT_MASK_PRO_IRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_MASK_PRO_IRAM`"]
pub struct DPORT_MASK_PRO_IRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_MASK_PRO_IRAM_W<'a> {
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
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn dport_mac_dump_mode(&self) -> DPORT_MAC_DUMP_MODE_R {
        DPORT_MAC_DUMP_MODE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dport_mask_ahb(&self) -> DPORT_MASK_AHB_R {
        DPORT_MASK_AHB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_mask_app_dram(&self) -> DPORT_MASK_APP_DRAM_R {
        DPORT_MASK_APP_DRAM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_mask_pro_dram(&self) -> DPORT_MASK_PRO_DRAM_R {
        DPORT_MASK_PRO_DRAM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_mask_app_iram(&self) -> DPORT_MASK_APP_IRAM_R {
        DPORT_MASK_APP_IRAM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_mask_pro_iram(&self) -> DPORT_MASK_PRO_IRAM_R {
        DPORT_MASK_PRO_IRAM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn dport_mac_dump_mode(&mut self) -> DPORT_MAC_DUMP_MODE_W {
        DPORT_MAC_DUMP_MODE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dport_mask_ahb(&mut self) -> DPORT_MASK_AHB_W {
        DPORT_MASK_AHB_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_mask_app_dram(&mut self) -> DPORT_MASK_APP_DRAM_W {
        DPORT_MASK_APP_DRAM_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_mask_pro_dram(&mut self) -> DPORT_MASK_PRO_DRAM_W {
        DPORT_MASK_PRO_DRAM_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dport_mask_app_iram(&mut self) -> DPORT_MASK_APP_IRAM_W {
        DPORT_MASK_APP_IRAM_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_mask_pro_iram(&mut self) -> DPORT_MASK_PRO_IRAM_W {
        DPORT_MASK_PRO_IRAM_W { w: self }
    }
}
