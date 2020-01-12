#[doc = "Reader of register UPDATE_CFG"]
pub type R = crate::R<u32, super::UPDATE_CFG>;
#[doc = "Writer for register UPDATE_CFG"]
pub type W = crate::W<u32, super::UPDATE_CFG>;
#[doc = "Register UPDATE_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::UPDATE_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OP2_FORCE_UP`"]
pub type OP2_FORCE_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP2_FORCE_UP`"]
pub struct OP2_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP2_FORCE_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `OP2_UP_EN`"]
pub type OP2_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP2_UP_EN`"]
pub struct OP2_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OP2_UP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `OP1_FORCE_UP`"]
pub type OP1_FORCE_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP1_FORCE_UP`"]
pub struct OP1_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_FORCE_UP_W<'a> {
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
#[doc = "Reader of field `OP1_UP_EN`"]
pub type OP1_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP1_UP_EN`"]
pub struct OP1_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_UP_EN_W<'a> {
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
#[doc = "Reader of field `OP0_FORCE_UP`"]
pub type OP0_FORCE_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP0_FORCE_UP`"]
pub struct OP0_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> OP0_FORCE_UP_W<'a> {
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
#[doc = "Reader of field `OP0_UP_EN`"]
pub type OP0_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP0_UP_EN`"]
pub struct OP0_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OP0_UP_EN_W<'a> {
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
#[doc = "Reader of field `GLOBAL_FORCE_UP`"]
pub type GLOBAL_FORCE_UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLOBAL_FORCE_UP`"]
pub struct GLOBAL_FORCE_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOBAL_FORCE_UP_W<'a> {
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
#[doc = "Reader of field `GLOBAL_UP_EN`"]
pub type GLOBAL_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GLOBAL_UP_EN`"]
pub struct GLOBAL_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLOBAL_UP_EN_W<'a> {
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
    #[doc = "Bit 7 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    pub fn op2_force_up(&self) -> OP2_FORCE_UP_R {
        OP2_FORCE_UP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    pub fn op2_up_en(&self) -> OP2_UP_EN_R {
        OP2_UP_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    pub fn op1_force_up(&self) -> OP1_FORCE_UP_R {
        OP1_FORCE_UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    pub fn op1_up_en(&self) -> OP1_UP_EN_R {
        OP1_UP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    pub fn op0_force_up(&self) -> OP0_FORCE_UP_R {
        OP0_FORCE_UP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    pub fn op0_up_en(&self) -> OP0_UP_EN_R {
        OP0_UP_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - A toggle (software negation of value of this bit) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_force_up(&self) -> GLOBAL_FORCE_UP_R {
        GLOBAL_FORCE_UP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_up_en(&self) -> GLOBAL_UP_EN_R {
        GLOBAL_UP_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    pub fn op2_force_up(&mut self) -> OP2_FORCE_UP_W {
        OP2_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    pub fn op2_up_en(&mut self) -> OP2_UP_EN_W {
        OP2_UP_EN_W { w: self }
    }
    #[doc = "Bit 5 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    pub fn op1_force_up(&mut self) -> OP1_FORCE_UP_W {
        OP1_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    pub fn op1_up_en(&mut self) -> OP1_UP_EN_W {
        OP1_UP_EN_W { w: self }
    }
    #[doc = "Bit 3 - A toggle (software negation of value of this bit) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    pub fn op0_force_up(&mut self) -> OP0_FORCE_UP_W {
        OP0_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    pub fn op0_up_en(&mut self) -> OP0_UP_EN_W {
        OP0_UP_EN_W { w: self }
    }
    #[doc = "Bit 1 - A toggle (software negation of value of this bit) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_force_up(&mut self) -> GLOBAL_FORCE_UP_W {
        GLOBAL_FORCE_UP_W { w: self }
    }
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_up_en(&mut self) -> GLOBAL_UP_EN_W {
        GLOBAL_UP_EN_W { w: self }
    }
}
