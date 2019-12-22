#[doc = "Reader of register MCMCPWM_INT_RAW_MCPWM_REG"]
pub type R = crate::R<u32, super::MCMCPWM_INT_RAW_MCPWM_REG>;
#[doc = "Writer for register MCMCPWM_INT_RAW_MCPWM_REG"]
pub type W = crate::W<u32, super::MCMCPWM_INT_RAW_MCPWM_REG>;
#[doc = "Register MCMCPWM_INT_RAW_MCPWM_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMCPWM_INT_RAW_MCPWM_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_CAP2_INT_RAW`"]
pub type MCPWM_CAP2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CAP2_INT_RAW`"]
pub struct MCPWM_CAP2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP2_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CAP1_INT_RAW`"]
pub type MCPWM_CAP1_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CAP1_INT_RAW`"]
pub struct MCPWM_CAP1_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP1_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CAP0_INT_RAW`"]
pub type MCPWM_CAP0_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CAP0_INT_RAW`"]
pub struct MCPWM_CAP0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP0_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_OST_INT_RAW`"]
pub type MCPWM_FH2_OST_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_OST_INT_RAW`"]
pub struct MCPWM_FH2_OST_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_OST_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH1_OST_INT_RAW`"]
pub type MCPWM_FH1_OST_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH1_OST_INT_RAW`"]
pub struct MCPWM_FH1_OST_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH1_OST_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH0_OST_INT_RAW`"]
pub type MCPWM_FH0_OST_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH0_OST_INT_RAW`"]
pub struct MCPWM_FH0_OST_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH0_OST_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH2_CBC_INT_RAW`"]
pub type MCPWM_FH2_CBC_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH2_CBC_INT_RAW`"]
pub struct MCPWM_FH2_CBC_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH2_CBC_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH1_CBC_INT_RAW`"]
pub type MCPWM_FH1_CBC_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH1_CBC_INT_RAW`"]
pub struct MCPWM_FH1_CBC_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH1_CBC_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FH0_CBC_INT_RAW`"]
pub type MCPWM_FH0_CBC_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FH0_CBC_INT_RAW`"]
pub struct MCPWM_FH0_CBC_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FH0_CBC_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_OP2_TEB_INT_RAW`"]
pub type MCPWM_OP2_TEB_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP2_TEB_INT_RAW`"]
pub struct MCPWM_OP2_TEB_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP2_TEB_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_OP1_TEB_INT_RAW`"]
pub type MCPWM_OP1_TEB_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP1_TEB_INT_RAW`"]
pub struct MCPWM_OP1_TEB_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP1_TEB_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_OP0_TEB_INT_RAW`"]
pub type MCPWM_OP0_TEB_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP0_TEB_INT_RAW`"]
pub struct MCPWM_OP0_TEB_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP0_TEB_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_OP2_TEA_INT_RAW`"]
pub type MCPWM_OP2_TEA_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP2_TEA_INT_RAW`"]
pub struct MCPWM_OP2_TEA_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP2_TEA_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_OP1_TEA_INT_RAW`"]
pub type MCPWM_OP1_TEA_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP1_TEA_INT_RAW`"]
pub struct MCPWM_OP1_TEA_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP1_TEA_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_OP0_TEA_INT_RAW`"]
pub type MCPWM_OP0_TEA_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_OP0_TEA_INT_RAW`"]
pub struct MCPWM_OP0_TEA_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_OP0_TEA_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FAULT2_CLR_INT_RAW`"]
pub type MCPWM_FAULT2_CLR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FAULT2_CLR_INT_RAW`"]
pub struct MCPWM_FAULT2_CLR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FAULT2_CLR_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_FAULT1_CLR_INT_RAW`"]
pub type MCPWM_FAULT1_CLR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FAULT1_CLR_INT_RAW`"]
pub struct MCPWM_FAULT1_CLR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FAULT1_CLR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_FAULT0_CLR_INT_RAW`"]
pub type MCPWM_FAULT0_CLR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FAULT0_CLR_INT_RAW`"]
pub struct MCPWM_FAULT0_CLR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FAULT0_CLR_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_FAULT2_INT_RAW`"]
pub type MCPWM_FAULT2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FAULT2_INT_RAW`"]
pub struct MCPWM_FAULT2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FAULT2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_FAULT1_INT_RAW`"]
pub type MCPWM_FAULT1_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FAULT1_INT_RAW`"]
pub struct MCPWM_FAULT1_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FAULT1_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_FAULT0_INT_RAW`"]
pub type MCPWM_FAULT0_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_FAULT0_INT_RAW`"]
pub struct MCPWM_FAULT0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_FAULT0_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER2_TEP_INT_RAW`"]
pub type MCPWM_TIMER2_TEP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER2_TEP_INT_RAW`"]
pub struct MCPWM_TIMER2_TEP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER2_TEP_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER1_TEP_INT_RAW`"]
pub type MCPWM_TIMER1_TEP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER1_TEP_INT_RAW`"]
pub struct MCPWM_TIMER1_TEP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER1_TEP_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER0_TEP_INT_RAW`"]
pub type MCPWM_TIMER0_TEP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER0_TEP_INT_RAW`"]
pub struct MCPWM_TIMER0_TEP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_TEP_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER2_TEZ_INT_RAW`"]
pub type MCPWM_TIMER2_TEZ_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER2_TEZ_INT_RAW`"]
pub struct MCPWM_TIMER2_TEZ_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER2_TEZ_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER1_TEZ_INT_RAW`"]
pub type MCPWM_TIMER1_TEZ_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER1_TEZ_INT_RAW`"]
pub struct MCPWM_TIMER1_TEZ_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER1_TEZ_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER0_TEZ_INT_RAW`"]
pub type MCPWM_TIMER0_TEZ_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER0_TEZ_INT_RAW`"]
pub struct MCPWM_TIMER0_TEZ_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_TEZ_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER2_STOP_INT_RAW`"]
pub type MCPWM_TIMER2_STOP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER2_STOP_INT_RAW`"]
pub struct MCPWM_TIMER2_STOP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER2_STOP_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER1_STOP_INT_RAW`"]
pub type MCPWM_TIMER1_STOP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER1_STOP_INT_RAW`"]
pub struct MCPWM_TIMER1_STOP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER1_STOP_INT_RAW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER0_STOP_INT_RAW`"]
pub type MCPWM_TIMER0_STOP_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER0_STOP_INT_RAW`"]
pub struct MCPWM_TIMER0_STOP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER0_STOP_INT_RAW_W<'a> {
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
    #[doc = "Bit 29 - The raw status bit for interrupt triggered by captureon channel 2"]
    #[inline(always)]
    pub fn mcpwm_cap2_int_raw(&self) -> MCPWM_CAP2_INT_RAW_R {
        MCPWM_CAP2_INT_RAW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The raw status bit for interrupt triggered by captureon channel 1"]
    #[inline(always)]
    pub fn mcpwm_cap1_int_raw(&self) -> MCPWM_CAP1_INT_RAW_R {
        MCPWM_CAP1_INT_RAW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - The raw status bit for interrupt triggered by captureon channel 0"]
    #[inline(always)]
    pub fn mcpwm_cap0_int_raw(&self) -> MCPWM_CAP0_INT_RAW_R {
        MCPWM_CAP0_INT_RAW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - The raw status bit for interrupt triggered by an one-shot mode action on PWM2"]
    #[inline(always)]
    pub fn mcpwm_fh2_ost_int_raw(&self) -> MCPWM_FH2_OST_INT_RAW_R {
        MCPWM_FH2_OST_INT_RAW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - The raw status bit for interrupt triggered by an one-shot mode action on PWM0"]
    #[inline(always)]
    pub fn mcpwm_fh1_ost_int_raw(&self) -> MCPWM_FH1_OST_INT_RAW_R {
        MCPWM_FH1_OST_INT_RAW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The raw status bit for interrupt triggered by an one-shot mode action on PWM0"]
    #[inline(always)]
    pub fn mcpwm_fh0_ost_int_raw(&self) -> MCPWM_FH0_OST_INT_RAW_R {
        MCPWM_FH0_OST_INT_RAW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The raw status bit for interrupt triggered by an cycle-by-cycle mode action on PWM2"]
    #[inline(always)]
    pub fn mcpwm_fh2_cbc_int_raw(&self) -> MCPWM_FH2_CBC_INT_RAW_R {
        MCPWM_FH2_CBC_INT_RAW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The raw status bit for interrupt triggered by an cycle-by-cycle mode action on PWM1"]
    #[inline(always)]
    pub fn mcpwm_fh1_cbc_int_raw(&self) -> MCPWM_FH1_CBC_INT_RAW_R {
        MCPWM_FH1_CBC_INT_RAW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The raw status bit for interrupt triggered by an cycle-by-cycle mode action on PWM0"]
    #[inline(always)]
    pub fn mcpwm_fh0_cbc_int_raw(&self) -> MCPWM_FH0_CBC_INT_RAW_R {
        MCPWM_FH0_CBC_INT_RAW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The raw status bit for interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn mcpwm_op2_teb_int_raw(&self) -> MCPWM_OP2_TEB_INT_RAW_R {
        MCPWM_OP2_TEB_INT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The raw status bit for interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn mcpwm_op1_teb_int_raw(&self) -> MCPWM_OP1_TEB_INT_RAW_R {
        MCPWM_OP1_TEB_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The raw status bit for interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn mcpwm_op0_teb_int_raw(&self) -> MCPWM_OP0_TEB_INT_RAW_R {
        MCPWM_OP0_TEB_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The raw status bit for interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn mcpwm_op2_tea_int_raw(&self) -> MCPWM_OP2_TEA_INT_RAW_R {
        MCPWM_OP2_TEA_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The raw status bit for interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn mcpwm_op1_tea_int_raw(&self) -> MCPWM_OP1_TEA_INT_RAW_R {
        MCPWM_OP1_TEA_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The raw status bit for interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn mcpwm_op0_tea_int_raw(&self) -> MCPWM_OP0_TEA_INT_RAW_R {
        MCPWM_OP0_TEA_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The raw status bit for interrupt triggered when event_f2 ends"]
    #[inline(always)]
    pub fn mcpwm_fault2_clr_int_raw(&self) -> MCPWM_FAULT2_CLR_INT_RAW_R {
        MCPWM_FAULT2_CLR_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The raw status bit for interrupt triggered when event_f1 ends"]
    #[inline(always)]
    pub fn mcpwm_fault1_clr_int_raw(&self) -> MCPWM_FAULT1_CLR_INT_RAW_R {
        MCPWM_FAULT1_CLR_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The raw status bit for interrupt triggered when event_f0 ends"]
    #[inline(always)]
    pub fn mcpwm_fault0_clr_int_raw(&self) -> MCPWM_FAULT0_CLR_INT_RAW_R {
        MCPWM_FAULT0_CLR_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The raw status bit for interrupt triggered when event_f2 starts"]
    #[inline(always)]
    pub fn mcpwm_fault2_int_raw(&self) -> MCPWM_FAULT2_INT_RAW_R {
        MCPWM_FAULT2_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The raw status bit for interrupt triggered when event_f1 starts"]
    #[inline(always)]
    pub fn mcpwm_fault1_int_raw(&self) -> MCPWM_FAULT1_INT_RAW_R {
        MCPWM_FAULT1_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The raw status bit for interrupt triggered when event_f0 starts"]
    #[inline(always)]
    pub fn mcpwm_fault0_int_raw(&self) -> MCPWM_FAULT0_INT_RAW_R {
        MCPWM_FAULT0_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The raw status bit for interrupt triggered by a PWM timer 2 TEP event"]
    #[inline(always)]
    pub fn mcpwm_timer2_tep_int_raw(&self) -> MCPWM_TIMER2_TEP_INT_RAW_R {
        MCPWM_TIMER2_TEP_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The raw status bit for interrupt triggered by a PWM timer 1 TEP event"]
    #[inline(always)]
    pub fn mcpwm_timer1_tep_int_raw(&self) -> MCPWM_TIMER1_TEP_INT_RAW_R {
        MCPWM_TIMER1_TEP_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The raw status bit for interrupt triggered by a PWM timer 0 TEP event"]
    #[inline(always)]
    pub fn mcpwm_timer0_tep_int_raw(&self) -> MCPWM_TIMER0_TEP_INT_RAW_R {
        MCPWM_TIMER0_TEP_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The raw status bit for interrupt triggered by a PWM timer 2 TEZ event"]
    #[inline(always)]
    pub fn mcpwm_timer2_tez_int_raw(&self) -> MCPWM_TIMER2_TEZ_INT_RAW_R {
        MCPWM_TIMER2_TEZ_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The raw status bit for interrupt triggered by a PWM timer 1 TEZ event"]
    #[inline(always)]
    pub fn mcpwm_timer1_tez_int_raw(&self) -> MCPWM_TIMER1_TEZ_INT_RAW_R {
        MCPWM_TIMER1_TEZ_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The raw status bit for interrupt triggered by a PWM timer 0 TEZ event"]
    #[inline(always)]
    pub fn mcpwm_timer0_tez_int_raw(&self) -> MCPWM_TIMER0_TEZ_INT_RAW_R {
        MCPWM_TIMER0_TEZ_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The raw status bit for interrupt triggered when timer 2 stops"]
    #[inline(always)]
    pub fn mcpwm_timer2_stop_int_raw(&self) -> MCPWM_TIMER2_STOP_INT_RAW_R {
        MCPWM_TIMER2_STOP_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The raw status bit for interrupt triggered when timer 1 stops"]
    #[inline(always)]
    pub fn mcpwm_timer1_stop_int_raw(&self) -> MCPWM_TIMER1_STOP_INT_RAW_R {
        MCPWM_TIMER1_STOP_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The raw status bit for interrupt triggered when timer 0 stops"]
    #[inline(always)]
    pub fn mcpwm_timer0_stop_int_raw(&self) -> MCPWM_TIMER0_STOP_INT_RAW_R {
        MCPWM_TIMER0_STOP_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - The raw status bit for interrupt triggered by captureon channel 2"]
    #[inline(always)]
    pub fn mcpwm_cap2_int_raw(&mut self) -> MCPWM_CAP2_INT_RAW_W {
        MCPWM_CAP2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 28 - The raw status bit for interrupt triggered by captureon channel 1"]
    #[inline(always)]
    pub fn mcpwm_cap1_int_raw(&mut self) -> MCPWM_CAP1_INT_RAW_W {
        MCPWM_CAP1_INT_RAW_W { w: self }
    }
    #[doc = "Bit 27 - The raw status bit for interrupt triggered by captureon channel 0"]
    #[inline(always)]
    pub fn mcpwm_cap0_int_raw(&mut self) -> MCPWM_CAP0_INT_RAW_W {
        MCPWM_CAP0_INT_RAW_W { w: self }
    }
    #[doc = "Bit 26 - The raw status bit for interrupt triggered by an one-shot mode action on PWM2"]
    #[inline(always)]
    pub fn mcpwm_fh2_ost_int_raw(&mut self) -> MCPWM_FH2_OST_INT_RAW_W {
        MCPWM_FH2_OST_INT_RAW_W { w: self }
    }
    #[doc = "Bit 25 - The raw status bit for interrupt triggered by an one-shot mode action on PWM0"]
    #[inline(always)]
    pub fn mcpwm_fh1_ost_int_raw(&mut self) -> MCPWM_FH1_OST_INT_RAW_W {
        MCPWM_FH1_OST_INT_RAW_W { w: self }
    }
    #[doc = "Bit 24 - The raw status bit for interrupt triggered by an one-shot mode action on PWM0"]
    #[inline(always)]
    pub fn mcpwm_fh0_ost_int_raw(&mut self) -> MCPWM_FH0_OST_INT_RAW_W {
        MCPWM_FH0_OST_INT_RAW_W { w: self }
    }
    #[doc = "Bit 23 - The raw status bit for interrupt triggered by an cycle-by-cycle mode action on PWM2"]
    #[inline(always)]
    pub fn mcpwm_fh2_cbc_int_raw(&mut self) -> MCPWM_FH2_CBC_INT_RAW_W {
        MCPWM_FH2_CBC_INT_RAW_W { w: self }
    }
    #[doc = "Bit 22 - The raw status bit for interrupt triggered by an cycle-by-cycle mode action on PWM1"]
    #[inline(always)]
    pub fn mcpwm_fh1_cbc_int_raw(&mut self) -> MCPWM_FH1_CBC_INT_RAW_W {
        MCPWM_FH1_CBC_INT_RAW_W { w: self }
    }
    #[doc = "Bit 21 - The raw status bit for interrupt triggered by an cycle-by-cycle mode action on PWM0"]
    #[inline(always)]
    pub fn mcpwm_fh0_cbc_int_raw(&mut self) -> MCPWM_FH0_CBC_INT_RAW_W {
        MCPWM_FH0_CBC_INT_RAW_W { w: self }
    }
    #[doc = "Bit 20 - The raw status bit for interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn mcpwm_op2_teb_int_raw(&mut self) -> MCPWM_OP2_TEB_INT_RAW_W {
        MCPWM_OP2_TEB_INT_RAW_W { w: self }
    }
    #[doc = "Bit 19 - The raw status bit for interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn mcpwm_op1_teb_int_raw(&mut self) -> MCPWM_OP1_TEB_INT_RAW_W {
        MCPWM_OP1_TEB_INT_RAW_W { w: self }
    }
    #[doc = "Bit 18 - The raw status bit for interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn mcpwm_op0_teb_int_raw(&mut self) -> MCPWM_OP0_TEB_INT_RAW_W {
        MCPWM_OP0_TEB_INT_RAW_W { w: self }
    }
    #[doc = "Bit 17 - The raw status bit for interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn mcpwm_op2_tea_int_raw(&mut self) -> MCPWM_OP2_TEA_INT_RAW_W {
        MCPWM_OP2_TEA_INT_RAW_W { w: self }
    }
    #[doc = "Bit 16 - The raw status bit for interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn mcpwm_op1_tea_int_raw(&mut self) -> MCPWM_OP1_TEA_INT_RAW_W {
        MCPWM_OP1_TEA_INT_RAW_W { w: self }
    }
    #[doc = "Bit 15 - The raw status bit for interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn mcpwm_op0_tea_int_raw(&mut self) -> MCPWM_OP0_TEA_INT_RAW_W {
        MCPWM_OP0_TEA_INT_RAW_W { w: self }
    }
    #[doc = "Bit 14 - The raw status bit for interrupt triggered when event_f2 ends"]
    #[inline(always)]
    pub fn mcpwm_fault2_clr_int_raw(&mut self) -> MCPWM_FAULT2_CLR_INT_RAW_W {
        MCPWM_FAULT2_CLR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 13 - The raw status bit for interrupt triggered when event_f1 ends"]
    #[inline(always)]
    pub fn mcpwm_fault1_clr_int_raw(&mut self) -> MCPWM_FAULT1_CLR_INT_RAW_W {
        MCPWM_FAULT1_CLR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 12 - The raw status bit for interrupt triggered when event_f0 ends"]
    #[inline(always)]
    pub fn mcpwm_fault0_clr_int_raw(&mut self) -> MCPWM_FAULT0_CLR_INT_RAW_W {
        MCPWM_FAULT0_CLR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11 - The raw status bit for interrupt triggered when event_f2 starts"]
    #[inline(always)]
    pub fn mcpwm_fault2_int_raw(&mut self) -> MCPWM_FAULT2_INT_RAW_W {
        MCPWM_FAULT2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10 - The raw status bit for interrupt triggered when event_f1 starts"]
    #[inline(always)]
    pub fn mcpwm_fault1_int_raw(&mut self) -> MCPWM_FAULT1_INT_RAW_W {
        MCPWM_FAULT1_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9 - The raw status bit for interrupt triggered when event_f0 starts"]
    #[inline(always)]
    pub fn mcpwm_fault0_int_raw(&mut self) -> MCPWM_FAULT0_INT_RAW_W {
        MCPWM_FAULT0_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8 - The raw status bit for interrupt triggered by a PWM timer 2 TEP event"]
    #[inline(always)]
    pub fn mcpwm_timer2_tep_int_raw(&mut self) -> MCPWM_TIMER2_TEP_INT_RAW_W {
        MCPWM_TIMER2_TEP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7 - The raw status bit for interrupt triggered by a PWM timer 1 TEP event"]
    #[inline(always)]
    pub fn mcpwm_timer1_tep_int_raw(&mut self) -> MCPWM_TIMER1_TEP_INT_RAW_W {
        MCPWM_TIMER1_TEP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6 - The raw status bit for interrupt triggered by a PWM timer 0 TEP event"]
    #[inline(always)]
    pub fn mcpwm_timer0_tep_int_raw(&mut self) -> MCPWM_TIMER0_TEP_INT_RAW_W {
        MCPWM_TIMER0_TEP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5 - The raw status bit for interrupt triggered by a PWM timer 2 TEZ event"]
    #[inline(always)]
    pub fn mcpwm_timer2_tez_int_raw(&mut self) -> MCPWM_TIMER2_TEZ_INT_RAW_W {
        MCPWM_TIMER2_TEZ_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4 - The raw status bit for interrupt triggered by a PWM timer 1 TEZ event"]
    #[inline(always)]
    pub fn mcpwm_timer1_tez_int_raw(&mut self) -> MCPWM_TIMER1_TEZ_INT_RAW_W {
        MCPWM_TIMER1_TEZ_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3 - The raw status bit for interrupt triggered by a PWM timer 0 TEZ event"]
    #[inline(always)]
    pub fn mcpwm_timer0_tez_int_raw(&mut self) -> MCPWM_TIMER0_TEZ_INT_RAW_W {
        MCPWM_TIMER0_TEZ_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2 - The raw status bit for interrupt triggered when timer 2 stops"]
    #[inline(always)]
    pub fn mcpwm_timer2_stop_int_raw(&mut self) -> MCPWM_TIMER2_STOP_INT_RAW_W {
        MCPWM_TIMER2_STOP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1 - The raw status bit for interrupt triggered when timer 1 stops"]
    #[inline(always)]
    pub fn mcpwm_timer1_stop_int_raw(&mut self) -> MCPWM_TIMER1_STOP_INT_RAW_W {
        MCPWM_TIMER1_STOP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0 - The raw status bit for interrupt triggered when timer 0 stops"]
    #[inline(always)]
    pub fn mcpwm_timer0_stop_int_raw(&mut self) -> MCPWM_TIMER0_STOP_INT_RAW_W {
        MCPWM_TIMER0_STOP_INT_RAW_W { w: self }
    }
}
