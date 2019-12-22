#[doc = "Reader of register MCPWM_FH2_STATUS_REG"]
pub type R = crate::R<u32, super::MCPWM_FH2_STATUS_REG>;
#[doc = "Writer for register MCPWM_FH2_STATUS_REG"]
pub type W = crate::W<u32, super::MCPWM_FH2_STATUS_REG>;
#[doc = "Register MCPWM_FH2_STATUS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCPWM_FH2_STATUS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_FH2_OST_ON`"]
pub type MCPWM_FH2_OST_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_OST_ON`"]
pub struct MCPWM_FH2_OST_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_OST_ON_W<'a> {
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
#[doc = "Reader of field `MCPWM_FH2_CBC_ON`"]
pub type MCPWM_FH2_CBC_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_CBC_ON`"]
pub struct MCPWM_FH2_CBC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_CBC_ON_W<'a> {
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
    #[doc = "Bit 1 - Set and reset by hardware. If set an one-shot mode action is on going"]
    #[inline(always)]
    pub fn mcpwm_fh2_ost_on(&self) -> MCPWM_FH2_OST_ON_R {
        MCPWM_FH2_OST_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set and reset by hardware. If set an cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn mcpwm_fh2_cbc_on(&self) -> MCPWM_FH2_CBC_ON_R {
        MCPWM_FH2_CBC_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set and reset by hardware. If set an one-shot mode action is on going"]
    #[inline(always)]
    pub fn mcpwm_fh2_ost_on(&mut self) -> MCPWM_FH2_OST_ON_W {
        MCPWM_FH2_OST_ON_W { w: self }
    }
    #[doc = "Bit 0 - Set and reset by hardware. If set an cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn mcpwm_fh2_cbc_on(&mut self) -> MCPWM_FH2_CBC_ON_W {
        MCPWM_FH2_CBC_ON_W { w: self }
    }
}
