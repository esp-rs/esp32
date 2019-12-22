#[doc = "Reader of register DPORT_SRAM_FO_CTRL_1_REG"]
pub type R = crate::R<u32, super::DPORT_SRAM_FO_CTRL_1_REG>;
#[doc = "Writer for register DPORT_SRAM_FO_CTRL_1_REG"]
pub type W = crate::W<u32, super::DPORT_SRAM_FO_CTRL_1_REG>;
#[doc = "Register DPORT_SRAM_FO_CTRL_1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_SRAM_FO_CTRL_1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_SRAM_FO_1`"]
pub type DPORT_SRAM_FO_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_SRAM_FO_1`"]
pub struct DPORT_SRAM_FO_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_SRAM_FO_1_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_sram_fo_1(&self) -> DPORT_SRAM_FO_1_R {
        DPORT_SRAM_FO_1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_sram_fo_1(&mut self) -> DPORT_SRAM_FO_1_W {
        DPORT_SRAM_FO_1_W { w: self }
    }
}
